use rig::{
    agent::Agent,
    image_generation::{ImageGenerationModel, ImageGenerationRequest},
    providers::openai::{self, Client as OpenAiClient},
};
use serde::de::DeserializeOwned;

use crate::{
    aliyun::{
        client::Client as AliyunClient,
        media::schemes::{Text2VideoGenerationRequest, Text2VideoInput, Text2VideoParameters},
        scheme::{TaskOutput, TaskQueryResponse},
    },
    chat::{ChatSession, ChatSessionView},
    config::Config,
    document_loader::DocumentManager,
    errors::AppResult,
    session_manager::{Sessions, UserID},
    vector_store::VectorStoreManager,
};

/// 应用程序核心组件，协调各模块功能
///
/// Kernel是应用程序的中央控制器，负责初始化和协调各个组件，
/// 管理配置、文档、会话和向量存储。它提供了创建和管理聊天会话
/// 的接口，是应用程序的核心骨架。
#[derive(Clone)]
pub struct Kernel {
    config: Config,
    doc_manager: DocumentManager,
    client: OpenAiClient,
    aliyun_client: AliyunClient,
    vector_store_manager: VectorStoreManager,
    sessions: Sessions<openai::CompletionModel>,
}

impl Kernel {
    /// 创建OpenAI兼容客户端
    ///
    /// 创建一个指向阿里云DashScope兼容OpenAI接口的客户端
    ///
    /// # 参数
    /// * `api_key` - API密钥
    ///
    /// # 返回值
    /// 返回配置好的OpenAI客户端
    fn create_client(api_key: &str) -> openai::Client {
        openai::Client::from_url(api_key, "https://dashscope.aliyuncs.com/compatible-mode/v1")
    }

    /// 初始化文档管理器
    ///
    /// 根据配置加载各类别的文档
    ///
    /// # 参数
    /// * `config` - 应用程序配置
    ///
    /// # 返回值
    /// 返回初始化的文档管理器或错误
    async fn initialize_document_manager(config: &Config) -> AppResult<DocumentManager> {
        let mut manager = DocumentManager::new();

        for category in &config.document.categories {
            manager
                .load_category(category.clone(), &category.directory)
                .await?;
        }

        Ok(manager)
    }

    /// 创建新的Kernel实例
    ///
    /// 初始化应用程序的核心组件，包括文档管理器和向量存储
    ///
    /// # 参数
    /// * `config` - 应用程序配置
    /// * `client` - OpenAI兼容API客户端
    ///
    /// # 返回值
    /// 返回初始化的Kernel实例
    pub async fn new(config: Config) -> Self {
        let client = Self::create_client(&config.client.api_key);
        let aliyun_client = AliyunClient::new(&config.embedding.api_key);

        let doc_manager = Self::initialize_document_manager(&config)
            .await
            .expect("Can not initialize document manager");

        let embedding_model = aliyun_client.embedding_model_with_ndims(
            &config.embedding.model,
            config.embedding.dimensions as usize,
        );

        let store_manager = VectorStoreManager::from_documents(&doc_manager, embedding_model)
            .await
            .expect("Can not initialize vector store manager");

        Self {
            config,
            doc_manager,
            client,
            aliyun_client,
            vector_store_manager: store_manager,
            sessions: Sessions::new(),
        }
    }

    /// 获取文档管理器
    pub fn doc_manager(&self) -> &DocumentManager {
        &self.doc_manager
    }

    /// 创建一个新的AI代理
    ///
    /// 根据指定的前置指令和文档类别创建代理实例
    /// 如果指定了文档类别，会自动关联相应的向量存储以支持RAG功能
    ///
    /// # 参数
    /// * `preamble` - 代理前置指令
    /// * `doc_category` - 可选的文档类别名称
    ///
    /// # 返回值
    /// 返回配置好的AI代理实例
    pub async fn create_agent(
        &self,
        preamble: &str,
        doc_category: Option<&str>,
    ) -> Agent<openai::CompletionModel> {
        let mut builder = self
            .client
            .agent(&self.config.client.chat_model)
            .preamble(preamble);

        let embedding_model = self.aliyun_client.embedding_model_with_ndims(
            &self.config.embedding.model,
            self.config.embedding.dimensions as usize,
        );

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

    /// 从会话视图恢复聊天会话
    ///
    /// 使用会话视图对象重建完整的聊天会话并添加到会话管理器中
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `session_id` - 会话ID
    /// * `chat_view` - 会话视图对象，包含会话的序列化数据
    ///
    /// # 返回值
    /// 成功则返回Ok，否则返回错误
    pub async fn recovery_chatview(
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

    /// 创建新的聊天会话
    ///
    /// 使用指定的前置指令和可选的文档类别创建一个新会话
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `preamble` - 会话前置指令
    /// * `doc_category` - 可选的文档类别
    ///
    /// # 返回值
    /// 成功则返回会话实例和会话ID，否则返回错误
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

    /// 获取指定ID的会话
    ///
    /// # 参数
    /// * `session_id` - 会话ID
    ///
    /// # 返回值
    /// 如果会话存在则返回会话实例，否则返回None
    pub async fn get_session(
        &self,
        session_id: &str,
    ) -> Option<ChatSession<openai::CompletionModel>> {
        self.sessions.get_session(session_id).await
    }

    /// 获取会话管理器
    pub fn sessions(&self) -> &Sessions<openai::CompletionModel> {
        &self.sessions
    }

    /// 删除指定的会话
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `session_id` - 会话ID
    ///
    /// # 返回值
    /// 操作成功则返回Ok，否则返回错误
    pub async fn remove_session(&self, user_id: &UserID, session_id: &str) -> AppResult<()> {
        self.sessions.remove_session(user_id, session_id).await;
        Ok(())
    }
}

// impl for image generation
impl Kernel {
    /// 生成图像
    ///
    /// # 参数
    /// * `prompt` - 图像生成提示
    ///
    /// # 返回值
    /// 图像任务的ID
    pub async fn image_generation_task(
        &self,
        prompt: &str,
        width: u32,
        height: u32,
    ) -> AppResult<String> {
        let model = self
            .aliyun_client
            .image_generation_model(&self.config.image.model);

        let request = ImageGenerationRequest {
            prompt: prompt.to_string(),
            width,
            height,
            additional_params: None,
        };

        let response = model.image_generation(request).await?;

        Ok(response.response.task_id)
    }

    /// 生成视频
    ///
    /// # 参数
    /// * `prompt` - 视频生成提示
    ///
    /// # 返回值
    /// 视频任务的ID
    pub async fn video_generation_task(
        &self,
        prompt: &str,
        width: u32,
        height: u32,
    ) -> AppResult<String> {
        let model = self
            .aliyun_client
            .video_generation_model(&self.config.video.model);

        let request = Text2VideoGenerationRequest {
            input: Text2VideoInput {
                prompt: prompt.to_string(),
            },
            parameters: Text2VideoParameters {
                size: Some(format!("{}*{}", width, height)),
                duration: Some(5),
                prompt_extend: Some(true),
                seed: None,
            },
        };

        let response = model.create_task(request).await?;

        Ok(response.task_id)
    }

    /// 查询生成任务的结果
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    ///
    /// # 返回值
    /// 任务查询结果
    pub async fn query_generation_task<O, U>(
        &self,
        task_id: &str,
    ) -> AppResult<TaskQueryResponse<O, U>>
    where
        O: TaskOutput + DeserializeOwned,
        U: DeserializeOwned,
    {
        Ok(self.aliyun_client.query_task(task_id).await?)
    }
}
