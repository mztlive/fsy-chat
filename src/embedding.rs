use rig::providers::openai;

use crate::agent::EmbeddingConfig;

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
pub async fn create_embedding_model(
    client: openai::Client,
    config: &EmbeddingConfig,
) -> openai::EmbeddingModel {
    openai::EmbeddingModel::new(client, &config.model, config.dimensions)
}
