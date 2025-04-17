use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    agent::{AgentConfig, EmbeddingConfig},
    chat::ChatSession,
    document_loader::DocumentManager,
};

use super::{Sessions, UserID, errors::SessionManagerError};

// 聊天会话管理器
#[derive(Clone)]
pub struct ChatSessionManager {
    sessions: Sessions,
    last_sync_at: Arc<Mutex<u64>>,
}

impl ChatSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Sessions::new(),
            last_sync_at: Arc::new(Mutex::new(0)),
        }
    }

    // 获取或创建会话
    pub async fn create_session(
        &self,
        user_id: UserID,
        agent_config: AgentConfig,
        category: Option<String>,
        embedding_config: Option<EmbeddingConfig>,
        document_manager: Option<DocumentManager>,
        qdrant_url: Option<String>,
    ) -> std::result::Result<(ChatSession, String), SessionManagerError> {
        let session_id = uuid::Uuid::new_v4().to_string();

        // 创建新会话
        let session = ChatSession::new(
            agent_config,
            embedding_config,
            document_manager,
            qdrant_url,
            category,
        )
        .await?;

        self.sessions
            .add_session(user_id, session_id.clone(), session.clone())
            .await;

        Ok((session, session_id))
    }

    pub async fn get_session(&self, session_id: &str) -> Option<ChatSession> {
        self.sessions.get_session(session_id).await
    }

    // // 从文件加载会话
    // pub async fn load_sessions_from_files(&self, base_path: &str) -> Result<(), WebError> {
    //     let sessions_dir = base_path;

    //     // 检查目录是否存在
    //     if !tokio::fs::metadata(&sessions_dir).await.is_ok() {
    //         return Ok(()); // 目录不存在，返回空结果
    //     }

    //     // 遍历用户目录
    //     let mut user_dirs = tokio::fs::read_dir(&sessions_dir).await?;
    //     while let Some(user_entry) = user_dirs.next_entry().await? {
    //         let user_id = user_entry.file_name().to_string_lossy().to_string();
    //         let user_path = user_entry.path();

    //         // 遍历会话文件
    //         let mut session_files = tokio::fs::read_dir(&user_path).await?;
    //         while let Some(session_entry) = session_files.next_entry().await? {
    //             if !session_entry
    //                 .file_name()
    //                 .to_string_lossy()
    //                 .ends_with(".json")
    //             {
    //                 continue;
    //             }

    //             // 读取文件内容
    //             let file_path = session_entry.path();
    //             let session_json = tokio::fs::read_to_string(&file_path).await?;

    //             // 反序列化会话
    //             let mut session: ChatSession = serde_json::from_str(&session_json)?;

    //             // 确保用户ID
    //             if session.user_id.is_none() {
    //                 session.user_id = Some(user_id.clone());
    //             }

    //             // 提取会话ID
    //             let file_name = session_entry.file_name().to_string_lossy();
    //             let session_id = file_name.trim_end_matches(".json").to_string();

    //             // 添加到内存
    //             let mut sessions = self.sessions.lock().await;
    //             sessions.insert(session_id, session);
    //         }
    //     }

    //     Ok(())
    // }
}
