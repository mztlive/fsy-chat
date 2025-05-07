use serde::{Deserialize, Serialize};

pub trait TaskOutput {
    fn is_succeeded(&self) -> bool;

    fn error_message(&self) -> String;

    fn is_failed(&self) -> bool;
}

/// 阿里云图像生成任务查询响应
#[derive(Debug, Clone, Deserialize)]
pub struct TaskQueryResponse<O: TaskOutput, U> {
    /// 请求ID
    pub request_id: String,
    /// 输出信息，包含任务状态和结果
    pub output: O,
    /// 资源使用统计，可选（任务成功时存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<U>,
}

/// 阿里云生成请求体结构
/// 符合阿里云API要求的格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest<I, P> {
    /// 模型名称，例如：wanx2.1-t2i-turbo
    pub model: String,
    /// 输入的基本信息，不同接口、模型不同
    pub input: I,
    /// 额外处理参数，可选，包含大小、数量、种子等配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<P>,
}

/// 阿里云API成功响应中的输出部分
/// 包含任务状态和任务ID
#[derive(Debug, Clone, Deserialize)]
pub struct AsyncGenerationOutput {
    /// 任务状态，例如："PENDING"
    pub task_status: String,
    /// 任务ID，用于后续查询任务结果
    pub task_id: String,
}

/// 阿里云API响应类型枚举
/// 使用untagged属性，可以根据JSON内容自动选择正确的变体
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AsyncImageGenerationResponse {
    /// 成功响应
    Success(AsyncGenerationSuccessResponse),
    /// 错误响应
    Error(AsyncGenerationErrorResponse),
}

/// 阿里云API成功响应结构
/// 当API调用成功时返回
#[derive(Debug, Clone, Deserialize)]
pub struct AsyncGenerationSuccessResponse {
    /// 输出信息，包含任务状态和ID
    pub output: AsyncGenerationOutput,
    /// 请求ID，用于追踪和调试
    pub request_id: String,
}

/// 阿里云API错误响应结构
/// 当API调用失败时返回
#[derive(Debug, Clone, Deserialize)]
pub struct AsyncGenerationErrorResponse {
    /// 错误码，例如："InvalidApiKey"
    pub code: String,
    /// 错误信息，描述具体错误原因
    pub message: String,
    /// 请求ID，用于追踪和调试
    pub request_id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AliyunError {
    /// 请求失败
    #[error("request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    /// 解析失败
    #[error("parse failed: {0}")]
    ParseFailed(#[from] serde_json::Error),
    /// 阿里云API错误
    #[error("aliyun api error: {0}")]
    ApiError(String),
}
