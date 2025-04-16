use futures_util::Stream;
use futures_util::stream::StreamExt;
use rig::agent::Agent;
use rig::completion::CompletionError;
use rig::message::Message;
use rig::providers::openai::CompletionModel;
use rig::streaming::{StreamingChat, StreamingChoice, StreamingCompletionModel};
use std::pin::Pin;
use std::sync::Arc;

use crate::agent::AgentConfig;
use crate::agent::EmbeddingConfig;
use crate::document_loader::DocumentManager;
use crate::errors::AppResult;

type ResponseStream = Pin<Box<dyn Stream<Item = Result<StreamingChoice, CompletionError>> + Send>>;

/// 表示处理流式响应的回调函数
pub type ResponseCallback = Arc<dyn Fn(StreamingChoice) + Send + Sync>;

/// AI聊天会话
pub struct ChatSession {
    agent: Agent<CompletionModel>,
    history: Vec<Message>,
}

impl ChatSession {
    /// 创建一个新的聊天会话
    pub async fn new(
        config: AgentConfig,
        embedding_config: Option<EmbeddingConfig>,
        document_manager: Option<DocumentManager>,
        qdrant_url: Option<String>,
        doc_category: Option<String>,
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
            agent,
            history: Vec::new(),
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

    /// 发送用户消息并获取响应流
    pub async fn send_message(&self, user_input: &str) -> Result<ResponseStream, CompletionError> {
        self.agent
            .stream_chat(user_input, self.history.clone())
            .await
    }

    /// 发送消息并使用回调处理响应
    pub async fn send_message_with_callback(
        &mut self,
        user_input: &str,
        callback: ResponseCallback,
    ) -> Result<(), CompletionError> {
        let mut response = self.send_message(user_input).await?;
        let mut response_text = String::new();

        // 添加用户消息到历史
        self.history.push(Message::user(user_input));

        // 处理流式响应
        while let Some(chunk) = response.next().await {
            match chunk {
                Ok(choice) => {
                    match &choice {
                        StreamingChoice::Message(text) => {
                            response_text.push_str(text);
                        }
                        _ => {}
                    }

                    // 调用回调函数
                    callback(choice);
                }
                Err(e) => return Err(e),
            }
        }

        // 只有在成功收到响应后才添加到历史
        if !response_text.is_empty() {
            self.history.push(Message::assistant(response_text));
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
        println!("使用'category <类别名称>'切换到特定类别的文档。");
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

    pub async fn handle_special_commands(
        input: &str,
        session: &mut ChatSession,
        config: &crate::config::Config,
        doc_manager: &DocumentManager,
    ) -> bool {
        match input.to_lowercase().as_str() {
            "exit" | "quit" => {
                println!("再见！");
                return true;
            }
            "clear" | "清空" => {
                session.clear_history();
                println!("对话历史已清空。");
                return true;
            }
            _ if input.starts_with("category ") => {
                let category = input.trim_start_matches("category ").trim();
                if category.is_empty() {
                    println!("请指定类别名称。");
                    return true;
                }

                // 检查类别是否存在
                if doc_manager.get_category_config(category).await.is_none() {
                    println!("类别'{}' 不存在。", category);
                    println!("可用类别: {:?}", doc_manager.get_categories().await);
                    return true;
                }

                // 创建新的会话
                match create_session_with_category(
                    config.agent.clone(),
                    config.embedding.clone(),
                    doc_manager.clone(),
                    config.qdrant_url.clone(),
                    Some(category.to_string()),
                )
                .await
                {
                    Ok(new_session) => {
                        *session = new_session;
                        println!("已切换到类别: {}", category);
                    }
                    Err(e) => {
                        println!("创建会话失败: {}", e);
                    }
                }
                return true;
            }
            _ if input.is_empty() => return true,
            _ => return false,
        }
    }

    /// 创建带有指定文档类别的聊天会话
    pub async fn create_session_with_category(
        agent_config: crate::agent::AgentConfig,
        embedding_config: Option<crate::agent::EmbeddingConfig>,
        doc_manager: DocumentManager,
        qdrant_url: String,
        category: Option<String>,
    ) -> AppResult<ChatSession> {
        ChatSession::new(
            agent_config,
            embedding_config,
            Some(doc_manager),
            Some(qdrant_url),
            category,
        )
        .await
    }

    /// 启动CLI聊天会话
    pub async fn start_cli_session(
        mut session: ChatSession,
        config: crate::config::Config,
        doc_manager: DocumentManager,
    ) {
        print_welcome_message();

        loop {
            let user_input = read_user_input();

            if handle_special_commands(&user_input, &mut session, &config, &doc_manager).await {
                if user_input.to_lowercase() == "exit" || user_input.to_lowercase() == "quit" {
                    break;
                }
                continue;
            }

            let callback = Arc::new(|choice: StreamingChoice| match choice {
                StreamingChoice::Message(text) => {
                    print!("{}", text);
                    std::io::stdout().flush().unwrap();
                }
                StreamingChoice::ToolCall(name, id, args) => {
                    println!("\n工具调用: {} (ID: {})", name, id);
                    println!("参数: {}", args);
                }
                _ => {}
            });

            match session
                .send_message_with_callback(&user_input, callback)
                .await
            {
                Ok(_) => println!("\n"),
                Err(e) => println!("错误: {}", e),
            }
        }
    }
}
