use std::time::Duration;

use axum::BoxError;
use axum::error_handling::HandleErrorLayer;
use axum::http::header;
use axum::http::{HeaderName, Method};
use axum::response::IntoResponse;
use axum::{Router, extract::DefaultBodyLimit, http::StatusCode};
use tower::{ServiceBuilder, timeout::TimeoutLayer};
use tower_http::cors::CorsLayer;

use crate::web::{app_state::AppState, handlers::chat_handler::chat_routes};

use super::errors::ApiResponse;

pub fn create_router(app_state: AppState) -> Router {
    // 配置允许的源
    let origins = [
        "http://localhost:3000".parse().unwrap(),
        "http://127.0.0.1:3000".parse().unwrap(),
    ];

    Router::new()
        .nest("/api", chat_routes())
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
                        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
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
}
