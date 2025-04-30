use rig::{completion::CompletionError, loaders::file::FileLoaderError, tool::ToolError};
use thiserror::Error;

use crate::chat::SessionMessage;

/// 应用程序错误类型
///
/// 定义应用程序中可能出现的所有错误类型
///
/// # 示例
/// ```
/// use fsy_ai_chat::errors::AppError;
/// use std::io;
///
/// fn example() -> Result<(), AppError> {
///     // 返回一个IO错误
///     Err(AppError::IOError(io::Error::new(
///         io::ErrorKind::NotFound,
///         "文件未找到"
///     )))
/// }
/// ```
#[derive(Debug, Error)]
pub enum AppError {
    /// 模型补全错误
    #[error("CompletionError: {0}")]
    CompletionError(#[from] CompletionError),

    /// 工具调用错误
    #[error("ToolError: {0}")]
    ToolError(#[from] ToolError),

    /// IO错误
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),

    /// JSON序列化/反序列化错误
    #[error("SerdeJsonError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    /// Qdrant向量数据库错误
    #[error("QdrantError: {0}")]
    QdrantError(#[from] qdrant_client::QdrantError),

    /// 向量存储错误
    #[error("VectorStoreError: {0}")]
    VectorStoreError(#[from] rig::vector_store::VectorStoreError),

    /// 嵌入模型错误
    #[error("EmbeddingError: {0}")]
    EmbeddingError(#[from] rig::embeddings::EmbeddingError),

    /// 文本嵌入错误
    #[error("EmbedError: {0}")]
    EmbedError(#[from] rig::embeddings::EmbedError),

    /// 文档加载错误
    #[error("DocumentLoaderError: {0}")]
    DocumentLoaderError(#[from] FileLoaderError),

    /// 消息发送错误
    #[error("SendError: {0}")]
    SendError(#[from] tokio::sync::broadcast::error::SendError<SessionMessage>),

    /// 其他错误
    #[error("Other: {0}")]
    Other(String),
}

/// 应用程序结果类型
///
/// 使用AppError作为错误类型的Result别名
///
/// # 示例
/// ```
/// use fsy_ai_chat::errors::AppResult;
///
/// fn get_value() -> AppResult<i32> {
///     Ok(42)
/// }
///
/// async fn example() -> AppResult<()> {
///     let value = get_value()?;
///     println!("值: {}", value);
///     Ok(())
/// }
/// ```
pub type AppResult<T> = Result<T, AppError>;
