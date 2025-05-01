use std::collections::HashMap;
use std::sync::Arc;

use crate::document_loader::DocumentManager;
/// 向量存储模块，提供文档嵌入和向量检索功能
use crate::errors::AppResult;
use crate::models::Document;
use rig::OneOrMany;
use rig::embeddings::{Embedding, EmbeddingModel, EmbeddingsBuilder};
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use tokio::sync::RwLock;
use tracing::{error, info};

/// 向量存储管理器
///
/// 管理多个文档类别的向量存储，提供文档嵌入和语义检索功能。
/// 将文本文档转换为向量表示，并支持相似性搜索，是实现RAG(检索增强生成)的关键组件。
#[derive(Clone)]
pub struct VectorStoreManager {
    /// 按类别存储的向量数据库集合
    stores: Arc<RwLock<HashMap<String, InMemoryVectorStore<Document>>>>,
}

impl VectorStoreManager {
    /// 创建一个新的向量存储管理器
    ///
    /// # 返回值
    /// 返回初始化好的向量存储管理器实例
    pub fn new() -> Self {
        Self {
            stores: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 从文档管理器创建向量存储
    ///
    /// 加载文档管理器中的所有文档，转换为向量表示，并按类别创建向量存储
    ///
    /// # 参数
    /// * `doc_manager` - 文档管理器，包含要转换的文档
    /// * `model` - 嵌入模型，用于生成文档的向量表示
    ///
    /// # 返回值
    /// 成功则返回向量存储管理器实例，否则返回错误
    pub async fn from_documents<M: EmbeddingModel>(
        doc_manager: &DocumentManager,
        model: M,
    ) -> AppResult<Self> {
        let manager = Self::new();
        let grouped_docs = doc_manager.grouped_documents().await;
        for (category, docs) in grouped_docs {
            let documents = build_documents(docs, model.clone()).await?;
            let store = InMemoryVectorStore::from_documents(documents);
            manager.stores.write().await.insert(category, store);
        }

        Ok(manager)
    }

    /// 查找指定类别的向量存储
    ///
    /// # 参数
    /// * `category` - 要查找的文档类别名称
    ///
    /// # 返回值
    /// 如果找到则返回对应的向量存储，否则返回None
    pub async fn find_store(&self, category: &str) -> Option<InMemoryVectorStore<Document>> {
        self.stores.read().await.get(category).cloned()
    }
}

/// 构建文档向量
///
/// 将文本文档转换为向量表示，用于向量存储和检索
///
/// # 参数
/// * `docs` - 要转换的文档列表
/// * `model` - 用于生成文档向量的嵌入模型
///
/// # 返回值
/// 成功则返回文档和向量对的列表，否则返回错误
async fn build_documents(
    docs: Vec<String>,
    model: impl EmbeddingModel,
) -> AppResult<Vec<(Document, OneOrMany<Embedding>)>> {
    let mut documents: Vec<(Document, OneOrMany<Embedding>)> = Vec::new();

    // 将文档分成25个一组的块进行处理，避免单次请求过大
    for chunk in docs.chunks(25) {
        // 创建嵌入构建器并添加文档
        let mut builder = EmbeddingsBuilder::new(model.clone());

        for (i, doc) in chunk.iter().enumerate() {
            let doc_str = doc.to_string();
            info!("{}", doc_str);

            builder = builder.document(Document {
                id: format!("doc_{}", i),
                message: doc_str,
            })?;
        }

        match builder.build().await {
            Ok(embeddings) => documents.extend(embeddings),
            Err(e) => {
                error!("Embedding 文档失败: {}", e);
            }
        }
    }

    Ok(documents)
}
