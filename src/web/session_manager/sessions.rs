use std::{collections::HashMap, ops::Deref, sync::Arc};

use futures_util::future::join_all;
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
pub type UserID = String;

/// 聊天会话集合，管理单个用户的所有聊天会话
#[derive(Clone)]
pub struct ChatSessions {
    inner: HashMap<String, ChatSession>,
}

impl ChatSessions {
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
    pub fn get(&self, session_id: &String) -> Option<&ChatSession> {
        self.inner.get(session_id)
    }

    /// 添加一个新的聊天会话
    ///
    /// # 参数
    /// * `session_id` - 会话ID
    /// * `session` - 聊天会话对象
    pub fn insert(&mut self, session_id: String, session: ChatSession) {
        self.inner.insert(session_id, session);
    }

    /// 移除指定会话ID的聊天会话
    ///
    /// # 参数
    /// * `session_id` - 需要移除的会话ID
    ///
    /// # 返回
    /// * `Option<ChatSession>` - 如果存在则返回被移除的会话，否则返回None
    pub fn remove(&mut self, session_id: &String) -> Option<ChatSession> {
        self.inner.remove(session_id)
    }

    /// 获取当前会话集合中的会话数量
    ///
    /// # 返回
    /// * `usize` - 会话数量
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 检查会话集合是否为空
    ///
    /// # 返回
    /// * `bool` - 如果为空则返回true，否则返回false
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// 将会话集合转换为会话向量
    ///
    /// # 返回
    /// * `Vec<ChatSession>` - 包含所有会话的向量
    pub fn to_vec(&self) -> Vec<ChatSession> {
        self.inner.values().cloned().collect()
    }
}

impl Deref for ChatSessions {
    type Target = HashMap<String, ChatSession>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// 全局会话管理器，管理所有用户的聊天会话
#[derive(Clone)]
pub struct Sessions {
    /// 按用户ID组织的会话集合
    grouped: Arc<Mutex<HashMap<UserID, ChatSessions>>>,

    /// 所有会话的快速索引，不区分用户ID，用于通过session_id快速获取ChatSession
    index: Arc<Mutex<ChatSessions>>,
}

impl Sessions {
    /// 创建一个新的全局会话管理器
    pub fn new() -> Self {
        Self {
            grouped: Arc::new(Mutex::new(HashMap::new())),
            index: Arc::new(Mutex::new(ChatSessions::new())),
        }
    }

    /// 获取指定用户的所有会话
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    ///
    /// # 返回
    /// * `Option<ChatSessions>` - 如果存在则返回用户的会话集合，否则返回None
    pub async fn get_sessions(&self, user_id: &UserID) -> Option<ChatSessions> {
        self.grouped.lock().await.get(user_id).cloned()
    }

    /// 获取指定用户的会话数量
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    ///
    /// # 返回
    /// * `usize` - 该用户的会话数量
    pub async fn len(&self, user_id: &UserID) -> usize {
        self.grouped
            .lock()
            .await
            .get(user_id)
            .map_or(0, |sessions| sessions.len())
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
    pub async fn get_session(&self, session_id: impl Into<String>) -> Option<ChatSession> {
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
        session: ChatSession,
    ) {
        let mut guard = self.grouped.lock().await;
        let sessions = guard.entry(user_id).or_insert(ChatSessions::new());
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
    ) -> Option<ChatSession> {
        let mut guard = self.grouped.lock().await;
        let session_id = session_id.into();

        guard
            .get_mut(user_id)
            .and_then(|sessions| sessions.remove(&session_id))
    }

    /// 获取所有用户及其会话的集合
    ///
    /// # 返回
    /// * `Vec<(UserID, ChatSessions)>` - 包含所有用户及其会话的集合
    ///
    /// # 示例
    /// ```rust
    /// let sessions = Sessions::new();
    /// let vec = sessions.into_iter().await;
    /// ```
    pub async fn into_iter(&self) -> Vec<(UserID, ChatSessions)> {
        self.grouped
            .lock()
            .await
            .iter()
            .map(|(user_id, sessions)| (user_id.clone(), sessions.clone()))
            .collect()
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
