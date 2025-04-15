use serde::Deserialize;
use std::path::PathBuf;

use crate::agent::{AgentConfig, EmbeddingConfig};

/// 应用程序配置结构体
///
/// 包含所有模块所需的配置信息
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
#[derive(Debug, Clone, Deserialize)]
pub struct DocumentConfig {
    /// 文档目录映射，键为类目名称，值为目录路径
    pub categories: Vec<CategoryConfig>,
}

/// 文档类目配置
#[derive(Debug, Clone, Deserialize)]
pub struct CategoryConfig {
    /// 类目名称
    pub name: String,
    /// 类目对应的文档目录
    pub directory: PathBuf,
}
