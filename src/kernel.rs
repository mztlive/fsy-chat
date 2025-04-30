use rig::{
    agent::Agent,
    providers::openai::{self, Client},
};

use crate::{
    chat::{ChatSession, ChatSessionView},
    config::Config,
    document_loader::DocumentManager,
    errors::AppResult,
    session_manager::{Sessions, UserID},
    vector_store::VectorStoreManager,
};

#[derive(Clone)]
pub struct Kernel {
    config: Config,
    doc_manager: DocumentManager,
    client: Client,
    vector_store_manager: VectorStoreManager,
    sessions: Sessions<openai::CompletionModel>,
}

impl Kernel {
    async fn initialize_document_manager(config: &Config) -> AppResult<DocumentManager> {
        let mut manager = DocumentManager::new();

        for category in &config.document.categories {
            manager
                .load_category(category.clone(), &category.directory)
                .await?;
        }

        Ok(manager)
    }

    pub async fn new(config: Config, client: Client) -> Self {
        let doc_manager = Self::initialize_document_manager(&config)
            .await
            .expect("Can not initialize document manager");

        let embedding_model = client.embedding_model(&config.embedding.model);

        let store_manager = VectorStoreManager::from_documents(&doc_manager, embedding_model)
            .await
            .expect("Can not initialize vector store manager");

        Self {
            config,
            doc_manager,
            client,
            vector_store_manager: store_manager,
            sessions: Sessions::new(),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn doc_manager(&self) -> &DocumentManager {
        &self.doc_manager
    }

    pub async fn create_agent(
        &self,
        preamble: &str,
        doc_category: Option<&str>,
    ) -> Agent<openai::CompletionModel> {
        let mut builder = self
            .client
            .agent(&self.config.agent.chat_model)
            .preamble(preamble);

        let embedding_model = self.client.embedding_model(&self.config.embedding.model);

        if let Some(doc_category) = doc_category {
            match self.vector_store_manager.find_store(doc_category).await {
                Some(store) => {
                    builder = builder.dynamic_context(5, store.index(embedding_model));
                }
                None => {}
            }
        }

        builder.build()
    }

    pub async fn add_history(
        &self,
        user_id: UserID,
        session_id: String,
        chat_view: ChatSessionView,
    ) -> AppResult<()> {
        let agent = self
            .create_agent(&chat_view.preamble, chat_view.doc_category.as_deref())
            .await;

        let chat_session = ChatSession::from_view(chat_view, agent).await?;

        self.sessions
            .add_session(user_id, session_id, chat_session)
            .await;

        Ok(())
    }

    // 获取或创建会话
    pub async fn create_session(
        &self,
        user_id: UserID,
        preamble: String,
        doc_category: Option<String>,
    ) -> AppResult<(ChatSession<openai::CompletionModel>, String)> {
        let agent = self.create_agent(&preamble, doc_category.as_deref()).await;

        let session_id = uuid::Uuid::new_v4().to_string();

        // 创建新会话
        let session = ChatSession::new(agent, preamble, doc_category).await?;

        self.sessions
            .add_session(user_id, session_id.clone(), session.clone())
            .await;

        Ok((session, session_id))
    }

    pub async fn get_session(
        &self,
        session_id: &str,
    ) -> Option<ChatSession<openai::CompletionModel>> {
        self.sessions.get_session(session_id).await
    }

    pub fn sessions(&self) -> &Sessions<openai::CompletionModel> {
        &self.sessions
    }

    pub async fn remove_session(&self, user_id: &UserID, session_id: &str) -> AppResult<()> {
        self.sessions.remove_session(user_id, session_id).await;
        Ok(())
    }
}
