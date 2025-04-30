mod agent;
mod chat;
mod client;
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
use crate::web::storage::Storage;
use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use vector_store::VectorStoreManager;
use web::AppState;
use web::file::FileStorage;

/// FSY AI聊天应用程序
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 配置文件路径
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,

    /// Web服务器端口
    #[arg(short, long, default_value = "3000")]
    port: u16,
}

/// 从文件加载配置
///
/// # 参数
/// * `path` - 配置文件路径
///
/// # 返回值
/// 返回加载的配置对象，如果加载失败则返回错误
///
/// # 示例
/// ```
/// use std::path::PathBuf;
/// use fsy_ai_chat::main::load_config;
///
/// fn example() -> Result<(), Box<dyn std::error::Error>> {
///     let config_path = PathBuf::from("config.toml");
///     let config = load_config(&config_path)?;
///     println!("加载配置成功");
///     Ok(())
/// }
/// ```
fn load_config(path: &PathBuf) -> AppResult<Config> {
    let config_content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(config)
}

/// 初始化文档管理器
///
/// 根据配置中的文档类目信息，加载所有文档
///
/// # 参数
/// * `config` - 应用配置
///
/// # 返回值
/// 返回初始化好的文档管理器，如果初始化失败则返回错误
///
/// # 示例
/// ```
/// use fsy_ai_chat::config::Config;
///
/// async fn example(config: Config) -> Result<(), Box<dyn std::error::Error>> {
///     let doc_manager = initialize_document_manager(&config).await?;
///     println!("文档管理器初始化成功");
///     Ok(())
/// }
/// ```
async fn initialize_document_manager(config: &Config) -> AppResult<DocumentManager> {
    let mut manager = DocumentManager::new();

    for category in &config.document.categories {
        manager
            .load_category(category.clone(), &category.directory)
            .await?;
    }

    Ok(manager)
}

/// 定期持久化聊天会话
///
/// 启动一个后台任务，定期将聊天会话状态保存到文件系统
///
/// # 参数
/// * `app_state` - 应用状态，包含聊天会话管理器
///
/// # 示例
/// ```
/// use fsy_ai_chat::web::AppState;
///
/// async fn example(app_state: AppState) {
///     dump_chat_sessions(app_state).await;
/// }
/// ```
async fn dump_chat_sessions(app_state: AppState) {
    let sessions = app_state.chat_session_manager.sessions();
    let file_storage = FileStorage::new("./".to_string());

    info!("启动聊天会话持久化任务");
    // 创建一个无限循环，每5秒执行一次持久化操作
    loop {
        info!("开始持久化聊天会话");
        let result = file_storage.persistence(&sessions).await;
        match result {
            Ok(_) => info!("聊天会话持久化成功"),
            Err(e) => tracing::error!("聊天会话持久化失败: {}", e),
        }

        // 等待5秒
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

/// 加载持久化的聊天会话
///
/// 从文件系统加载之前保存的聊天会话状态
///
/// # 参数
/// * `app_state` - 应用状态，包含聊天会话管理器
///
/// # 示例
/// ```
/// use fsy_ai_chat::web::AppState;
///
/// async fn example(app_state: AppState) {
///     load_chat_sessions(app_state).await;
/// }
/// ```
async fn load_chat_sessions(app_state: AppState) {
    let file_storage = FileStorage::new("./".to_string());
    let sessions = app_state.chat_session_manager.sessions();

    file_storage
        .load(
            &sessions,
            app_state.config.agent.clone(),
            app_state.config.embedding.clone(),
            Some(app_state.doc_manager.clone()),
        )
        .await
        .expect("加载聊天会话失败");

    info!("聊天会话加载完成");
}

/// 启动Web服务器
///
/// 初始化并启动Web服务，包括加载会话、设置持久化任务等
///
/// # 参数
/// * `config` - 应用配置
/// * `doc_manager` - 文档管理器
/// * `port` - Web服务器监听端口
///
/// # 返回值
/// 如果服务器启动并运行成功直到关闭则返回Ok，否则返回错误
///
/// # 示例
/// ```
/// use fsy_ai_chat::config::Config;
/// use fsy_ai_chat::document_loader::DocumentManager;
///
/// async fn example(
///     config: Config,
///     doc_manager: DocumentManager
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     start_web_server(config, doc_manager, 3000).await?;
///     Ok(())
/// }
/// ```
async fn start_web_server(
    config: Config,
    doc_manager: DocumentManager,
    port: u16,
) -> AppResult<()> {
    info!("初始化Web服务器");

    let client = client::create_client(&config.agent.api_key);

    // 初始化聊天会话管理器
    let app_state = AppState::new(config, doc_manager, client).await;

    load_chat_sessions(app_state.clone()).await;

    info!("启动聊天会话持久化后台任务");
    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        dump_chat_sessions(app_state_clone).await;
    });

    // 创建路由
    let app = web::create_router(app_state);

    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Web服务器已启动，监听地址: {}", addr);

    // 启动服务器
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(())
}

/// 程序入口点
///
/// 解析命令行参数，加载配置，根据运行模式启动CLI或Web服务
///
/// # 示例
/// ```no_run
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     fsy_ai_chat::main();
///     Ok(())
/// }
/// ```
#[tokio::main]
async fn main() -> AppResult<()> {
    // 初始化tracing
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    info!("应用程序启动");

    // 解析命令行参数
    let args = Args::parse();

    // 加载配置
    let config = load_config(&args.config)?;
    info!("配置加载完成");

    // 初始化文档管理器
    let doc_manager = initialize_document_manager(&config).await?;
    info!("文档管理器初始化完成");

    start_web_server(config, doc_manager, args.port).await?;

    Ok(())
}
