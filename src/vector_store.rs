use std::collections::HashMap;
use std::sync::Arc;

use crate::document_loader::DocumentManager;
/// 向量存储模块，提供文档嵌入和向量检索功能
use crate::errors::AppResult;
use crate::models::Document;
use rig::OneOrMany;
use rig::embeddings::{Embedding, EmbeddingModel, EmbeddingsBuilder};
use rig::vector_store::VectorStoreIndex;
use rig::vector_store::in_memory_store::{InMemoryVectorIndex, InMemoryVectorStore};
use tokio::sync::RwLock;
use tracing::{error, info};

/// 初始化向量存储
///
/// 加载文档，创建嵌入，并初始化向量存储。
///
/// # 参数
/// * `model` - 实现了EmbeddingModel特性的嵌入模型
/// * `config` - 向量存储配置
/// * `document_manager` - 文档管理器，提供文档数据
///
/// # 返回值
/// 返回初始化好的向量存储索引，如果初始化过程中发生错误则返回错误
///
/// # 示例
/// ```
/// use fsy_ai_chat::vector_store::{self, VectorStoreConfig};
/// use fsy_ai_chat::document_loader::DocumentManager;
/// use rig::providers::openai::{Client, EmbeddingModel};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
///     // 初始化嵌入模型
///     let client = Client::new("your-api-key");
///     let model = EmbeddingModel::new(client, "text-embedding-3-small", 1536);
///
///     // 配置向量存储
///     let config = VectorStoreConfig::new(
///         "documents".to_string(),
///         1536,
///     );
///
///     // 初始化文档管理器
///     let doc_manager = DocumentManager::new();
///
///     // 初始化向量存储
///     let index = vector_store::initialize_vector_store(model, config, doc_manager).await?;
///
///     Ok(())
/// }
/// ```
pub async fn create_vector_store(
    model: impl EmbeddingModel,
    document_manager: DocumentManager,
) -> AppResult<impl VectorStoreIndex> {
    // 加载文档
    let docs = document_manager.get_all_documents().await;
    let mut documents: Vec<(Document, OneOrMany<Embedding>)> = Vec::new();

    // 将文档分成25个一组的块进行处理
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

        documents.extend(builder.build().await.unwrap());
    }

    let vector_store = InMemoryVectorStore::from_documents(documents);
    let index = vector_store.index(model);

    Ok(index)
}

#[derive(Clone)]
pub struct VectorStoreManager {
    stores: Arc<RwLock<HashMap<String, InMemoryVectorStore<Document>>>>,
}

impl VectorStoreManager {
    pub fn new() -> Self {
        Self {
            stores: Arc::new(RwLock::new(HashMap::new())),
        }
    }

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

    pub async fn find_store(&self, category: &str) -> Option<InMemoryVectorStore<Document>> {
        self.stores.read().await.get(category).cloned()
    }
}

async fn build_documents(
    docs: Vec<String>,
    model: impl EmbeddingModel,
) -> AppResult<Vec<(Document, OneOrMany<Embedding>)>> {
    let mut documents: Vec<(Document, OneOrMany<Embedding>)> = Vec::new();

    // 将文档分成25个一组的块进行处理
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
