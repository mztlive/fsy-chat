use rig::loaders::FileLoader;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::errors::AppResult;

/// 文档管理器结构体，支持加载和按类目访问文档
#[derive(Clone)]
pub struct DocumentManager {
    /// 按类目存储的文档集合
    documents: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl DocumentManager {
    /// 创建一个新的文档管理器实例
    pub fn new() -> Self {
        Self {
            documents: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 加载指定类目的文档
    ///
    /// * `category` - 类目名称
    /// * `directory` - 包含文档的目录路径
    pub fn load_category<P: AsRef<Path>>(
        &mut self,
        category: String,
        directory: P,
    ) -> AppResult<()> {
        let glob_pattern = format!("{}/*.csv", directory.as_ref().display());

        FileLoader::with_glob(&glob_pattern)?
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
                let chunks = content
                    .split("\n")
                    .map(|chunk| chunk.to_string())
                    .collect::<Vec<String>>();
                chunks
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|chunk| {
                self.add_document(category.clone(), chunk);
            });

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
}
