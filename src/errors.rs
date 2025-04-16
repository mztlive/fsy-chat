use rig::{completion::CompletionError, loaders::file::FileLoaderError, tool::ToolError};
use thiserror::Error;

use crate::chat::SessionMessage;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("CompletionError: {0}")]
    CompletionError(#[from] CompletionError),

    #[error("ToolError: {0}")]
    ToolError(#[from] ToolError),

    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),

    #[error("SerdeJsonError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("QdrantError: {0}")]
    QdrantError(#[from] qdrant_client::QdrantError),

    #[error("VectorStoreError: {0}")]
    VectorStoreError(#[from] rig::vector_store::VectorStoreError),

    #[error("EmbeddingError: {0}")]
    EmbeddingError(#[from] rig::embeddings::EmbeddingError),

    #[error("EmbedError: {0}")]
    EmbedError(#[from] rig::embeddings::EmbedError),

    #[error("DocumentLoaderError: {0}")]
    DocumentLoaderError(#[from] FileLoaderError),

    #[error("SendError: {0}")]
    SendError(#[from] tokio::sync::broadcast::error::SendError<SessionMessage>),
}

pub type AppResult<T> = Result<T, AppError>;
