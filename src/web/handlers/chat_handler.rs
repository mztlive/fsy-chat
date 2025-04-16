use axum::{
    Json, Router,
    extract::{Path, Query, State},
    response::{
        IntoResponse,
        sse::{Event, Sse},
    },
    routing::{get, post},
};

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::Infallible;
use std::time::Duration;
use uuid::Uuid;

use crate::web::{
    app_state::AppState,
    errors::{ApiResponse, ApiResult, WebError},
};

// 请求体结构
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

// 响应体结构
#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub session_id: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct NewSSEQuery {
    pub category: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NewSSEResponse {
    pub session_id: String,
}

pub async fn post_message(
    State(app_state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<ChatRequest>,
) -> ApiResult<()> {
    let mut session = app_state
        .chat_session_manager
        .get_session(&session_id)
        .await
        .ok_or(WebError::SessionNotFound)?;

    let message_id = Uuid::new_v4().to_string();
    session.send_message(&request.message, message_id).await?;

    Ok(ApiResponse::<()>::success(()))
}

pub async fn create_session(
    State(app_state): State<AppState>,
    Query(request): Query<NewSSEQuery>,
) -> ApiResult<NewSSEResponse> {
    let (_, session_id) = app_state
        .chat_session_manager
        .create_session(request.category)
        .await?;

    Ok(ApiResponse::success(NewSSEResponse { session_id }))
}

// SSE处理程序
pub async fn chat_sse_handler(
    State(app_state): State<AppState>,
    Path(session_id): Path<String>,
) -> axum::response::Response<axum::body::Body> {
    // 创建新会话，使用match或if let来处理错误
    let session = app_state
        .chat_session_manager
        .get_session(&session_id)
        .await;

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
        None => ApiResponse::<()>::bad_request("Session not found".to_string()).into_response(),
    }
}

pub async fn get_all_document_category(
    State(app_state): State<AppState>,
) -> ApiResult<Vec<String>> {
    Ok(ApiResponse::success(
        app_state.doc_manager.get_categories().await,
    ))
}

// 设置路由
pub fn chat_routes() -> Router<AppState> {
    Router::new()
        .route("/chat/sse/{session_id}", get(chat_sse_handler))
        .route("/chat/message/{session_id}", post(post_message))
        .route("/chat/create", get(create_session))
        .route("/all/document/category", get(get_all_document_category))
}
