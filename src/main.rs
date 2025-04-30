mod agent;
mod chat;
mod client;
mod config;
mod document_loader;
mod errors;
mod kernel;
mod models;
mod session_manager;
mod tools;
mod vector_store;
mod web;

use crate::config::Config;
use crate::errors::AppResult;
use crate::session_manager::storages::storage::Storage;
use clap::Parser;
use kernel::Kernel;
use session_manager::storages::file::FileStorage;
use std::net::SocketAddr;
use std::path::PathBuf;
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use web::AppState;

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
async fn dump_chat_sessions(app_state: &Kernel) {
    let file_storage = FileStorage::new("./".to_string());

    info!("启动聊天会话持久化任务");
    // 创建一个无限循环，每5秒执行一次持久化操作
    loop {
        info!("开始持久化聊天会话");
        let result = file_storage.persistence(app_state).await;
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
async fn load_chat_sessions(app_state: &Kernel) {
    let file_storage = FileStorage::new("./".to_string());

    file_storage
        .load(app_state)
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
async fn start_web_server(config: Config, port: u16) -> AppResult<()> {
    info!("初始化Web服务器");

    let client = client::create_client(&config.agent.api_key);

    let kernel = Kernel::new(config, client).await;

    // 初始化聊天会话管理器
    let app_state = AppState::new(kernel.clone());

    load_chat_sessions(&kernel).await;

    info!("启动聊天会话持久化后台任务");
    tokio::spawn(async move {
        dump_chat_sessions(&kernel).await;
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

    start_web_server(config, args.port).await?;

    Ok(())
}
