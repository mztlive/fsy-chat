use std::collections::HashMap;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use rig::streaming::StreamingCompletionModel;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

use crate::chat::ChatSessionView;
use crate::kernel::Kernel;
use crate::{
    chat::ChatSession,
    session_manager::{UserChatSessions, UserID},
};

use super::storage::{Storage, StorageError};

/// 文件存储结构体，用于将聊天会话持久化到文件系统
///
/// 存储结构:
/// - base_path/sessions/         # 根目录下的会话目录
///   - {user_id}/                # 每个用户有一个目录，以用户ID命名
///     - {session_id}.json       # 每个会话保存为一个JSON文件，以会话ID命名
pub struct FileStorage {
    pub base_path: String,
}

impl FileStorage {
    /// 创建一个新的文件存储实例
    ///
    /// # 参数
    /// * `path` - 存储根目录路径
    pub fn new(path: String) -> Self {
        Self { base_path: path }
    }

    /// 清理过期的会话文件
    ///
    /// 此方法比较当前内存中的会话ID和文件系统中的会话文件，
    /// 删除文件系统中存在但内存中不存在的会话文件，实现会话文件的过期清理。
    ///
    /// # 参数
    /// * `user_dir` - 用户目录路径
    /// * `sessions` - 用户的聊天会话集合
    ///
    /// # 返回
    /// * `Result<(), StorageError>` - 成功则返回Ok，失败则返回错误
    ///
    /// # 工作流程
    /// 1. 获取内存中的会话ID集合
    /// 2. 读取用户目录下所有JSON文件名
    /// 3. 删除不在内存中会话集合的文件
    async fn expire_sessions<M: StreamingCompletionModel>(
        &self,
        user_dir: &PathBuf,
        sessions: &UserChatSessions<M>,
    ) -> Result<(), StorageError> {
        let session_ids: HashSet<String> = sessions.session_ids().into_iter().collect();

        // 收集目录中的所有JSON文件ID
        let mut file_session_ids = HashSet::new();
        let mut file_paths = HashMap::new();

        let mut read_dir = fs::read_dir(user_dir).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                    let session_id = file_name.replace(".json", "");
                    file_session_ids.insert(session_id.clone());
                    file_paths.insert(session_id, path);
                }
            }
        }

        // 计算差集：文件中存在但内存中不存在的会话ID
        let to_delete_ids = file_session_ids.difference(&session_ids);

        // 标记过期文件
        for id in to_delete_ids {
            if let Some(path) = file_paths.get(id) {
                let delete_path = path.with_file_name(format!(
                    "{}.delete",
                    path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                ));

                fs::rename(&path, &delete_path).await.unwrap_or_else(|e| {
                    tracing::warn!("标记过期会话文件失败: {:?}, 错误: {}", path, e);
                });
            }
        }

        Ok(())
    }

    /// 保存用户的所有聊天会话到文件
    ///
    /// # 参数
    /// * `sessions` - 用户的聊天会话集合
    /// * `user_dir` - 用户目录路径
    ///
    /// # 返回
    /// * `Result<(), StorageError>` - 成功则返回Ok，失败则返回错误
    async fn save_user_sessions<M: StreamingCompletionModel>(
        &self,
        sessions: &UserChatSessions<M>,
        user_dir: &PathBuf,
    ) -> Result<(), StorageError> {
        self.expire_sessions(user_dir, sessions).await?;

        let sessions = sessions
            .iter()
            .map(|(session_id, session)| (session_id, session))
            .collect::<Vec<(&String, &ChatSession<M>)>>();

        for (session_id, session) in sessions {
            let mut file = File::create(user_dir.join(format!("{}.json", session_id))).await?;

            let session_view = session.to_view().await;
            let session_view_str = serde_json::to_string(&session_view)?;

            file.write_all(session_view_str.as_bytes()).await?;
        }

        Ok(())
    }

    /// 从文件加载单个聊天会话
    ///
    /// # 参数
    /// * `file_path` - 会话文件路径
    ///
    /// # 返回
    /// * `Result<(String, ChatSessionView), StorageError>` - 成功则返回会话ID和会话视图，失败则返回错误
    async fn load_chat_session_from_file(
        &self,
        file_path: PathBuf,
    ) -> Result<(String, ChatSessionView), StorageError> {
        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| StorageError::Other("无效的文件名".to_string()))?;

        let session_id = file_name.to_string().replace(".json", "");

        let json_file = fs::read(&file_path).await?;
        let chat_view = serde_json::from_slice::<ChatSessionView>(&json_file)?;

        Ok((session_id, chat_view))
    }

    /// 加载单个用户的所有聊天会话并添加到内核中
    ///
    /// # 参数
    /// * `user_dir` - 用户目录路径
    /// * `kernel` - 系统内核实例
    ///
    /// # 返回
    /// * `Result<(), StorageError>` - 成功则返回Ok，失败则返回错误
    async fn load_user_sessions(
        &self,
        user_dir: PathBuf,
        kernel: &Kernel,
    ) -> Result<(), StorageError> {
        // 从目录名称解析用户ID
        let user_id = parse_user_id_from_directory(&user_dir)?;

        // 读取目录中的所有文件
        let mut entries = fs::read_dir(&user_dir).await?;
        let mut loaded_count = 0;
        let mut error_count = 0;

        while let Some(entry) = entries.next_entry().await? {
            let file_path = entry.path();

            // 跳过非JSON文件
            if !file_path.is_file()
                || file_path.extension().and_then(|ext| ext.to_str()) != Some("json")
            {
                continue;
            }

            // 加载会话文件
            match self.load_chat_session_from_file(file_path.clone()).await {
                Ok((session_id, chat_view)) => {
                    kernel
                        .recovery_chatview(user_id.clone(), session_id.clone(), chat_view)
                        .await
                        .map(|_| loaded_count += 1)
                        .unwrap_or_else(|_| error_count += 1);
                }
                Err(e) => {
                    tracing::warn!("加载会话文件失败: {:?}, 错误: {}", file_path, e);
                    error_count += 1;
                }
            }
        }

        tracing::info!(
            "用户 {} 会话加载完成, 成功: {}, 失败: {}",
            user_id,
            loaded_count,
            error_count
        );

        Ok(())
    }
}

impl Storage for FileStorage {
    /// 将内核中的所有聊天会话持久化到文件
    ///
    /// 存储结构:
    /// - base_path/sessions/{user_id}/{session_id}.json
    ///
    /// # 参数
    /// * `kernel` - 系统内核实例
    ///
    /// # 返回
    /// * `Result<(), StorageError>` - 成功则返回Ok，失败则返回错误
    async fn persistence(&self, kernel: &Kernel) -> Result<(), StorageError> {
        // 确保目录存在
        let sessions_dir = Path::new(&self.base_path).join("sessions");
        tokio::fs::create_dir_all(&sessions_dir).await?;

        let sessions = kernel.sessions();

        for user_id in sessions.user_ids().await {
            // 创建用户的目录
            let user_dir = sessions_dir.clone().join(&user_id);
            tokio::fs::create_dir_all(&user_dir).await?;

            match sessions.get_sessions(&user_id).await {
                Some(sessions) => {
                    self.save_user_sessions(&sessions, &user_dir).await?;
                }
                None => {
                    continue;
                }
            }
        }

        Ok(())
    }

    /// 从文件加载所有聊天会话到内核中
    ///
    /// 加载过程:
    /// 1. 遍历sessions目录下的所有用户目录
    /// 2. 对每个用户目录，加载其中的所有JSON文件作为聊天会话
    /// 3. 将加载的会话添加到内核中
    ///
    /// # 参数
    /// * `kernel` - 系统内核实例
    ///
    /// # 返回
    /// * `Result<(), StorageError>` - 成功则返回Ok，失败则返回错误
    async fn load(&self, kernel: &Kernel) -> Result<(), StorageError> {
        let sessions_dir = Path::new(&self.base_path).join("sessions");

        // 确保会话目录存在
        if !sessions_dir.exists() {
            return Ok(());
        }

        let mut user_dirs = fs::read_dir(&sessions_dir).await?;
        let sessions = kernel.sessions();

        while let Ok(Some(entry)) = user_dirs.next_entry().await {
            let user_dir = entry.path();

            if !user_dir.is_dir() {
                continue;
            }

            match self.load_user_sessions(user_dir, kernel).await {
                Ok(_) => {}
                Err(e) => {
                    tracing::warn!("加载用户会话失败: {}", e);
                    continue;
                }
            }
        }

        tracing::info!(
            "加载所有用户会话完成, 共加载{}个会话",
            sessions.capacity().await
        );
        Ok(())
    }
}

/// 从用户目录路径中提取用户ID
///
/// # 参数
/// * `user_dir` - 用户目录路径
///
/// # 返回
/// * `Result<UserID, StorageError>` - 成功则返回用户ID，失败则返回错误
fn parse_user_id_from_directory(user_dir: &PathBuf) -> Result<UserID, StorageError> {
    let user_id = user_dir
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| StorageError::Other("无效的用户目录名".to_string()))?;

    Ok(UserID::from(user_id))
}
