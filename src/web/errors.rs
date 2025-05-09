use axum::{Json, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

use crate::errors::AppError;

/// Web错误枚举
///
/// 定义了Web服务中可能出现的各种错误类型
#[derive(Debug, Error)]
pub enum WebError {
    /// 会话不存在错误
    #[error("Session not found")]
    SessionNotFound,

    /// 内部服务器错误，包装自应用核心错误
    #[error("Internal server error: {0}")]
    InternalServerError(#[from] AppError),

    /// IO错误，如文件读写失败
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// 其他类型的错误
    #[error("Other error: {0}")]
    OtherError(String),
}

/// API响应结构体
///
/// 提供统一的API响应格式，包含状态码、数据和消息
#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    /// 状态码，成功为200，错误根据类型设置不同值
    pub status: i32,
    /// 响应数据
    pub data: T,
    /// 可选的消息，通常在错误时提供错误描述
    pub message: Option<String>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// 创建新的API响应
    ///
    /// # 参数
    /// * `data` - 响应数据
    /// * `message` - 可选的响应消息
    ///
    /// # 返回值
    /// 返回包含指定数据和消息的API响应
    pub fn new(data: T, message: Option<String>) -> Self {
        Self {
            status: 200,
            data,
            message,
        }
    }

    /// 创建成功的API响应
    ///
    /// # 参数
    /// * `data` - 响应数据
    ///
    /// # 返回值
    /// 返回包含指定数据的成功响应，状态码为200
    pub fn success(data: T) -> Self {
        Self::new(data, None)
    }
}

impl<T: Default + Serialize> ApiResponse<T> {
    /// 创建内部服务器错误响应
    ///
    /// # 参数
    /// * `message` - 错误消息
    ///
    /// # 返回值
    /// 返回包含错误消息的服务器错误响应，状态码为500
    pub fn internal_server_error(message: String) -> Self {
        Self {
            status: 500,
            data: T::default(),
            message: Some(message),
        }
    }

    /// 创建未授权错误响应
    ///
    /// # 参数
    /// * `message` - 错误消息
    ///
    /// # 返回值
    /// 返回包含错误消息的未授权响应，状态码为401
    #[allow(unused)]
    pub fn unauthorized(message: String) -> Self {
        Self {
            status: 401,
            data: T::default(),
            message: Some(message),
        }
    }

    /// 创建禁止访问错误响应
    ///
    /// # 参数
    /// * `message` - 错误消息
    ///
    /// # 返回值
    /// 返回包含错误消息的禁止访问响应，状态码为403
    #[allow(unused)]
    pub fn not_permitted(message: String) -> Self {
        Self {
            status: 403,
            data: T::default(),
            message: Some(message),
        }
    }

    /// 创建请求参数错误响应
    ///
    /// # 参数
    /// * `message` - 错误消息
    ///
    /// # 返回值
    /// 返回包含错误消息的请求参数错误响应，状态码为400
    pub fn bad_request(message: String) -> Self {
        Self {
            status: 400,
            data: T::default(),
            message: Some(message),
        }
    }
}

/// ApiResult类型别名
///
/// 为常用的API结果类型提供简写
pub type ApiResult<T> = Result<ApiResponse<T>, WebError>;

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    /// 将API响应转换为HTTP响应
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        let api_response = match self {
            Self::InternalServerError(e) => ApiResponse::<()>::internal_server_error(e.to_string()),
            _ => ApiResponse::<()>::bad_request(self.to_string()),
        };

        api_response.into_response()
    }
}
