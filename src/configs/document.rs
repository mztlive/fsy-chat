use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DocumentConfig {
    pub categories: Vec<CategoryConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryConfig {
    /// 类目名称
    pub name: String,
    /// 类目对应的文档目录
    pub directory: PathBuf,
}
