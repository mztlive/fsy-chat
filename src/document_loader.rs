use rig::loaders::FileLoader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::CategoryConfig;
use crate::errors::AppResult;

/// 文档结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonDocument {
    /// 文档ID
    pub id: String,
    /// 部门
    pub department: String,
    /// 类别
    pub category: String,
    /// 问题
    pub question: String,
    /// 问题变体
    pub question_variants: Vec<String>,
    /// 答案
    pub answer: String,
}

/// 文档管理器结构体，支持加载和按类目访问文档
#[derive(Clone)]
pub struct DocumentManager {
    /// 按类目存储的文档集合
    documents: Arc<Mutex<HashMap<String, Vec<String>>>>,
    /// 类目配置
    category_configs: Arc<Mutex<HashMap<String, CategoryConfig>>>,
}

impl DocumentManager {
    /// 创建一个新的文档管理器实例
    pub fn new() -> Self {
        Self {
            documents: Arc::new(Mutex::new(HashMap::new())),
            category_configs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 加载指定类目的文档
    ///
    /// * `category_config` - 类目配置
    /// * `directory` - 包含文档的目录路径
    pub async fn load_category<P: AsRef<Path>>(
        &mut self,
        category_config: CategoryConfig,
        directory: P,
    ) -> AppResult<()> {
        let category = category_config.name.clone();
        let glob_pattern = format!("{}/*.json", directory.as_ref().display());

        // 存储类目配置
        self.category_configs
            .lock()
            .await
            .insert(category.clone(), category_config);

        let chunks = FileLoader::with_glob(&glob_pattern)?
            .read()
            .into_iter()
            .filter_map(|result| {
                result
                    .map_err(|e| {
                        eprintln!("Error reading document: {}", e);
                        e
                    })
                    .ok()
            })
            .flat_map(|content| {
                // let chunks = content
                //     .split("\n")
                //     .map(|chunk| chunk.to_string())
                //     .collect::<Vec<String>>();

                let chunks = serde_json::from_str::<Vec<JsonDocument>>(&content).unwrap();
                chunks
                    .into_iter()
                    .map(|chunk| serde_json::to_string(&chunk).unwrap())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<_>>();

        for chunk in chunks {
            self.add_document(category.clone(), chunk).await;
        }

        Ok(())
    }

    /// 添加文档到指定类目
    pub async fn add_document(&mut self, category: String, content: String) {
        self.documents
            .lock()
            .await
            .entry(category)
            .or_insert_with(Vec::new)
            .push(content);
    }

    /// 获取指定类目的所有文档
    #[allow(dead_code)]
    pub async fn get_documents(&self, category: &str) -> Option<Vec<String>> {
        self.documents.lock().await.get(category).map(|v| v.clone())
    }

    /// 获取所有类目
    pub async fn get_categories(&self) -> Vec<String> {
        self.documents.lock().await.keys().cloned().collect()
    }

    /// 获取所有文档
    pub async fn get_all_documents(&self) -> Vec<String> {
        self.documents
            .lock()
            .await
            .values()
            .flatten()
            .cloned()
            .collect()
    }

    /// 获取类目配置
    pub async fn get_category_config(&self, category: &str) -> Option<CategoryConfig> {
        self.category_configs.lock().await.get(category).cloned()
    }
}
