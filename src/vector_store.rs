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
