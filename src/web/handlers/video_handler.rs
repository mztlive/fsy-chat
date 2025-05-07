use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use crate::{
    aliyun::media::schemes::{Text2VideoTaskQueryOutput, Text2VideoTaskUsage},
    web::{
        AppState,
        errors::{ApiResponse, ApiResult},
    },
};

use super::utils::wait_generation_task;

#[derive(Debug, Deserialize)]
pub struct VideoGenerationRequest {
    pub prompt: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize)]
pub struct GeneratedVideo {
    pub url: String,
    pub timestamp: i64,
    pub actual_prompt: String,
}

pub async fn video_generation(
    State(app_state): State<AppState>,
    Json(request): Json<VideoGenerationRequest>,
) -> ApiResult<GeneratedVideo> {
    let task_id = app_state
        .kernel()
        .video_generation_task(&request.prompt, request.width, request.height)
        .await?;

    let response = wait_generation_task::<Text2VideoTaskQueryOutput, Text2VideoTaskUsage>(
        &app_state.kernel(),
        &task_id,
    )
    .await?;

    Ok(ApiResponse::success(GeneratedVideo {
        url: response.output.video_url.unwrap_or_default(),
        timestamp: chrono::Local::now().timestamp_millis() as i64,
        actual_prompt: response.output.actual_prompt.unwrap_or_default(),
    }))
}
