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
/// 表示从JSON文件加载的原始文档内容
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
    ///
    /// # 返回值
    /// 返回初始化的文档管理器实例
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::document_loader::DocumentManager;
    ///
    /// fn example() {
    ///     let manager = DocumentManager::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            documents: Arc::new(Mutex::new(HashMap::new())),
            category_configs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 加载指定类目的文档
    ///
    /// # 参数
    /// * `category_config` - 类目配置
    /// * `directory` - 包含文档的目录路径
    ///
    /// # 返回值
    /// 如果加载成功则返回Ok，否则返回错误
    ///
    /// # 示例
    /// ```
    /// use std::path::Path;
    /// use fsy_ai_chat::document_loader::DocumentManager;
    /// use fsy_ai_chat::config::CategoryConfig;
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut manager = DocumentManager::new();
    ///     
    ///     let category_config = CategoryConfig {
    ///         name: "faq".to_string(),
    ///         directory: Path::new("./data/faq").to_path_buf(),
    ///         collection_name: "faq_collection".to_string(),
    ///     };
    ///     
    ///     manager.load_category(category_config, "./data/faq").await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
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
    ///
    /// # 参数
    /// * `category` - 文档所属类目
    /// * `content` - 文档内容
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::document_loader::DocumentManager;
    ///
    /// async fn example() {
    ///     let mut manager = DocumentManager::new();
    ///     
    ///     let document = r#"{"id":"1","department":"技术","category":"常见问题",
    ///                         "question":"如何重置密码?",
    ///                         "question_variants":["密码忘记了怎么办","怎样修改密码"],
    ///                         "answer":"您可以在登录页面点击'忘记密码'链接进行重置。"}"#;
    ///     
    ///     manager.add_document("faq".to_string(), document.to_string()).await;
    /// }
    /// ```
    pub async fn add_document(&mut self, category: String, content: String) {
        self.documents
            .lock()
            .await
            .entry(category)
            .or_insert_with(Vec::new)
            .push(content);
    }

    /// 获取指定类目的所有文档
    ///
    /// # 参数
    /// * `category` - 要获取文档的类目名称
    ///
    /// # 返回值
    /// 返回指定类目的所有文档，如果类目不存在则返回None
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::document_loader::DocumentManager;
    ///
    /// async fn example() {
    ///     let manager = DocumentManager::new();
    ///     
    ///     if let Some(documents) = manager.get_documents("faq").await {
    ///         println!("找到{}个文档", documents.len());
    ///     } else {
    ///         println!("类目不存在");
    ///     }
    /// }
    /// ```
    #[allow(dead_code)]
    pub async fn get_documents(&self, category: &str) -> Option<Vec<String>> {
        self.documents.lock().await.get(category).map(|v| v.clone())
    }

    /// 获取所有类目
    ///
    /// # 返回值
    /// 返回系统中所有已加载的类目名称列表
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::document_loader::DocumentManager;
    ///
    /// async fn example() {
    ///     let manager = DocumentManager::new();
    ///     
    ///     let categories = manager.get_categories().await;
    ///     println!("系统中有以下类目: {:?}", categories);
    /// }
    /// ```
    pub async fn get_categories(&self) -> Vec<String> {
        self.documents.lock().await.keys().cloned().collect()
    }

    /// 获取所有文档
    ///
    /// # 返回值
    /// 返回所有已加载的文档，不区分类目
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::document_loader::DocumentManager;
    ///
    /// async fn example() {
    ///     let manager = DocumentManager::new();
    ///     
    ///     let all_docs = manager.get_all_documents().await;
    ///     println!("系统中共有{}个文档", all_docs.len());
    /// }
    /// ```
    pub async fn get_all_documents(&self) -> Vec<String> {
        self.documents
            .lock()
            .await
            .values()
            .flatten()
            .cloned()
            .collect()
    }

    pub async fn grouped_documents(&self) -> HashMap<String, Vec<String>> {
        self.documents.lock().await.clone()
    }
}
