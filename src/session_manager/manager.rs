use rig::{agent::Agent, streaming::StreamingCompletionModel};

use crate::chat::ChatSession;

use super::{Sessions, UserID, errors::SessionManagerError};

// 聊天会话管理器
#[derive(Clone)]
pub struct ChatSessionManager<M: StreamingCompletionModel> {
    sessions: Sessions<M>,
}

impl<M: StreamingCompletionModel> ChatSessionManager<M> {
    pub fn new() -> Self {
        Self {
            sessions: Sessions::new(),
        }
    }
    // 获取或创建会话
    pub async fn create_session(
        &self,
        user_id: UserID,
        agent: Agent<M>,
        preamble: String,
        doc_category: Option<String>,
    ) -> std::result::Result<(ChatSession<M>, String), SessionManagerError> {
        let session_id = uuid::Uuid::new_v4().to_string();

        // 创建新会话
        let session = ChatSession::new(agent, preamble, doc_category).await?;

        self.sessions
            .add_session(user_id, session_id.clone(), session.clone())
            .await;

        Ok((session, session_id))
    }

    pub async fn get_session(&self, session_id: &str) -> Option<ChatSession<M>> {
        self.sessions.get_session(session_id).await
    }

    pub fn sessions(&self) -> &Sessions<M> {
        &self.sessions
    }

    pub async fn remove_session(
        &self,
        user_id: &UserID,
        session_id: &str,
    ) -> Result<(), SessionManagerError> {
        self.sessions.remove_session(user_id, session_id).await;
        Ok(())
    }
}
