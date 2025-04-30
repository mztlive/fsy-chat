/// 代理模块，提供与AI模型交互的功能，支持嵌入式向量和检索增强生成
use crate::document_loader::DocumentManager;
use crate::errors::AppResult;
use crate::vector_store;
use rig::agent::Agent;
use rig::providers::openai;
use serde::Deserialize;

/// 代理配置结构体
///
/// 包含初始化AI代理所需的基本配置信息
#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    /// OpenAI兼容API的密钥
    pub api_key: String,
    /// 代理的前置指令文本
    pub preamble: String,
    /// 使用的聊天模型名称
    pub chat_model: String,
}

/// 嵌入模型配置结构体
///
/// 包含初始化向量嵌入和检索所需的配置信息
#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingConfig {
    /// OpenAI兼容API的密钥
    pub api_key: String,
    /// 使用的嵌入模型名称
    pub model: String,
    /// 嵌入向量的维度
    pub dimensions: usize,
}

/// 创建OpenAI兼容的客户端
///
/// # 参数
/// * `api_key` - OpenAI兼容API的密钥
///
/// # 返回值
/// 返回配置好的OpenAI客户端
///
/// # 示例
/// ```
/// use fsy_ai_chat::agent;
///
/// let client = agent::create_client("your-api-key");
/// ```
fn create_client(api_key: &str) -> openai::Client {
    openai::Client::from_url(api_key, "https://dashscope.aliyuncs.com/compatible-mode/v1")
}

/// 初始化AI代理
///
/// 根据提供的配置创建代理实例，可选择是否启用向量嵌入功能
///
/// # 参数
/// * `agent_config` - 代理的基本配置
/// * `embedding_config` - 可选的嵌入模型配置，如果提供则启用检索增强生成
/// * `document_manager` - 可选的文档管理器，提供向量存储需要的文档
/// * `category_name` - 可选的文档类别名称，用于创建向量存储
///
/// # 返回值
/// 返回配置好的Agent实例，如果初始化过程中发生错误则返回错误
///
/// # 示例
/// ```
/// use fsy_ai_chat::agent::{self, AgentConfig, EmbeddingConfig};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
///     // 基本代理配置
///     let config = AgentConfig {
///         api_key: "your-api-key".to_string(),
///         preamble: "你是一个助手".to_string(),
///         chat_model: "qwen-max".to_string(),
///     };
///     
///     // 不使用嵌入功能的代理
///     let agent = agent::initialize_agent(config, None, None, None).await?;
///     
///     Ok(())
/// }
/// ```
pub async fn build_agent(
    agent_config: AgentConfig,
    embedding_config: Option<EmbeddingConfig>,
    document_manager: Option<DocumentManager>,
) -> AppResult<Agent<openai::CompletionModel>> {
    // 创建客户端
    let client = create_client(&agent_config.api_key);

    // 初始化代理构建器
    let mut builder = client
        .agent(&agent_config.chat_model)
        .preamble(&agent_config.preamble);

    // 如果提供了嵌入配置、文档管理器和类别名称，添加向量存储功能
    if let (Some(embed_config), Some(doc_manager)) = (embedding_config, document_manager) {
        // 创建嵌入模型
        let embedding = create_embedding_model(client.clone(), &embed_config).await;

        // 初始化向量存储
        let index = vector_store::create_vector_store(embedding.clone(), doc_manager).await?;

        // 添加动态上下文
        builder = builder.dynamic_context(5, index);
    }

    // 构建并返回代理
    Ok(builder.build())
}

/// 创建嵌入模型
///
/// # 参数
/// * `client` - OpenAI兼容的客户端实例
/// * `config` - 嵌入模型的配置
///
/// # 返回值
/// 返回配置好的嵌入模型实例
///
/// # 示例
/// ```
/// use fsy_ai_chat::agent::{self, EmbeddingConfig};
/// use rig::providers::openai;
///
/// async fn example() {
///     let client = openai::Client::new("your-api-key");
///     let config = EmbeddingConfig {
///         api_key: "your-api-key".to_string(),
///         model: "text-embedding-v1".to_string(),
///         dimensions: 1536,
///     };
///     
///     let embedding_model = agent::create_embedding_model(client, &config).await;
/// }
/// ```
async fn create_embedding_model(
    client: openai::Client,
    config: &EmbeddingConfig,
) -> openai::EmbeddingModel {
    openai::EmbeddingModel::new(client, &config.model, config.dimensions)
}
