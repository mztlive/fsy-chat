use std::time::Duration;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use crate::{
    aliyun::media::schemes::{ImageTaskQueryOutput, Text2ImageTaskItem, Text2ImageTaskUsage},
    web::{
        AppState,
        errors::{ApiResponse, ApiResult, WebError},
    },
};

use super::utils::wait_generation_task;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageGenerationRequest {
    pub prompt: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize)]
pub struct GeneratedImage {
    pub urls: Vec<String>,
    pub timestamp: i64,
    pub actual_prompt: String,
}

fn build_result(results: &Vec<Text2ImageTaskItem>) -> GeneratedImage {
    let mut urls = vec![];
    let mut result_actual_prompt = String::new();

    for result in results {
        match result {
            Text2ImageTaskItem::Success {
                url,
                orig_prompt: _,
                actual_prompt: Some(actual_prompt),
            } => {
                urls.push(url.to_string());
                result_actual_prompt = actual_prompt.to_string();
            }
            _ => {}
        }
    }

    GeneratedImage {
        urls,
        timestamp: chrono::Local::now().timestamp_millis() as i64,
        actual_prompt: result_actual_prompt,
    }
}

pub async fn image_generation(
    State(app_state): State<AppState>,
    Json(request): Json<ImageGenerationRequest>,
) -> ApiResult<GeneratedImage> {
    let task_id = app_state
        .kernel()
        .image_generation_task(&request.prompt, request.width, request.height)
        .await?;

    let response = wait_generation_task::<ImageTaskQueryOutput, Text2ImageTaskUsage>(
        &app_state.kernel(),
        &task_id,
    )
    .await?;

    match response.output.results {
        Some(results) => Ok(ApiResponse::success(build_result(&results))),
        None => Err(WebError::OtherError(
            "Image generation failed: can not get results".to_string(),
        )),
    }
}
