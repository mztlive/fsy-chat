use crate::document_loader::DocumentManager;
/// 向量存储模块，提供文档嵌入和向量检索功能
use crate::errors::AppResult;
use crate::models::Document;
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, QueryPointsBuilder, VectorParamsBuilder,
};
use rig::OneOrMany;
use rig::embeddings::{Embedding, EmbeddingModel, EmbeddingsBuilder};
use rig::vector_store::VectorStoreIndex;
use rig_qdrant::QdrantVectorStore;
use tracing::info;

/// 向量存储配置结构体
///
/// 包含初始化Qdrant向量数据库所需的配置参数
pub struct VectorStoreConfig {
    /// Qdrant集合名称
    collection_name: String,
    /// Qdrant服务器URL
    qdrant_url: String,
    /// 向量维度
    dimensions: u64,
}

impl VectorStoreConfig {
    /// 创建新的向量存储配置
    ///
    /// # 参数
    /// * `collection_name` - Qdrant集合名称
    /// * `qdrant_url` - Qdrant服务器URL
    /// * `dimensions` - 向量维度
    ///
    /// # 返回值
    /// 返回配置好的VectorStoreConfig实例
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::vector_store::VectorStoreConfig;
    ///
    /// let config = VectorStoreConfig::new(
    ///     "documents".to_string(),
    ///     "http://localhost:6334".to_string(),
    ///     1536,
    /// );
    /// ```
    pub fn new(collection_name: String, qdrant_url: String, dimensions: u64) -> Self {
        Self {
            collection_name,
            qdrant_url,
            dimensions,
        }
    }
}

/// 初始化向量存储
///
/// 加载文档，创建嵌入，并初始化Qdrant向量存储。如果指定的集合不存在，会创建新集合。
///
/// # 参数
/// * `model` - 实现了EmbeddingModel特性的嵌入模型
/// * `config` - 向量存储配置
///
/// # 返回值
/// 返回初始化好的向量存储索引，如果初始化过程中发生错误则返回错误
///
/// # 示例
/// ```
/// use fsy_ai_chat::vector_store::{self, VectorStoreConfig};
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
///         "http://localhost:6334".to_string(),
///         1536,
///     );
///
///     // 初始化向量存储
///     let index = vector_store::initialize_vector_store(model, config).await?;
///
///     Ok(())
/// }
/// ```
pub async fn initialize_vector_store(
    model: impl EmbeddingModel,
    config: VectorStoreConfig,
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

    // 创建Qdrant客户端
    let client = Qdrant::from_url(config.qdrant_url.as_str())
        .skip_compatibility_check()
        .build()?;

    // 检查集合是否存在，不存在则创建
    if !client
        .collection_exists(config.collection_name.clone())
        .await?
    {
        client
            .create_collection(
                CreateCollectionBuilder::new(config.collection_name.clone()).vectors_config(
                    VectorParamsBuilder::new(config.dimensions, Distance::Cosine),
                ),
            )
            .await?;
    }

    // 配置查询参数
    let query_params = QueryPointsBuilder::new(config.collection_name.clone()).with_payload(true);

    // 创建向量存储
    let vector_store = QdrantVectorStore::new(client, model, query_params.build());

    // 插入文档
    vector_store.insert_documents(documents).await?;

    Ok(vector_store)
}
