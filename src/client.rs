use rig::providers::openai;

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
pub fn create_client(api_key: &str) -> openai::Client {
    openai::Client::from_url(api_key, "https://dashscope.aliyuncs.com/compatible-mode/v1")
}
