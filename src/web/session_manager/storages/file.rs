use std::path::{Path, PathBuf};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::{
    chat::ChatSession,
    web::session_manager::{ChatSessions, Sessions},
};

use super::storage::{Storage, StorageError};

pub struct FileStorage {
    pub base_path: String,
}

impl FileStorage {
    pub fn new(path: String) -> Self {
        Self { base_path: path }
    }

    async fn save_user_sessions(
        &self,
        sessions: &ChatSessions,
        user_dir: &PathBuf,
    ) -> Result<(), StorageError> {
        let sessions = sessions
            .iter()
            .map(|(session_id, session)| (session_id, session))
            .collect::<Vec<(&String, &ChatSession)>>();

        for (session_id, session) in sessions {
            let mut file = File::create(user_dir.join(format!("{}.json", session_id))).await?;

            let history = session.get_history().await;
            let history = serde_json::to_string(&history)?;
            file.write_all(history.as_bytes()).await?;
        }

        Ok(())
    }
}

impl Storage for FileStorage {
    async fn persistence(&self, sessions: &Sessions) -> Result<(), StorageError> {
        // 确保目录存在
        let sessions_dir = Path::new(&self.base_path).join("sessions");
        tokio::fs::create_dir_all(&sessions_dir).await?;

        for user_id in sessions.user_ids().await {
            // 创建用户的目录
            let user_dir = sessions_dir.clone().join(&user_id);
            tokio::fs::create_dir_all(&user_dir).await?;

            match sessions.get_sessions(&user_id).await {
                Some(sessions) => {
                    self.save_user_sessions(&sessions, &user_dir).await?;
                }
                None => {
                    continue;
                }
            }
        }

        Ok(())
    }

    async fn load(&self) -> Result<(), StorageError> {
        todo!()
    }
}
