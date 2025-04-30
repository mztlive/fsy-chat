use axum::{Json, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

use crate::errors::AppError;

#[derive(Debug, Error)]
pub enum WebError {
    #[error("Session not found")]
    SessionNotFound,

    #[error("Internal server error: {0}")]
    InternalServerError(#[from] AppError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status: i32,
    pub data: T,
    pub message: Option<String>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new(data: T, message: Option<String>) -> Self {
        Self {
            status: 200,
            data,
            message,
        }
    }

    pub fn success(data: T) -> Self {
        Self::new(data, None)
    }
}

impl<T: Default + Serialize> ApiResponse<T> {
    pub fn internal_server_error(message: String) -> Self {
        Self {
            status: 500,
            data: T::default(),
            message: Some(message),
        }
    }

    #[allow(unused)]
    pub fn unauthorized(message: String) -> Self {
        Self {
            status: 401,
            data: T::default(),
            message: Some(message),
        }
    }

    #[allow(unused)]
    pub fn not_permitted(message: String) -> Self {
        Self {
            status: 403,
            data: T::default(),
            message: Some(message),
        }
    }

    pub fn bad_request(message: String) -> Self {
        Self {
            status: 400,
            data: T::default(),
            message: Some(message),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
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

pub type ApiResult<T> = Result<ApiResponse<T>, WebError>;
