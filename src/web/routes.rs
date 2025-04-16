use axum::{Router, routing::get};
use std::sync::Arc;

use crate::web::{app_state::AppState, handlers::chat_handler::chat_routes};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .nest("/api", chat_routes())
        .with_state(app_state)
}
