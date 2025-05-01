use rig::loaders::FileLoader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::CategoryConfig;
use crate::errors::AppResult;

/// 文档结构体
///
/// 表示从JSON文件加载的结构化文档，包含问答对和相关元数据
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonDocument {
    /// 文档唯一标识符
    pub id: String,
    /// 所属部门
    pub department: String,
    /// 文档类别
    pub category: String,
    /// 主要问题
    pub question: String,
    /// 问题的其他表述形式
    pub question_variants: Vec<String>,
    /// 问题的答案
    pub answer: String,
}

/// 文档管理器
///
/// 负责从文件系统加载文档，并按类别组织和管理文档。
/// 支持多个类别的文档集合，每个类别可以包含多个文档。
#[derive(Clone)]
pub struct DocumentManager {
    /// 按类目存储的文档集合
    documents: Arc<Mutex<HashMap<String, Vec<String>>>>,
    /// 类目配置
    category_configs: Arc<Mutex<HashMap<String, CategoryConfig>>>,
}

impl DocumentManager {
    /// 创建一个新的文档管理器
    ///
    /// 初始化一个空的文档管理器，不加载任何文档
    ///
    /// # 返回值
    /// 返回初始化的文档管理器实例
    pub fn new() -> Self {
        Self {
            documents: Arc::new(Mutex::new(HashMap::new())),
            category_configs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 加载指定类别的文档
    ///
    /// 从指定目录加载JSON格式的文档，并按类别存储
    ///
    /// # 参数
    /// * `category_config` - 类别配置，包含类别名称和其他信息
    /// * `directory` - 文档所在的目录路径
    ///
    /// # 返回值
    /// 加载成功返回Ok，否则返回错误
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
                // 解析JSON文档集合
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

    /// 添加文档到指定类别
    ///
    /// # 参数
    /// * `category` - 文档类别名称
    /// * `content` - 文档内容（JSON格式的字符串）
    pub async fn add_document(&mut self, category: String, content: String) {
        self.documents
            .lock()
            .await
            .entry(category)
            .or_insert_with(Vec::new)
            .push(content);
    }

    /// 获取指定类别的所有文档
    ///
    /// # 参数
    /// * `category` - 类别名称
    ///
    /// # 返回值
    /// 如果类别存在，返回该类别的所有文档；否则返回None
    #[allow(dead_code)]
    pub async fn get_documents(&self, category: &str) -> Option<Vec<String>> {
        self.documents.lock().await.get(category).map(|v| v.clone())
    }

    /// 获取所有已加载的类别名称
    ///
    /// # 返回值
    /// 返回所有类别名称的列表
    pub async fn get_categories(&self) -> Vec<String> {
        self.documents.lock().await.keys().cloned().collect()
    }

    /// 获取所有文档
    ///
    /// 返回所有已加载的文档，不区分类别
    ///
    /// # 返回值
    /// 返回所有文档内容的列表
    pub async fn get_all_documents(&self) -> Vec<String> {
        self.documents
            .lock()
            .await
            .values()
            .flatten()
            .cloned()
            .collect()
    }

    /// 获取按类别分组的文档集合
    ///
    /// # 返回值
    /// 返回一个映射，键为类别名称，值为该类别下的所有文档
    pub async fn grouped_documents(&self) -> HashMap<String, Vec<String>> {
        self.documents.lock().await.clone()
    }
}
