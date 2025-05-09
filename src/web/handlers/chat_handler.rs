use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    response::{
        IntoResponse,
        sse::{Event, Sse},
    },
};

use rig::message::Message;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::Infallible;
use std::time::Duration;
use uuid::Uuid;

use crate::{
    session_manager::{SessionHistory, UserID},
    web::{
        app_state::AppState,
        errors::{ApiResponse, ApiResult, WebError},
    },
};

const DEFAULT_USER_ID: &str = "default";

/// 聊天请求结构体
///
/// 包含用户发送的消息内容
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    /// 消息内容
    pub message: String,
}

/// 聊天响应结构体
///
/// 包含会话ID和响应消息
#[derive(Debug, Serialize)]
pub struct ChatResponse {
    /// 会话ID
    pub session_id: String,
    /// 响应消息内容
    pub message: String,
}

/// 创建会话请求查询参数
///
/// 包含可选的文档类别
#[derive(Debug, Deserialize)]
pub struct NewSSEQuery {
    /// 可选的文档类别
    pub category: Option<String>,
}

/// 新会话响应结构体
///
/// 包含创建的会话ID
#[derive(Debug, Serialize)]
pub struct NewSSEResponse {
    /// 会话ID
    pub session_id: String,
}

/// 发送消息处理函数
///
/// 接收用户消息并发送到指定会话，触发AI响应
///
/// # 参数
/// * `app_state` - 应用状态
/// * `session_id` - 会话ID
/// * `request` - 包含消息内容的请求体
///
/// # 返回值
/// 成功则返回空的成功响应，失败则返回错误
pub async fn post_message(
    State(app_state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<ChatRequest>,
) -> ApiResult<()> {
    let mut session = app_state
        .kernel()
        .get_session(&session_id)
        .await
        .ok_or(WebError::SessionNotFound)?;

    let message_id = Uuid::new_v4().to_string();
    session.send_message(&request.message, message_id).await?;

    // 异步总结会话
    tokio::spawn(async move {
        session.do_summary().await;
    });

    Ok(ApiResponse::<()>::success(()))
}

/// 获取会话历史处理函数
///
/// 获取当前用户的所有会话历史记录
///
/// # 参数
/// * `app_state` - 应用状态
/// * `user_id` - 用户ID
///
/// # 返回值
/// 成功则返回会话历史列表，失败则返回错误
pub async fn session_history(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<UserID>,
) -> ApiResult<Vec<SessionHistory>> {
    let session = app_state
        .kernel()
        .sessions()
        .get_session_history(&user_id)
        .await;

    Ok(ApiResponse::success(session))
}

/// 获取消息历史处理函数
///
/// 获取指定会话的所有消息历史
///
/// # 参数
/// * `app_state` - 应用状态
/// * `session_id` - 会话ID
///
/// # 返回值
/// 成功则返回消息历史列表，失败则返回错误
pub async fn message_history(
    State(app_state): State<AppState>,
    Path(session_id): Path<String>,
) -> ApiResult<Vec<Message>> {
    let session = app_state
        .kernel()
        .get_session(&session_id)
        .await
        .ok_or(WebError::SessionNotFound)?;

    Ok(ApiResponse::success(session.get_history().await))
}

/// 创建新会话处理函数
///
/// 创建一个新的聊天会话
///
/// # 参数
/// * `app_state` - 应用状态
/// * `request` - 包含可选文档类别的查询参数
/// * `user_id` - 用户ID
///
/// # 返回值
/// 成功则返回包含会话ID的响应，失败则返回错误
pub async fn create_session(
    State(app_state): State<AppState>,
    Query(request): Query<NewSSEQuery>,
    Extension(user_id): Extension<UserID>,
) -> ApiResult<NewSSEResponse> {
    let (_, session_id) = app_state
        .kernel()
        .create_session(user_id, "你是热心的助手".to_string(), request.category)
        .await?;

    Ok(ApiResponse::success(NewSSEResponse { session_id }))
}

/// 删除会话处理函数
///
/// 删除指定的聊天会话
///
/// # 参数
/// * `app_state` - 应用状态
/// * `session_id` - 要删除的会话ID
/// * `user_id` - 用户ID
///
/// # 返回值
/// 成功则返回空的成功响应，失败则返回错误
pub async fn remove_session(
    State(app_state): State<AppState>,
    Path(session_id): Path<String>,
    Extension(user_id): Extension<UserID>,
) -> ApiResult<()> {
    app_state
        .kernel()
        .remove_session(&user_id, &session_id)
        .await?;

    Ok(ApiResponse::success(()))
}

// SSE处理程序
pub async fn chat_sse_handler(
    State(app_state): State<AppState>,
    Path(session_id): Path<String>,
) -> axum::response::Response<axum::body::Body> {
    let session = app_state.kernel().get_session(&session_id).await;

    match session {
        Some(session) => {
            let mut rx = session.subscribe();

            // 创建从接收端读取消息的Stream
            let stream = async_stream::stream! {
                while let Ok(msg) = rx.recv().await {
                    // 将消息和ID一起发送
                    let response = json!({
                        "id": msg.message_id,
                        "content": msg.message
                    });

                    yield Ok::<_, Infallible>(Event::default().event("new-message").data(response.to_string()));
                }
            };

            Sse::new(stream)
                .keep_alive(
                    axum::response::sse::KeepAlive::new()
                        .interval(Duration::from_secs(1))
                        .text("keep-alive"),
                )
                .into_response()
        }
        None => {
            // 创建一个包含错误信息的流
            let stream = async_stream::stream! {
                let error_response = json!({
                    "error": true,
                    "message": "会话不存在",
                    "code": "SESSION_NOT_FOUND"
                });

                yield Ok::<_, Infallible>(Event::default().event("error").data(error_response.to_string()));
            };

            Sse::new(stream).into_response()
        }
    }
}

pub async fn get_all_document_category(
    State(app_state): State<AppState>,
) -> ApiResult<Vec<String>> {
    Ok(ApiResponse::success(
        app_state.kernel().doc_manager().get_categories().await,
    ))
}
