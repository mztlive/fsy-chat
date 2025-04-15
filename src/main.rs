mod agent;
mod chat;
mod config;
mod document_loader;
mod errors;
mod models;
mod tools;
mod vector_store;

use crate::config::Config;
use crate::document_loader::DocumentManager;
use crate::errors::AppResult;
use clap::{ArgAction, Parser};
use std::path::PathBuf;

/// FSY AI聊天应用程序
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 配置文件路径
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,

    /// 启用向量检索
    #[arg(short, long, action = ArgAction::SetTrue)]
    vector_search: bool,
}

/// 从文件加载配置
fn load_config(path: &PathBuf) -> AppResult<Config> {
    let config_content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(config)
}

/// 初始化文档管理器
async fn initialize_document_manager(config: &Config) -> AppResult<DocumentManager> {
    let mut manager = DocumentManager::new();

    for category in &config.document.categories {
        manager.load_category(category.name.clone(), &category.directory)?;
    }

    Ok(manager)
}

#[tokio::main]
async fn main() -> AppResult<()> {
    // 解析命令行参数
    let args = Args::parse();

    // 加载配置
    let config = load_config(&args.config)?;

    // 初始化文档管理器
    let doc_manager = initialize_document_manager(&config).await?;

    // 初始化聊天会话
    let embedding_config = if args.vector_search {
        config.embedding.clone()
    } else {
        None
    };
    let agent_config = config.agent.clone();

    let session = chat::ChatSession::new(agent_config, embedding_config, Some(doc_manager)).await?;

    // 启动聊天会话
    chat::cli::start_cli_session(session).await;

    Ok(())
}
