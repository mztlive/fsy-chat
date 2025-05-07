use rig::image_generation::ImageGenerationRequest;
use serde::{Deserialize, Serialize};

use crate::aliyun::scheme::{GenerationRequest, TaskOutput};

const DEFAULT_TEXT2VIDEO_MODEL: &str = "wanx2.1-t2v-turbo";

const DEFAULT_TEXT2IMAGE_MODEL: &str = "wanx2.1-t2i-turbo";

/// 阿里云图像生成输入结构
/// 包含用于描述生成图像的提示词
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenerationInput {
    /// 正向提示词，用来描述生成图像中期望包含的元素和视觉特点
    pub prompt: String,
    /// 反向提示词，用来描述不希望在画面中看到的内容，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
}

/// 阿里云图像生成参数结构
/// 定义了图像生成的各种可选参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenerationParameters {
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
impl From<ImageGenerationRequest>
    for GenerationRequest<ImageGenerationInput, ImageGenerationParameters>
{
    fn from(request: ImageGenerationRequest) -> Self {
        // 创建默认参数，设置图像尺寸
        let mut parameters = ImageGenerationParameters {
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
            let input = ImageGenerationInput {
                prompt: request.prompt,
                negative_prompt,
            };

            return GenerationRequest {
                // 使用默认模型或从additional_params中提取
                model: params
                    .get("model")
                    .and_then(|v| v.as_str())
                    .unwrap_or(DEFAULT_TEXT2IMAGE_MODEL)
                    .to_string(),
                input,
                parameters: Some(parameters),
            };
        }

        // 如果没有additional_params，使用默认值
        GenerationRequest {
            model: DEFAULT_TEXT2IMAGE_MODEL.to_string(), // 默认模型
            input: ImageGenerationInput {
                prompt: request.prompt,
                negative_prompt: None,
            },
            parameters: Some(parameters),
        }
    }
}

/// 阿里云图像生成任务查询结果项
/// 成功时包含图像URL，失败时包含错误信息
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Text2ImageTaskItem {
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
pub struct Text2ImageTaskMetrics {
    /// 总任务数
    pub TOTAL: u32,
    /// 成功完成的任务数
    pub SUCCEEDED: u32,
    /// 失败的任务数
    pub FAILED: u32,
}

/// 资源使用统计
#[derive(Debug, Clone, Deserialize)]
pub struct Text2ImageTaskUsage {
    /// 生成的图像数量
    pub image_count: u32,
}

/// 任务查询成功输出
#[derive(Debug, Clone, Deserialize)]
pub struct ImageTaskQueryOutput {
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
    pub results: Option<Vec<Text2ImageTaskItem>>,
    /// 错误码，任务失败时存在
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息，任务失败时存在
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 任务指标统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_metrics: Option<Text2ImageTaskMetrics>,
}

impl TaskOutput for ImageTaskQueryOutput {
    fn is_succeeded(&self) -> bool {
        self.task_status == "SUCCEEDED"
    }

    fn is_failed(&self) -> bool {
        self.task_status == "FAILED"
    }

    fn error_message(&self) -> String {
        self.message.clone().unwrap_or_default()
    }
}

/// 阿里云文生视频输入结构
/// 阿里云文生视频输入结构
/// 包含用于描述生成视频的提示词
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text2VideoInput {
    /// 文本提示词，支持中英文，长度不超过800个字符
    pub prompt: String,
}

/// 阿里云文生视频参数结构
/// 定义了视频生成的各种可选参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text2VideoParameters {
    /// 生成视频的分辨率，默认值1280*720，格式为"宽*高"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// 生成视频的时长，默认为5，单位为秒，目前仅支持5秒固定时长生成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    /// 是否开启prompt智能改写，默认为true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_extend: Option<bool>,
    /// 随机数种子，用于控制模型生成内容的随机性，取值范围是[0, 2147483647]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text2VideoGenerationRequest {
    pub input: Text2VideoInput,
    pub parameters: Text2VideoParameters,
}

impl From<Text2VideoGenerationRequest>
    for GenerationRequest<Text2VideoInput, Text2VideoParameters>
{
    fn from(request: Text2VideoGenerationRequest) -> Self {
        GenerationRequest {
            model: DEFAULT_TEXT2VIDEO_MODEL.to_string(),
            input: request.input,
            parameters: Some(request.parameters),
        }
    }
}

/// 阿里云文生视频任务统计信息
#[derive(Debug, Clone, Deserialize)]
pub struct Text2VideoTaskUsage {
    /// 生成视频的时长，单位秒
    pub video_duration: u32,
    /// 生成视频的比例，固定为standard
    pub video_ratio: String,
    /// 生成视频的数量
    pub video_count: u32,
}

/// 阿里云文生视频任务查询结果
/// 包含任务状态、视频URL、提示词信息等
#[derive(Debug, Clone, Deserialize)]
pub struct Text2VideoTaskQueryOutput {
    /// 任务ID
    pub task_id: String,
    /// 任务状态：PENDING(排队中)、RUNNING(处理中)、SUCCEEDED(成功)、FAILED(失败)、CANCELED(取消)、UNKNOWN(未知)
    pub task_status: String,
    /// 任务提交时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    /// 任务调度时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time: Option<String>,
    /// 任务结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 生成视频的URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
    /// 原始提示词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_prompt: Option<String>,
    /// 实际使用的提示词（智能改写后）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_prompt: Option<String>,
    /// 错误码，任务失败时存在
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息，任务失败时存在
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl TaskOutput for Text2VideoTaskQueryOutput {
    fn is_succeeded(&self) -> bool {
        self.task_status == "SUCCEEDED"
    }

    fn is_failed(&self) -> bool {
        self.task_status == "FAILED"
    }

    fn error_message(&self) -> String {
        self.message.clone().unwrap_or_default()
    }
}
