use crate::vector_store;
use rig::agent::Agent;
use rig::providers::openai;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AgentInitConfig {
    pub api_key: String,
    pub embedding_model: String,
    pub embedding_model_dimensions: usize,
    pub preamble: String,
    pub chat_model: String,
    pub vector_store: String,
    pub context_size: usize,
}

// pub const API_KEY: &str = "sk-33e3643fd6db453c9015697413b44bae";

pub async fn initialize_agent(config: AgentInitConfig) -> Agent<openai::CompletionModel> {
    let client = openai::Client::from_url(
        &config.api_key,
        "https://dashscope.aliyuncs.com/compatible-mode/v1",
    );

    let embedding = openai::EmbeddingModel::new(
        client.clone(),
        &config.embedding_model,
        config.embedding_model_dimensions,
    );

    let index = vector_store::initialize_vector_store(embedding.clone()).await;

    let agent = client
        .agent(&config.chat_model)
        .preamble(&config.preamble)
        .tool(crate::tools::adder::Adder)
        .dynamic_context(3, index)
        .build();

    agent
}
