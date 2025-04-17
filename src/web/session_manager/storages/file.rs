use std::path::Path;

use tokio::fs::File;

use crate::web::session_manager::Sessions;

use super::storage::{Storage, StorageError};

pub struct FileStorage {
    pub base_path: String,
}

impl FileStorage {
    pub fn new(path: String) -> Self {
        Self { base_path: path }
    }
}

impl Storage for FileStorage {
    async fn persistence(&self, sessions: &Sessions) -> Result<(), StorageError> {
        // 确保目录存在
        let sessions_dir = Path::new(&self.base_path).join("sessions");
        tokio::fs::create_dir_all(&sessions_dir).await?;

        let user_ids = sessions.user_ids().await;

        for user_id in user_ids {
            // 创建文件，文件名是用户ID + .json
            let file_path = sessions_dir.join(format!("{}.json", user_id));
            if !file_path.exists() {
                let mut file = File::create(file_path).await?;

                match sessions.get_sessions(&user_id).await {
                    Some(sessions) => {
                        serde_json::to_writer(&mut file, &sessions).await?;
                    }
                    None => {
                        continue;
                    }
                }
            }
        }

        todo!()
    }

    async fn load(&self) -> Result<(), StorageError> {
        todo!()
    }
}
