use rig::{
    agent::Agent,
    providers::openai::{self, Client},
};

use crate::{
    config::Config, document_loader::DocumentManager, errors::AppResult,
    session_manager::ChatSessionManager, vector_store::VectorStoreManager,
};

#[derive(Clone)]
pub struct Kernel {
    config: Config,
    doc_manager: DocumentManager,
    client: Client,
    vector_store_manager: VectorStoreManager,
    chat_session_manager: ChatSessionManager<openai::CompletionModel>,
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
            chat_session_manager: ChatSessionManager::new(),
        }
    }

    pub fn session_manager(&self) -> &ChatSessionManager<openai::CompletionModel> {
        &self.chat_session_manager
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
}
