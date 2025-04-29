use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum ProviderType {
    #[serde(rename = "openai")]
    OpenAI,
    #[serde(rename = "gemini")]
    Gemini,
    #[serde(rename = "deepseek")]
    DeepSeek,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfig {
    pub api_key: String,
    #[serde(default)]
    pub models: Vec<String>,
}

impl ProviderConfig {
    pub fn has_model(&self, model: &str) -> bool {
        self.models.contains(&model.to_string())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Providers {
    #[serde(flatten)]
    pub inner: HashMap<ProviderType, ProviderConfig>,
}

impl Providers {
    pub fn get_provider(&self, provider_type: &ProviderType) -> Option<ProviderConfig> {
        self.inner.get(provider_type).map(|config| config.clone())
    }
}
