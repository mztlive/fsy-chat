use super::storages::storage::StorageError;

#[derive(Debug, thiserror::Error)]
pub enum SessionManagerError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("App error: {0}")]
    AppError(#[from] crate::errors::AppError),

    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),
}
