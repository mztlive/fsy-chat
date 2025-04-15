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
    pub async fn new(config: AgentConfig) -> AppResult<Self> {
        let agent = crate::agent::initialize_agent(config, None, None).await?;

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

    pub fn handle_special_commands(input: &str, session: &mut ChatSession) -> bool {
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
            _ if input.is_empty() => return true,
            _ => return false,
        }
    }

    /// 启动CLI聊天会话
    pub async fn start_cli_session(mut session: ChatSession) {
        print_welcome_message();

        loop {
            let user_input = read_user_input();

            if handle_special_commands(&user_input, &mut session) {
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

/// 创建默认配置的聊天会话的便捷函数
pub async fn create_default_session() -> AppResult<ChatSession> {
    let session = ChatSession::new(AgentConfig {
        api_key: "sk-33e3643fd6db453c9015697413b44bae".to_string(),
        preamble: "你是一个无所不知的助手".to_string(),
        chat_model: "qwen-max".to_string(),
    })
    .await?;

    Ok(session)
}

/// 为了兼容性保留的原始入口点
pub async fn start_chat_session() {
    let session = create_default_session().await.unwrap();
    cli::start_cli_session(session).await;
}
