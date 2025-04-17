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
}

impl ChatSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Sessions::new(),
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

    pub fn sessions(&self) -> Sessions {
        self.sessions.clone()
    }
}
