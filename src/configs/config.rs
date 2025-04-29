use serde::Deserialize;

use super::document::DocumentConfig;
use super::embedding::EmbeddingConfig;
use super::provider::Providers;
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub providers: Providers,

    pub embedding: EmbeddingConfig,

    pub document: DocumentConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml() {
        let cfg_str = r#"
[providers]
openai = { api_key = "sk-33e3643fd6db453c9015697413b44bae", models = [
    "gpt-4o",
    "gpt-4o-mini",
] }
anthropic = { api_key = "sk-ant-api03-33e3643fd6db453c9015697413b44bae", models = [
    "claude-3-5-sonnet-20240620",
] }
gemini = { api_key = "sk-33e3643fd6db453c9015697413b44bae", models = [
    "gemini-1.5-flash",
] }
deepseek = { api_key = "sk-33e3643fd6db453c9015697413b44bae", models = [
    "deepseek-coder",
] }

[embedding]
api_key = "sk-33e3643fd6db453c9015697413b44bae"
model = "text-embedding-v2"

[document]
[[document.categories]]
name = "faq"
directory = "./docs/faq"

[[document.categories]]
name = "faq"
directory = "./docs/faq"

[[document.categories]]
name = "faq"
directory = "./docs/faq"
        "#;
        let cfg: Config = toml::from_str(cfg_str).unwrap();
        println!("{:?}", cfg);
    }
}
