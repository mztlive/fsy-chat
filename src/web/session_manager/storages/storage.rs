use crate::{
    agent::{AgentConfig, EmbeddingConfig},
    document_loader::DocumentManager,
    web::session_manager::Sessions,
};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Persistence error: {0}")]
    PersistenceError(String),

    #[error("Serialize error: {0}")]
    SerializeError(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}

pub trait Storage {
    async fn persistence(&self, sessions: &Sessions) -> Result<(), StorageError>;

    async fn load(
        &self,
        sessions: &Sessions,
        config: AgentConfig,
        embedding_config: Option<EmbeddingConfig>,
        document_manager: Option<DocumentManager>,
    ) -> Result<(), StorageError>;
}
