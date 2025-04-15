use futures_util::Stream;
use futures_util::stream::StreamExt;
use rig::agent::Agent;
use rig::completion::CompletionError;
use rig::message::Message;
use rig::streaming::{StreamingChat, StreamingChoice, StreamingCompletionModel};
use std::io::{self, Write};
use std::pin::Pin;

use crate::agent::AgentInitConfig;

type ResponseStream = Pin<Box<dyn Stream<Item = Result<StreamingChoice, CompletionError>> + Send>>;

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

pub fn handle_special_commands(input: &str, history: &mut Vec<Message>) -> bool {
    match input.to_lowercase().as_str() {
        "exit" | "quit" => {
            println!("再见！");
            return true;
        }
        "clear" | "清空" => {
            history.clear();
            println!("对话历史已清空。");
            return true;
        }
        _ if input.is_empty() => return true,
        _ => return false,
    }
}

pub async fn process_response<M: StreamingCompletionModel>(
    response: &mut ResponseStream,
    user_input: &str,
    history: &mut Vec<Message>,
    agent: &Agent<M>,
) {
    let mut chunks = vec![];

    while let Some(chunk) = response.next().await {
        match chunk {
            Ok(StreamingChoice::Message(text)) => {
                print!("{}", text);
                chunks.push(text);
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
            }
            Ok(StreamingChoice::ToolCall(name, id, resp)) => {
                println!("tool call: {:?}", name);
                println!("tool call: {:?}", id);
                println!("tool call: {:?}", resp);

                agent
                    .tools
                    .call(&name, resp.to_string())
                    .await
                    .inspect(|result| {
                        println!("工具调用成功: {:?}", result);
                        chunks.push(format!("工具调用结果: {:?}", result));
                    })
                    .inspect_err(|e| {
                        println!("工具调用错误: {:?}", e);
                    })
                    .ok();
            }
            _ => {
                eprintln!("Error");
                break;
            }
        }
    }

    println!("\n");

    if chunks.len() > 0 {
        history.push(Message::user(user_input));
        history.push(Message::assistant(chunks.join("")));
    }
}

pub async fn start_chat_session() {
    let agent = crate::agent::initialize_agent(AgentInitConfig {
        api_key: "sk-33e3643fd6db453c9015697413b44bae".to_string(),
        embedding_model: "text-embedding-v2".to_string(),
        embedding_model_dimensions: 1536,
        preamble: "你是一个无所不知的助手".to_string(),
        chat_model: "qwen-max".to_string(),
        vector_store: "qwen-max".to_string(),
        context_size: 10,
    })
    .await;

    let mut history: Vec<Message> = vec![];

    print_welcome_message();

    loop {
        let user_input = read_user_input();

        if handle_special_commands(&user_input, &mut history) {
            if user_input.to_lowercase() == "exit" || user_input.to_lowercase() == "quit" {
                break;
            }
            continue;
        }

        // 发送请求到模型
        let response = agent.stream_chat(&user_input, history.clone()).await;

        match response {
            Ok(mut response) => {
                // 处理响应
                process_response(&mut response, &user_input, &mut history, &agent).await;
            }
            Err(e) => println!("错误: {}", e),
        }
    }
}
