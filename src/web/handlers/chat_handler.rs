use axum::{
    Json, Router,
    extract::{Path, State},
    response::{
        IntoResponse,
        sse::{Event, Sse},
    },
    routing::{get, post},
};
use futures_util::FutureExt;
use futures_util::stream::RepeatWith;
use futures_util::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::{Mutex, mpsc};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use crate::{
    agent::{AgentConfig, EmbeddingConfig},
    chat::{ChatSession, ResponseCallback},
    document_loader::DocumentManager,
    errors::AppResult,
    web::{
        app_state::AppState,
        errors::{ApiResponse, ApiResult, WebError},
    },
};

// 请求体结构
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub category: Option<String>,
}

// 响应体结构
#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub session_id: String,
    pub message: String,
}

pub async fn post_message(
    State(app_state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<ChatRequest>,
) -> ApiResult<ChatResponse> {
    todo!()
}

// SSE处理程序
pub async fn chat_sse_handler(
    Path(session_id): Path<String>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    println!("`{}` connected", session_id);

    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok::<Event, Infallible>)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

// 设置路由
pub fn chat_routes() -> Router<AppState> {
    Router::new().route("/chat/:session_id", get(chat_sse_handler))
}
