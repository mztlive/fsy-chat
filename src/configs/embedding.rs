use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingConfig {
    pub api_key: String,
    pub model: String,
}
