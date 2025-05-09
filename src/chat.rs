use crate::errors::{AppError, AppResult};
use futures_util::stream::StreamExt;
use rig::agent::Agent;
use rig::completion::Chat;
use rig::message::Message;
use rig::streaming::{StreamingChat, StreamingChoice, StreamingCompletionModel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use tokio::time::{Duration, Instant};

/// 会话消息结构体
///
/// 表示聊天会话中的一条消息，包含消息内容和唯一ID
#[derive(Debug, Clone)]
pub struct SessionMessage {
    /// 消息内容
    pub message: String,
    /// 消息唯一标识符
    pub message_id: String,
}

/// AI聊天会话
///
/// 管理与AI代理的对话，包含历史记录和状态信息
#[derive(Clone)]
pub struct ChatSession<M: StreamingCompletionModel> {
    /// 文档类别
    doc_category: Option<String>,
    /// 会话预设
    preamble: String,
    /// 会话摘要
    summary: Arc<RwLock<String>>,
    /// 代理
    agent: Arc<Agent<M>>,
    /// 会话历史
    history: Arc<RwLock<Vec<Message>>>,
    /// 最后一条消息时间
    last_message_at: Arc<RwLock<Option<Instant>>>,
    /// 会话消息发送器，用于向会话流发送用户查询
    session_tx: broadcast::Sender<SessionMessage>,
}

impl<M: StreamingCompletionModel> ChatSession<M> {
    /// 将会话转换为可序列化的视图对象
    ///
    /// # 返回值
    /// 返回会话的视图对象，包含会话的基本信息
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::ChatSession;
    ///
    /// async fn example(session: ChatSession) {
    ///     let view = session.to_view().await;
    ///     println!("会话摘要: {}", view.summary);
    /// }
    /// ```
    pub async fn to_view(&self) -> ChatSessionView {
        ChatSessionView {
            summary: self.summary().await,
            history: self.get_history().await,
            last_message_at: self.last_message_at().await.elapsed().as_millis() as i64,
            preamble: self.preamble.clone(),
            doc_category: self.doc_category.clone(),
        }
    }

    /// 从视图对象恢复会话
    ///
    /// # 参数
    /// * `view` - 会话视图对象
    /// * `agent` - AI代理
    ///
    /// # 返回值
    /// 返回恢复的聊天会话，如果恢复过程中发生错误则返回错误
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::{ChatSession, ChatSessionView};
    /// use rig::agent::Agent;
    ///
    /// async fn example(
    ///     view: ChatSessionView,
    ///     agent: Agent<impl StreamingCompletionModel>
    /// ) -> Result<ChatSession<impl StreamingCompletionModel>, Box<dyn std::error::Error>> {
    ///     let session = ChatSession::from_view(view, agent).await?;
    ///     Ok(session)
    /// }
    /// ```
    pub async fn from_view(view: ChatSessionView, agent: Agent<M>) -> AppResult<Self> {
        let mut session = Self::new(agent, view.preamble, view.doc_category).await?;

        session.set_history(view.history).await;
        *session.summary.write().await = view.summary;
        *session.last_message_at.write().await =
            Some(Instant::now() - Duration::from_millis(view.last_message_at as u64));

        Ok(session)
    }

    /// 创建一个新的聊天会话
    ///
    /// # 参数
    /// * `agent` - AI代理
    /// * `preamble` - 会话预设
    /// * `doc_category` - 可选的文档类别
    ///
    /// # 返回值
    /// 返回新创建的聊天会话，如果创建过程中发生错误则返回错误
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::ChatSession;
    /// use rig::agent::Agent;
    ///
    /// async fn example(
    ///     agent: Agent<impl StreamingCompletionModel>
    /// ) -> Result<ChatSession<impl StreamingCompletionModel>, Box<dyn std::error::Error>> {
    ///     let session = ChatSession::new(agent, "欢迎使用AI助手".to_string(), None).await?;
    ///     Ok(session)
    /// }
    /// ```
    pub async fn new(
        agent: Agent<M>,
        preamble: String,
        doc_category: Option<String>,
    ) -> AppResult<Self> {
        let (session_tx, _) = broadcast::channel(100);

        Ok(Self {
            doc_category,
            preamble,
            summary: Arc::new(RwLock::new(String::from("新会话"))),
            agent: Arc::new(agent),
            history: Arc::new(RwLock::new(Vec::new())),
            last_message_at: Arc::new(RwLock::new(None)),
            session_tx,
        })
    }

    /// 获取会话历史
    ///
    /// # 返回值
    /// 返回会话的所有历史消息
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::ChatSession;
    ///
    /// async fn example(session: ChatSession<impl StreamingCompletionModel>) {
    ///     let history = session.get_history().await;
    ///     println!("会话历史消息数: {}", history.len());
    /// }
    /// ```
    pub async fn get_history(&self) -> Vec<Message> {
        self.history.read().await.clone()
    }

    /// 清空会话历史
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::ChatSession;
    ///
    /// async fn example(mut session: ChatSession<impl StreamingCompletionModel>) {
    ///     session.clear_history().await;
    ///     assert_eq!(session.get_history().await.len(), 0);
    /// }
    /// ```
    #[allow(dead_code)]
    pub async fn clear_history(&mut self) {
        self.history.write().await.clear();
    }

    /// 设置会话历史
    ///
    /// # 参数
    /// * `history` - 新的会话历史消息列表
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::ChatSession;
    /// use rig::message::Message;
    ///
    /// async fn example(mut session: ChatSession<impl StreamingCompletionModel>) {
    ///     let new_history = vec![
    ///         Message::user("你好"),
    ///         Message::assistant("你好！有什么可以帮助你的吗？")
    ///     ];
    ///     session.set_history(new_history).await;
    /// }
    /// ```
    pub async fn set_history(&mut self, history: Vec<Message>) {
        *self.history.write().await = history;
    }

    /// 添加消息到历史
    ///
    /// # 参数
    /// * `message` - 要添加的消息
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::ChatSession;
    /// use rig::message::Message;
    ///
    /// async fn example(mut session: ChatSession<impl StreamingCompletionModel>) {
    ///     session.add_to_history(Message::user("新的问题")).await;
    /// }
    /// ```
    #[allow(dead_code)]
    pub async fn add_to_history(&mut self, message: Message) {
        self.history.write().await.push(message);
    }

    /// 生成会话摘要
    ///
    /// 使用AI代理为当前会话生成简短摘要
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::ChatSession;
    ///
    /// async fn example(mut session: ChatSession<impl StreamingCompletionModel>) {
    ///     session.do_summary().await;
    ///     println!("会话摘要: {}", session.summary().await);
    /// }
    /// ```
    pub async fn do_summary(&mut self) {
        let history = self.get_history().await;
        let summary = self
            .agent
            .chat(
                "帮我总结这次聊天的内容为7个字，直接给我结果，不要多余的话，我只要7个字，一个字都不能超出",
                history,
            )
            .await
            .unwrap_or("无法总结的会话".to_string());

        *self.summary.write().await = summary;
    }

    /// 获取最后消息时间
    ///
    /// # 返回值
    /// 返回最后一条消息的时间戳
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::ChatSession;
    ///
    /// async fn example(session: ChatSession<impl StreamingCompletionModel>) {
    ///     let last_message_time = session.last_message_at().await;
    ///     println!("距离上次消息已过去: {}秒", last_message_time.elapsed().as_secs());
    /// }
    /// ```
    pub async fn last_message_at(&self) -> Instant {
        self.last_message_at.read().await.unwrap_or(Instant::now())
    }

    /// 获取会话摘要
    ///
    /// # 返回值
    /// 返回当前会话的摘要
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::ChatSession;
    ///
    /// async fn example(session: ChatSession<impl StreamingCompletionModel>) {
    ///     let summary = session.summary().await;
    ///     println!("会话摘要: {}", summary);
    /// }
    /// ```
    pub async fn summary(&self) -> String {
        self.summary.read().await.clone()
    }

    /// 获取消息接收器
    ///
    /// # 返回值
    /// 返回可以接收会话消息的接收器
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::ChatSession;
    /// use tokio::sync::broadcast;
    ///
    /// async fn example(session: ChatSession<impl StreamingCompletionModel>) {
    ///     let mut receiver = session.subscribe();
    ///     tokio::spawn(async move {
    ///         while let Ok(message) = receiver.recv().await {
    ///             println!("收到消息: {}", message.message);
    ///         }
    ///     });
    /// }
    /// ```
    pub fn subscribe(&self) -> broadcast::Receiver<SessionMessage> {
        self.session_tx.subscribe()
    }

    /// 发送消息并使用回调处理响应
    ///
    /// # 参数
    /// * `user_input` - 用户输入的消息
    /// * `message_id` - 消息的唯一标识符
    ///
    /// # 返回值
    /// 如果消息发送成功则返回Ok，否则返回错误
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::ChatSession;
    /// use uuid::Uuid;
    ///
    /// async fn example(mut session: ChatSession<impl StreamingCompletionModel>) -> Result<(), Box<dyn std::error::Error>> {
    ///     let message_id = Uuid::new_v4().to_string();
    ///     session.send_message("你好，AI助手", message_id).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_message(&mut self, user_input: &str, message_id: String) -> AppResult<()> {
        let mut response = self
            .agent
            .stream_chat(user_input, self.history.read().await.clone())
            .await?;

        let mut response_text = String::new();

        // 添加用户消息到历史
        self.history.write().await.push(Message::user(user_input));

        // 处理流式响应
        while let Some(chunk) = response.next().await {
            match chunk {
                Ok(choice) => match &choice {
                    StreamingChoice::Message(text) => {
                        response_text.push_str(text);
                        self.session_tx.send(SessionMessage {
                            message: text.clone(),
                            message_id: message_id.clone(),
                        })?;
                    }
                    _ => {}
                },
                Err(e) => return Err(AppError::CompletionError(e)),
            }
        }

        // 只有在成功收到响应后才添加到历史
        if !response_text.is_empty() {
            self.history
                .write()
                .await
                .push(Message::assistant(response_text.clone()));
        }

        // update last message at
        *self.last_message_at.write().await = Some(Instant::now());

        Ok(())
    }

    /// 处理工具调用并返回结果
    ///
    /// # 参数
    /// * `name` - 工具名称
    /// * `args` - 工具参数，JSON格式
    ///
    /// # 返回值
    /// 成功返回工具调用结果，失败返回错误信息
    ///
    /// # 示例
    /// ```
    /// use fsy_ai_chat::chat::ChatSession;
    ///
    /// async fn example(session: ChatSession<impl StreamingCompletionModel>) -> Result<(), Box<dyn std::error::Error>> {
    ///     let result = session.handle_tool_call("add", r#"{"x": 5, "y": 3}"#.to_string()).await;
    ///     match result {
    ///         Ok(value) => println!("工具调用结果: {}", value),
    ///         Err(e) => println!("调用失败: {}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    #[allow(dead_code)]
    pub async fn handle_tool_call(&self, name: &str, args: String) -> Result<String, String> {
        self.agent
            .tools
            .call(name, args)
            .await
            .map_err(|e| format!("工具调用错误: {:?}", e))
    }
}

/// 会话视图结构体
///
/// 提供聊天会话的可序列化视图，用于持久化存储和恢复
#[derive(Serialize, Deserialize)]
pub struct ChatSessionView {
    /// 会话摘要
    pub summary: String,
    /// 会话历史消息
    pub history: Vec<Message>,
    /// 最后一条消息的时间戳（毫秒）
    pub last_message_at: i64,
    /// 会话预设
    pub preamble: String,
    /// 文档类别
    pub doc_category: Option<String>,
}
