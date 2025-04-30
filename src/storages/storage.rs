
use crate::kernel::Kernel;

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
    async fn persistence(&self, kernel: &Kernel) -> Result<(), StorageError>;

    async fn load(&self, kernel: &Kernel) -> Result<(), StorageError>;
}
