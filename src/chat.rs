use futures_util::Stream;
use futures_util::stream::StreamExt;
use rig::agent::Agent;
use rig::completion::CompletionError;
use rig::message::Message;
use rig::providers::openai::CompletionModel;
use rig::streaming::{StreamingChat, StreamingChoice};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::agent::AgentConfig;
use crate::agent::EmbeddingConfig;
use crate::document_loader::DocumentManager;
use crate::errors::{AppError, AppResult};

type ResponseStream = Pin<Box<dyn Stream<Item = Result<StreamingChoice, CompletionError>> + Send>>;

/// 表示处理流式响应的回调函数
pub type ResponseCallback = Arc<dyn Fn(StreamingChoice) + Send + Sync>;

pub struct SessionMessage {
    pub message: String,
}

/// AI聊天会话
#[derive(Clone)]
pub struct ChatSession {
    agent: Arc<Agent<CompletionModel>>,
    history: Vec<Message>,
    /// 会话消息发送器，用于向会话流发送用户查询
    session_tx: mpsc::Sender<SessionMessage>,
}

impl ChatSession {
    /// 创建一个新的聊天会话
    pub async fn new(
        config: AgentConfig,
        embedding_config: Option<EmbeddingConfig>,
        document_manager: Option<DocumentManager>,
        qdrant_url: Option<String>,
        doc_category: Option<String>,
        session_tx: mpsc::Sender<SessionMessage>,
    ) -> AppResult<Self> {
        let agent = crate::agent::initialize_agent(
            config,
            embedding_config,
            document_manager,
            qdrant_url,
            doc_category,
        )
        .await?;

        Ok(Self {
            agent: Arc::new(agent),
            history: Vec::new(),
            session_tx: session_tx,
        })
    }

    /// 获取会话历史
    pub fn get_history(&self) -> &Vec<Message> {
        &self.history
    }

    /// 清空会话历史
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// 设置会话历史
    pub fn set_history(&mut self, history: Vec<Message>) {
        self.history = history;
    }

    /// 添加消息到历史
    pub fn add_to_history(&mut self, message: Message) {
        self.history.push(message);
    }

    /// 发送消息并使用回调处理响应
    pub async fn send_message(&mut self, user_input: &str) -> AppResult<()> {
        let mut response = self
            .agent
            .stream_chat(user_input, self.history.clone())
            .await?;

        let mut response_text = String::new();

        // 添加用户消息到历史
        self.history.push(Message::user(user_input));

        // 处理流式响应
        while let Some(chunk) = response.next().await {
            match chunk {
                Ok(choice) => match &choice {
                    StreamingChoice::Message(text) => {
                        response_text.push_str(text);
                        self.session_tx
                            .send(SessionMessage {
                                message: text.clone(),
                            })
                            .await?;
                    }
                    _ => {}
                },
                Err(e) => return Err(AppError::CompletionError(e)),
            }
        }

        // 只有在成功收到响应后才添加到历史
        if !response_text.is_empty() {
            self.history.push(Message::assistant(response_text.clone()));
        }

        Ok(())
    }

    /// 处理工具调用并返回结果
    pub async fn handle_tool_call(&self, name: &str, args: String) -> Result<String, String> {
        self.agent
            .tools
            .call(name, args)
            .await
            .map_err(|e| format!("工具调用错误: {:?}", e))
    }
}

/// CLI相关的帮助函数，可供CLI入口使用
pub mod cli {
    use super::*;
    use std::io::{self, Write};

    pub fn print_welcome_message() {
        println!("欢迎使用AI聊天程序！输入'exit'或'quit'退出程序。");
    }

    pub fn read_user_input() -> String {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        user_input.trim().to_string()
    }

    /// 启动CLI聊天会话
    pub async fn start_cli_session(config: crate::config::Config, doc_manager: DocumentManager) {
        print_welcome_message();

        let (session_tx, mut session_rx) = mpsc::channel::<SessionMessage>(100);

        tokio::spawn(async move {
            while let Some(message) = session_rx.recv().await {
                print!("{}", message.message);
                std::io::stdout().flush().unwrap();
            }
        });

        let mut session = ChatSession::new(
            config.agent,
            config.embedding,
            Some(doc_manager),
            Some(config.qdrant_url),
            None,
            session_tx,
        )
        .await
        .unwrap();

        loop {
            let user_input = read_user_input();

            session.send_message(&user_input).await.unwrap();
        }
    }
}
