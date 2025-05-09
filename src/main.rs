mod aliyun;
mod chat;
mod config;
mod document_loader;
mod errors;
mod kernel;
mod models;
mod session_manager;
mod storages;
mod tools;
mod vector_store;
mod web;

use crate::config::Config;
use crate::errors::AppResult;
use crate::storages::file::FileStorage;
use crate::storages::storage::Storage;
use clap::Parser;
use kernel::Kernel;
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

/// 从文件系统加载聊天会话到内核中
///
/// 从文件存储中恢复之前保存的所有聊天会话到系统内核
///
/// # 参数
/// * `kernel` - 系统内核实例
///
/// # 示例
/// ```
/// use fsy_ai_chat::kernel::Kernel;
///
/// async fn example(kernel: &Kernel) {
///     load_chat_sessions(kernel).await;
///     println!("会话加载完成");
/// }
/// ```
async fn load_chat_sessions(kernel: &Kernel) {
    // 创建文件存储并加载会话
    let storage = FileStorage::new("./data".to_string());

    match storage.load(kernel).await {
        Ok(_) => {
            info!("聊天会话加载完成");
        }
        Err(e) => {
            info!("加载会话时发生错误: {}", e);
        }
    }
}

/// 定期持久化聊天会话
///
/// 启动一个后台任务，定期将聊天会话状态保存到文件系统
///
/// # 参数
/// * `kernel` - 系统内核实例，包含需要持久化的会话数据
///
/// # 示例
/// ```
/// use fsy_ai_chat::kernel::Kernel;
///
/// async fn example(kernel: &Kernel) {
///     dump_chat_sessions(kernel).await;
/// }
/// ```
async fn dump_chat_sessions(kernel: &Kernel) {
    use tokio::time::{Duration, interval};

    // 创建文件存储
    let storage = FileStorage::new("./data".to_string());

    // 创建30秒间隔的定时器
    let mut interval = interval(Duration::from_secs(30));

    loop {
        // 等待下一个时间点
        interval.tick().await;

        // 持久化会话
        match storage.persistence(kernel).await {
            Ok(_) => {
                info!("聊天会话持久化完成");
            }
            Err(e) => {
                info!("持久化会话时发生错误: {}", e);
            }
        }
    }
}

/// 启动Web服务器
///
/// 初始化并启动Web服务，包括加载会话、设置持久化任务等
///
/// # 参数
/// * `config` - 应用配置
/// * `port` - Web服务器监听端口
///
/// # 返回值
/// 如果服务器启动并运行成功直到关闭则返回Ok，否则返回错误
///
/// # 示例
/// ```
/// use fsy_ai_chat::config::Config;
///
/// async fn example(config: Config) -> Result<(), Box<dyn std::error::Error>> {
///     start_web_server(config, 3000).await?;
///     Ok(())
/// }
/// ```
async fn start_web_server(config: Config, port: u16) -> AppResult<()> {
    info!("初始化Web服务器");

    let kernel = Kernel::new(config).await;

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
