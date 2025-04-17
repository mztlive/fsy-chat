use std::sync::Arc;

use crate::{
    config::Config, document_loader::DocumentManager, web::session_manager::ChatSessionManager,
};

#[derive(Clone)]
pub struct AppState {
    pub chat_session_manager: ChatSessionManager,
    pub config: Arc<Config>,
    pub doc_manager: DocumentManager,
}

impl AppState {
    pub fn new(config: Config, doc_manager: DocumentManager) -> Self {
        Self {
            chat_session_manager: ChatSessionManager::new(),
            config: Arc::new(config),
            doc_manager,
        }
    }
}
