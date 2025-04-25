use serde::Deserialize;
use std::path::PathBuf;

use crate::agent::{AgentConfig, EmbeddingConfig};

/// 应用程序配置结构体
///
/// 包含所有模块所需的配置信息
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
    /// 代理配置
    pub agent: AgentConfig,
    /// 嵌入模型配置，可选
    pub embedding: Option<EmbeddingConfig>,
    /// 文档配置
    pub document: DocumentConfig,
}

/// 文档配置结构体
///
/// 包含文档管理器所需的配置信息
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
    /// 文档类目配置
    pub categories: Vec<CategoryConfig>,
}

/// 文档类目配置
///
/// 定义文档类目的名称、目录和向量存储集合
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
    /// 类目名称
    pub name: String,
    /// 类目对应的文档目录
    pub directory: PathBuf,
}
