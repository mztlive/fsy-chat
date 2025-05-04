/// 阿里云图像生成API模块
/// 实现了与阿里云AI图像生成服务的交互功能
use super::Client;
use rig::image_generation::{
    ImageGenerationError, ImageGenerationRequest, ImageGenerationResponse,
};
use serde::{Deserialize, Serialize};
use std::convert::From;

/// 阿里云图像生成请求体结构
/// 符合阿里云API要求的格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliyunImageGenerationRequest {
    /// 模型名称，例如：wanx2.1-t2i-turbo
    pub model: String,
    /// 输入的基本信息，包含提示词等
    pub input: AliyunImageGenerationInput,
    /// 图像处理参数，可选，包含大小、数量、种子等配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<AliyunImageGenerationParameters>,
}

/// 阿里云图像生成输入结构
/// 包含用于描述生成图像的提示词
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliyunImageGenerationInput {
    /// 正向提示词，用来描述生成图像中期望包含的元素和视觉特点
    pub prompt: String,
    /// 反向提示词，用来描述不希望在画面中看到的内容，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
}

/// 阿里云图像生成参数结构
/// 定义了图像生成的各种可选参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliyunImageGenerationParameters {
    /// 输出图像的分辨率，默认值是1024*1024，格式为"宽*高"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// 生成图片的数量，取值范围为1~4张，默认为4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,
    /// 随机数种子，用于控制模型生成内容的随机性，取值范围是[0, 2147483647]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    /// 是否开启prompt智能改写，默认为true，仅对正向提示词有效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_extend: Option<bool>,
    /// 是否添加水印标识，默认为false，水印位于图片右下角，文案为"AI生成"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<bool>,
}

/// 实现从通用ImageGenerationRequest到AliyunImageGenerationRequest的转换
/// 将通用请求结构映射到阿里云特定格式
impl From<ImageGenerationRequest> for AliyunImageGenerationRequest {
    fn from(request: ImageGenerationRequest) -> Self {
        // 创建默认参数，设置图像尺寸
        let mut parameters = AliyunImageGenerationParameters {
            size: Some(format!("{}*{}", request.width, request.height)),
            n: None,
            seed: None,
            prompt_extend: None,
            watermark: None,
        };

        // 从additional_params解析可选参数
        if let Some(params) = request.additional_params {
            // 尝试提取negative_prompt
            let negative_prompt = params
                .get("negative_prompt")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            // 尝试提取其他参数并设置到parameters
            if let Some(n) = params.get("n").and_then(|v| v.as_u64()) {
                parameters.n = Some(n as u8);
            }

            if let Some(seed) = params.get("seed").and_then(|v| v.as_i64()) {
                parameters.seed = Some(seed as i32);
            }

            if let Some(prompt_extend) = params.get("prompt_extend").and_then(|v| v.as_bool()) {
                parameters.prompt_extend = Some(prompt_extend);
            }

            if let Some(watermark) = params.get("watermark").and_then(|v| v.as_bool()) {
                parameters.watermark = Some(watermark);
            }

            // 创建包含负面提示词的输入
            let input = AliyunImageGenerationInput {
                prompt: request.prompt,
                negative_prompt,
            };

            return AliyunImageGenerationRequest {
                // 使用默认模型或从additional_params中提取
                model: params
                    .get("model")
                    .and_then(|v| v.as_str())
                    .unwrap_or("wanx2.1-t2i-turbo")
                    .to_string(),
                input,
                parameters: Some(parameters),
            };
        }

        // 如果没有additional_params，使用默认值
        AliyunImageGenerationRequest {
            model: "wanx2.1-t2i-turbo".to_string(), // 默认模型
            input: AliyunImageGenerationInput {
                prompt: request.prompt,
                negative_prompt: None,
            },
            parameters: Some(parameters),
        }
    }
}

/// 阿里云API成功响应中的输出部分
/// 包含任务状态和任务ID
#[derive(Debug, Clone, Deserialize)]
pub struct AliyunImageGenerationOutput {
    /// 任务状态，例如："PENDING"
    pub task_status: String,
    /// 任务ID，用于后续查询任务结果
    pub task_id: String,
}

/// 阿里云API成功响应结构
/// 当API调用成功时返回
#[derive(Debug, Clone, Deserialize)]
pub struct AliyunImageGenerationSuccessResponse {
    /// 输出信息，包含任务状态和ID
    pub output: AliyunImageGenerationOutput,
    /// 请求ID，用于追踪和调试
    pub request_id: String,
}

/// 阿里云API错误响应结构
/// 当API调用失败时返回
#[derive(Debug, Clone, Deserialize)]
pub struct AliyunImageGenerationErrorResponse {
    /// 错误码，例如："InvalidApiKey"
    pub code: String,
    /// 错误信息，描述具体错误原因
    pub message: String,
    /// 请求ID，用于追踪和调试
    pub request_id: String,
}

/// 阿里云API响应类型枚举
/// 使用untagged属性，可以根据JSON内容自动选择正确的变体
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AliyunImageGenerationResponse {
    /// 成功响应
    Success(AliyunImageGenerationSuccessResponse),
    /// 错误响应
    Error(AliyunImageGenerationErrorResponse),
}

/// 阿里云图像生成模型
/// 实现rig框架的ImageGenerationModel trait
#[derive(Clone)]
pub struct ImageGenerationModel {
    /// 阿里云API客户端
    client: Client,
    /// 使用的模型名称
    model: String,
}

/// ImageGenerationModel的构造和辅助方法
impl ImageGenerationModel {
    /// 创建新的ImageGenerationModel实例
    ///
    /// # 参数
    /// * `client` - 阿里云API客户端
    /// * `model` - 要使用的模型名称，例如"wanx2.1-t2i-turbo"
    pub fn new(client: Client, model: String) -> Self {
        Self { client, model }
    }
}

/// 实现rig框架的ImageGenerationModel trait
/// 提供标准化的图像生成接口
impl rig::image_generation::ImageGenerationModel for ImageGenerationModel {
    /// 定义响应类型为我们的阿里云响应枚举类型
    type Response = AliyunImageGenerationResponse;

    /// 图像生成方法
    /// 将通用请求转换为阿里云特定请求，调用API并处理响应
    ///
    /// ❗IMPORTANT: 阿里云API的图像生成接口是异步的，
    /// 因此需要后续查询任务结果来获取实际的图像数据。
    ///
    /// # 参数
    /// * `request` - 通用图像生成请求
    ///
    /// # 返回
    /// * 成功 - 包含响应数据的ImageGenerationResponse
    /// * 错误 - 包含错误信息的ImageGenerationError
    async fn image_generation(
        &self,
        request: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse<Self::Response>, ImageGenerationError> {
        // 将通用请求转换为阿里云特定请求
        let mut request = AliyunImageGenerationRequest::from(request);
        // 使用配置的模型名称
        request.model = self.model.clone();

        // 调用阿里云API
        let response = self
            .client
            .post("api/v1/services/aigc/text2image/image-synthesis")
            .header("X-DashScope-Async", "enable") // 启用异步模式
            .json(&request)
            .send()
            .await?;

        // 解析响应
        let body = response.text().await?;
        let response: AliyunImageGenerationResponse = serde_json::from_str(&body)?;

        // 处理不同类型的响应
        match response {
            // 对于成功响应，返回包含响应数据的结果
            // 注意：实际图像数据为空，因为这是异步API，需要后续查询结果
            AliyunImageGenerationResponse::Success(_) => Ok(ImageGenerationResponse {
                image: vec![],
                response,
            }),
            // 对于错误响应，返回包含错误信息的错误
            AliyunImageGenerationResponse::Error(error) => {
                Err(ImageGenerationError::ProviderError(error.message))
            }
        }
    }
}
