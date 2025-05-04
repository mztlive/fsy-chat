use std::time::Duration;

use axum::BoxError;
use axum::error_handling::HandleErrorLayer;
use axum::http::Method;
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::{Router, extract::DefaultBodyLimit};
use tower::{ServiceBuilder, timeout::TimeoutLayer};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::web::app_state::AppState;

use super::errors::ApiResponse;
use super::handlers::chat_handler::chat_sse_handler;
use super::handlers::chat_handler::create_session;
use super::handlers::chat_handler::get_all_document_category;
use super::handlers::chat_handler::message_history;
use super::handlers::chat_handler::post_message;
use super::handlers::chat_handler::remove_session;
use super::handlers::chat_handler::session_history;
use super::handlers::image_handler::image_generation;

// 设置路由
pub fn app_routes() -> Router<AppState> {
    Router::new()
        .route("/chat/sse/{session_id}", get(chat_sse_handler))
        .route("/chat/message/{session_id}", post(post_message))
        .route("/chat/create", get(create_session))
        .route("/all/document/category", get(get_all_document_category))
        .route("/session/history", get(session_history))
        .route("/message/history/{session_id}", get(message_history))
        .route("/session/{session_id}", delete(remove_session))
        .route("/image/generation", post(image_generation))
}

pub fn create_router(app_state: AppState) -> Router {
    info!("创建Web路由");
    // 配置允许的源
    let origins = [
        "http://localhost:3000".parse().unwrap(),
        "http://127.0.0.1:3000".parse().unwrap(),
    ];

    Router::new()
        .nest("/api", app_routes())
        .with_state(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|e: BoxError| {
                    let e_string = e.to_string();
                    async move {
                        ApiResponse::<()>::internal_server_error(e_string).into_response()
                    }
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(300)))
                .layer(
                    CorsLayer::new()
                        .allow_origin(origins)
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::OPTIONS,
                            Method::DELETE,
                            Method::PUT,
                        ])
                        .allow_headers([
                            header::AUTHORIZATION,
                            header::ACCEPT,
                            header::CONTENT_TYPE,
                            header::ORIGIN,
                        ])
                        .allow_credentials(true),
                ),
        )
        .layer(DefaultBodyLimit::max(300 * 1024 * 1024))
        .layer(TraceLayer::new_for_http())
}
