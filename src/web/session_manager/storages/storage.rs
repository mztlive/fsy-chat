use crate::web::session_manager::Sessions;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Persistence error: {0}")]
    PersistenceError(String),
}

pub trait Storage {
    async fn persistence(&self, sessions: &Sessions) -> Result<(), StorageError>;

    async fn load(&self) -> Result<(), StorageError>;
}
