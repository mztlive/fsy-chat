use serde::Deserialize;
use std::path::PathBuf;

/// 代理配置
///
/// 包含AI代理的基本配置参数，如API密钥、前置指令和模型名称
#[derive(Debug, Clone, Deserialize)]
pub struct ClientConfig {
    /// OpenAI兼容API的密钥
    pub api_key: String,
    /// 使用的大语言模型名称
    pub chat_model: String,
}

/// 嵌入模型配置
///
/// 包含文本嵌入相关的配置参数，用于向量化文档和语义搜索
#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingConfig {
    pub api_key: String,
    /// 使用的嵌入模型名称
    pub model: String,
    /// 嵌入模型的维度
    pub dimensions: u32,
}

/// 应用程序配置
///
/// 包含整个应用程序所需的所有配置信息，是最顶层的配置结构
///
/// # 示例
/// ```
/// use fsy_ai_chat::config::Config;
/// use std::path::PathBuf;
/// use fsy_ai_chat::agent::{AgentConfig, EmbeddingConfig};
/// use fsy_ai_chat::config::{DocumentConfig, CategoryConfig};
///
/// fn example() -> Config {
///     Config {
///         agent: AgentConfig {
///             api_key: "your-api-key".to_string(),
///             preamble: "你好，我是AI助手".to_string(),
///             chat_model: "qwen-max".to_string(),
///         },
///         embedding: Some(EmbeddingConfig {
///             api_key: "your-api-key".to_string(),
///             model: "text-embedding-v1".to_string(),
///             dimensions: 1536,
///         }),
///         document: DocumentConfig {
///             categories: vec![
///                 CategoryConfig {
///                     name: "faq".to_string(),
///                     directory: PathBuf::from("./data/faq"),
///                     collection_name: "faq_collection".to_string(),
///                 }
///             ],
///         },
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// 代理配置，包含LLM相关参数
    pub client: ClientConfig,
    /// 嵌入模型配置，用于文档向量化
    pub embedding: EmbeddingConfig,
    /// 文档配置，包含各类别文档的加载信息
    pub document: DocumentConfig,
}

/// 文档配置
///
/// 包含文档加载和管理的相关配置信息
///
/// # 示例
/// ```
/// use fsy_ai_chat::config::{DocumentConfig, CategoryConfig};
/// use std::path::PathBuf;
///
/// fn example() -> DocumentConfig {
///     DocumentConfig {
///         categories: vec![
///             CategoryConfig {
///                 name: "faq".to_string(),
///                 directory: PathBuf::from("./data/faq"),
///                 collection_name: "faq_collection".to_string(),
///             },
///             CategoryConfig {
///                 name: "knowledge_base".to_string(),
///                 directory: PathBuf::from("./data/kb"),
///                 collection_name: "kb_collection".to_string(),
///             }
///         ],
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct DocumentConfig {
    /// 文档类别配置列表
    pub categories: Vec<CategoryConfig>,
}

/// 文档类别配置
///
/// 定义单个文档类别的配置参数，如名称和存储路径
///
/// # 示例
/// ```
/// use fsy_ai_chat::config::CategoryConfig;
/// use std::path::PathBuf;
///
/// fn example() -> CategoryConfig {
///     CategoryConfig {
///         name: "faq".to_string(),
///         directory: PathBuf::from("./data/faq"),
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct CategoryConfig {
    /// 类别名称
    pub name: String,
    /// 类别对应的文档目录
    pub directory: PathBuf,
}
