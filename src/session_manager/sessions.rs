use std::{collections::HashMap, fmt::Display, ops::Deref, path::Path, sync::Arc};

use futures_util::{StreamExt, future::join_all};
use rig::streaming::StreamingCompletionModel;
use serde::Serialize;
use tokio::sync::Mutex;

use crate::chat::ChatSession;

/// 对前端友好的会话历史
#[derive(Debug, Clone, Serialize)]
pub struct SessionHistory {
    pub session_id: String,
    pub title: String,
}

/// 用户标识类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct UserID(pub String);

impl Display for UserID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for UserID {
    fn from(user_id: &str) -> Self {
        UserID(user_id.to_string())
    }
}

impl From<String> for UserID {
    fn from(user_id: String) -> Self {
        UserID(user_id)
    }
}

impl Deref for UserID {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Path> for UserID {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

/// 对HashMap<String, ChatSession<M>>的封装，用于管理单个用户的所有聊天会话
#[derive(Clone)]
pub struct UserChatSessions<M: StreamingCompletionModel> {
    /// key is session_id
    inner: HashMap<String, ChatSession<M>>,
}

impl<M: StreamingCompletionModel> UserChatSessions<M> {
    /// 创建一个新的聊天会话集合
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// 根据会话ID获取聊天会话
    ///
    /// # 参数
    /// * `session_id` - 会话ID
    ///
    /// # 返回
    /// * `Option<&ChatSession>` - 如果存在则返回会话引用，否则返回None
    pub fn get(&self, session_id: &String) -> Option<&ChatSession<M>> {
        self.inner.get(session_id)
    }

    /// 添加一个新的聊天会话
    ///
    /// # 参数
    /// * `session_id` - 会话ID
    /// * `session` - 聊天会话对象
    pub fn insert(&mut self, session_id: String, session: ChatSession<M>) {
        self.inner.insert(session_id, session);
    }

    /// 移除指定会话ID的聊天会话
    ///
    /// # 参数
    /// * `session_id` - 需要移除的会话ID
    ///
    /// # 返回
    /// * `Option<ChatSession>` - 如果存在则返回被移除的会话，否则返回None
    pub fn remove(&mut self, session_id: &String) -> Option<ChatSession<M>> {
        self.inner.remove(session_id)
    }

    /// 获取当前会话集合中的会话数量
    ///
    /// # 返回
    /// * `usize` - 会话数量
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<M: StreamingCompletionModel> Deref for UserChatSessions<M> {
    type Target = HashMap<String, ChatSession<M>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// 全局会话管理器，管理所有用户的聊天会话
#[derive(Clone)]
pub struct Sessions<M: StreamingCompletionModel> {
    /// 按用户ID组织的会话集合
    grouped: Arc<Mutex<HashMap<UserID, UserChatSessions<M>>>>,

    /// 所有会话的快速索引，不区分用户ID，用于通过session_id快速获取ChatSession
    index: Arc<Mutex<UserChatSessions<M>>>,
}

impl<M: StreamingCompletionModel> Sessions<M> {
    /// 创建一个新的全局会话管理器
    pub fn new() -> Self {
        Self {
            grouped: Arc::new(Mutex::new(HashMap::new())),
            index: Arc::new(Mutex::new(UserChatSessions::new())),
        }
    }

    pub async fn add_user_session(&self, user_id: UserID, sessions: UserChatSessions<M>) {
        let mut guard = self.grouped.lock().await;
        guard.insert(user_id, sessions.clone());

        for (session_id, session) in sessions.inner.iter() {
            self.index
                .lock()
                .await
                .insert(session_id.clone(), session.clone());
        }
    }

    /// 获取指定用户的所有会话
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    ///
    /// # 返回
    /// * `Option<ChatSessions>` - 如果存在则返回用户的会话集合，否则返回None
    pub async fn get_sessions(&self, user_id: &UserID) -> Option<UserChatSessions<M>> {
        self.grouped.lock().await.get(user_id).cloned()
    }

    /// 获取当前管理的用户总数
    ///
    /// # 返回
    /// * `usize` - 用户总数
    pub async fn capacity(&self) -> usize {
        self.grouped.lock().await.len()
    }

    /// 根据会话ID获取聊天会话（全局查找）
    ///
    /// # 参数
    /// * `session_id` - 会话ID
    ///
    /// # 返回
    /// * `Option<ChatSession>` - 如果存在则返回会话，否则返回None
    pub async fn get_session(&self, session_id: impl Into<String>) -> Option<ChatSession<M>> {
        self.index.lock().await.get(&session_id.into()).cloned()
    }

    /// 为指定用户添加一个新的聊天会话
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `session_id` - 会话ID
    /// * `session` - 聊天会话对象
    pub async fn add_session(
        &self,
        user_id: UserID,
        session_id: impl Into<String>,
        session: ChatSession<M>,
    ) {
        let mut guard = self.grouped.lock().await;
        let sessions = guard.entry(user_id).or_insert(UserChatSessions::new());
        let session_id = session_id.into();

        sessions.insert(session_id.clone(), session.clone());
        self.index.lock().await.insert(session_id, session);
    }

    /// 移除指定用户的特定会话
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `session_id` - 需要移除的会话ID
    ///
    /// # 返回
    /// * `Option<ChatSession>` - 如果存在则返回被移除的会话，否则返回None
    pub async fn remove_session(
        &self,
        user_id: &UserID,
        session_id: impl Into<String>,
    ) -> Option<ChatSession<M>> {
        let session_id = session_id.into();

        let mut guard = self.grouped.lock().await;
        self.index.lock().await.remove(&session_id);

        guard
            .get_mut(user_id)
            .and_then(|sessions| sessions.remove(&session_id))
    }

    /// 获取所有用户ID的集合
    ///
    /// # 返回
    /// * `Vec<UserID>` - 包含所有用户ID的集合
    pub async fn user_ids(&self) -> Vec<UserID> {
        self.grouped.lock().await.keys().cloned().collect()
    }

    /// 获取指定用户的会话历史
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    ///
    /// # 返回
    /// * `Vec<SessionHistory>` - 包含所有会话历史的集合
    pub async fn get_session_history(&self, user_id: &UserID) -> Vec<SessionHistory> {
        let chat_sessions = self.grouped.lock().await.get(user_id).cloned();

        match chat_sessions {
            Some(sessions) => {
                let futures = sessions
                    .iter()
                    .map(|(session_id, session)| {
                        let session_id = session_id.clone();
                        let session = session.clone();
                        async move {
                            let title = session.summary().await;
                            let last_message_time = session.last_message_at().await;
                            (SessionHistory { session_id, title }, last_message_time)
                        }
                    })
                    .collect::<Vec<_>>();

                let mut results = join_all(futures).await;

                // 按最后消息时间降序排序
                results.sort_by(|(_, a_time), (_, b_time)| b_time.cmp(a_time));

                // 只返回SessionHistory部分
                results.into_iter().map(|(history, _)| history).collect()
            }
            None => Vec::new(),
        }
    }
}
