use rig::image_generation::ImageGenerationRequest;
use serde::{Deserialize, Serialize};

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

/// 阿里云图像生成任务查询结果项
/// 成功时包含图像URL，失败时包含错误信息
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AliyunTaskResultItem {
    /// 成功生成的图像
    Success {
        /// 生成图像的URL
        url: String,
        /// 原始提示词，可选
        #[serde(skip_serializing_if = "Option::is_none")]
        orig_prompt: Option<String>,
        /// 实际使用的提示词，可选（智能改写后）
        #[serde(skip_serializing_if = "Option::is_none")]
        actual_prompt: Option<String>,
    },
    /// 生成失败的错误信息
    Error {
        /// 错误码
        code: String,
        /// 错误信息
        message: String,
    },
}

/// 任务指标统计
#[derive(Debug, Clone, Deserialize)]
pub struct AliyunTaskMetrics {
    /// 总任务数
    pub TOTAL: u32,
    /// 成功完成的任务数
    pub SUCCEEDED: u32,
    /// 失败的任务数
    pub FAILED: u32,
}

/// 资源使用统计
#[derive(Debug, Clone, Deserialize)]
pub struct AliyunUsage {
    /// 生成的图像数量
    pub image_count: u32,
}

/// 任务查询成功输出
#[derive(Debug, Clone, Deserialize)]
pub struct AliyunTaskQueryOutput {
    /// 任务ID
    pub task_id: String,
    /// 任务状态，如"SUCCEEDED"或"FAILED"
    pub task_status: String,
    /// 任务提交时间，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    /// 任务调度时间，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time: Option<String>,
    /// 任务结束时间，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 任务结果，成功时包含图像URL和提示词信息，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<AliyunTaskResultItem>>,
    /// 错误码，任务失败时存在
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息，任务失败时存在
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 任务指标统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_metrics: Option<AliyunTaskMetrics>,
}

/// 阿里云图像生成任务查询响应
#[derive(Debug, Clone, Deserialize)]
pub struct AliyunTaskQueryResponse {
    /// 请求ID
    pub request_id: String,
    /// 输出信息，包含任务状态和结果
    pub output: AliyunTaskQueryOutput,
    /// 资源使用统计，可选（任务成功时存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<AliyunUsage>,
}
