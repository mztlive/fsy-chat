use crate::kernel::Kernel;

/// 存储错误枚举
///
/// 表示在存储操作过程中可能发生的各种错误类型
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// IO错误，如文件读写失败
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// 持久化错误，通常包含自定义错误信息
    #[error("Persistence error: {0}")]
    PersistenceError(String),

    /// 序列化/反序列化错误
    #[error("Serialize error: {0}")]
    SerializeError(#[from] serde_json::Error),

    /// 其他类型的错误
    #[error("{0}")]
    Other(String),
}

/// 存储接口特质
///
/// 定义了持久化存储的基本操作，可被不同的存储实现实现（如文件存储、数据库存储等）
pub trait Storage {
    /// 将内核中的聊天会话持久化到存储介质
    ///
    /// # 参数
    /// * `kernel` - 系统内核实例，包含所有需要持久化的会话
    ///
    /// # 返回
    /// * `Result<(), StorageError>` - 成功则返回Ok，失败则返回错误
    async fn persistence(&self, kernel: &Kernel) -> Result<(), StorageError>;

    /// 从存储介质加载所有聊天会话到内核中
    ///
    /// # 参数
    /// * `kernel` - 系统内核实例，将加载的会话保存到该内核中
    ///
    /// # 返回
    /// * `Result<(), StorageError>` - 成功则返回Ok，失败则返回错误
    async fn load(&self, kernel: &Kernel) -> Result<(), StorageError>;
}
