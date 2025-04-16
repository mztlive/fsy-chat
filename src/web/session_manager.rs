use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    agent::{AgentConfig, EmbeddingConfig},
    chat::ChatSession,
    document_loader::DocumentManager,
};

use super::errors::WebError;

// 聊天会话管理器
#[derive(Clone)]
pub struct ChatSessionManager {
    sessions: Arc<Mutex<HashMap<String, ChatSession>>>,
    agent_config: AgentConfig,
    embedding_config: Option<EmbeddingConfig>,
    document_manager: Option<DocumentManager>,
    qdrant_url: Option<String>,
}

impl ChatSessionManager {
    pub fn new(
        agent_config: AgentConfig,
        embedding_config: Option<EmbeddingConfig>,
        document_manager: Option<DocumentManager>,
        qdrant_url: Option<String>,
    ) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            agent_config,
            embedding_config,
            document_manager,
            qdrant_url,
        }
    }

    // 获取或创建会话
    pub async fn get_or_create_session(
        &self,
        session_id: &str,
        category: Option<String>,
    ) -> std::result::Result<ChatSession, WebError> {
        let mut sessions = self.sessions.lock().await;

        if let Some(session) = sessions.get(session_id) {
            return Ok(session.clone());
        }

        // 创建新会话
        let session = ChatSession::new(
            self.agent_config.clone(),
            self.embedding_config.clone(),
            self.document_manager.clone(),
            self.qdrant_url.clone(),
            category,
        )
        .await?;

        sessions.insert(session_id.to_string(), session.clone());

        Ok(session)
    }
}
