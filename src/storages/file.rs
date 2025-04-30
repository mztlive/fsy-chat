use std::path::{Path, PathBuf};

use rig::agent::Agent;
use rig::streaming::StreamingCompletionModel;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

use crate::chat::ChatSessionView;
use crate::document_loader::DocumentManager;
use crate::kernel::Kernel;
use crate::{
    chat::ChatSession,
    session_manager::{Sessions, UserChatSessions, UserID},
};

use super::storage::{Storage, StorageError};

pub struct FileStorage {
    pub base_path: String,
}

impl FileStorage {
    pub fn new(path: String) -> Self {
        Self { base_path: path }
    }

    async fn save_user_sessions<M: StreamingCompletionModel>(
        &self,
        sessions: &UserChatSessions<M>,
        user_dir: &PathBuf,
    ) -> Result<(), StorageError> {
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

    // 新增：从文件加载单个会话
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

    // 新增：加载单个用户的所有会话
    async fn load_user_sessions(
        &self,
        user_dir: PathBuf,
        kernel: &Kernel,
    ) -> Result<(), StorageError> {
        let user_id = parse_user_id_from_directory(&user_dir)?;

        let mut files = fs::read_dir(&user_dir).await?;

        while let Ok(Some(entry)) = files.next_entry().await {
            let file_path = entry.path();

            if !file_path.is_file()
                || file_path.extension().and_then(|ext| ext.to_str()) != Some("json")
            {
                continue;
            }

            match self.load_chat_session_from_file(file_path).await {
                Ok((session_id, chat_view)) => {
                    kernel
                        .add_history(user_id.clone(), session_id, chat_view)
                        .await
                        .map_err(|e| StorageError::Other(format!("添加历史会话失败: {}", e)))?;
                }
                Err(e) => {
                    tracing::warn!("加载会话文件失败: {}", e);
                    continue;
                }
            }
        }

        // tracing::info!(
        //     "加载用户{}的会话完成, 会话数量: {}",
        //     user_id,
        //     user_sessions.len()
        // );

        Ok(())
    }
}

impl Storage for FileStorage {
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
