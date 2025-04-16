mod agent;
mod chat;
mod config;
mod document_loader;
mod errors;
mod models;
mod tools;
mod vector_store;
mod web;

use crate::config::Config;
use crate::document_loader::DocumentManager;
use crate::errors::AppResult;
use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use web::AppState;

/// FSY AI聊天应用程序
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 配置文件路径
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,

    /// 运行模式: web或cli
    #[arg(short, long, default_value = "cli")]
    mode: String,

    /// Web服务器端口
    #[arg(short, long, default_value = "3000")]
    port: u16,
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
        manager
            .load_category(category.clone(), &category.directory)
            .await?;
    }

    Ok(manager)
}

/// 启动Web服务器
async fn start_web_server(
    config: Config,
    doc_manager: DocumentManager,
    port: u16,
) -> AppResult<()> {
    // 初始化聊天会话管理器
    let app_state = AppState::new(config, doc_manager);

    // 创建路由
    let app = web::create_router(app_state);

    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Web服务器已启动，监听地址: {}", addr);

    // 启动服务器
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(())
}

#[tokio::main]
async fn main() -> AppResult<()> {
    // 解析命令行参数
    let args = Args::parse();

    // 加载配置
    let config = load_config(&args.config)?;

    // 初始化文档管理器
    let doc_manager = initialize_document_manager(&config).await?;

    // 根据运行模式启动服务
    match args.mode.as_str() {
        "web" => {
            start_web_server(config, doc_manager, args.port).await?;
        }
        _ => {
            // CLI模式
            let doc_manager_clone = doc_manager.clone();

            // 初始化聊天会话
            let embedding_config = config.embedding.clone();
            let agent_config = config.agent.clone();
            let qdrant_url = config.qdrant_url.clone();

            let session = chat::ChatSession::new(
                agent_config,
                embedding_config,
                Some(doc_manager),
                Some(qdrant_url),
                Some("客服".to_string()),
            )
            .await?;

            // 启动聊天会话
            chat::cli::start_cli_session(session, config, doc_manager_clone).await;
        }
    }

    Ok(())
}
