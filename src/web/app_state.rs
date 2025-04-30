use std::sync::Arc;

use rig::providers::openai::Client;

use crate::{
    config::Config, document_loader::DocumentManager, vector_store::VectorStoreManager,
    web::session_manager::ChatSessionManager,
};

#[derive(Clone)]
pub struct AppState {
    pub chat_session_manager: ChatSessionManager,
    pub config: Arc<Config>,
    pub doc_manager: DocumentManager,
    pub vector_store_manager: VectorStoreManager,
    client: Client,
}

impl AppState {
    pub async fn new(config: Config, doc_manager: DocumentManager, client: Client) -> Self {
        let vector_store_manager = VectorStoreManager::from_documents(
            &doc_manager,
            client.embedding_model(&config.embedding.clone().unwrap().model),
        )
        .await
        .expect("向量存储管理器初始化失败");

        Self {
            chat_session_manager: ChatSessionManager::new(),
            config: Arc::new(config),
            doc_manager,
            vector_store_manager,
            client,
        }
    }
}
