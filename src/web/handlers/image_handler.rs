use std::time::Duration;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use crate::{
    aliyun::image::schemes::AliyunTaskResultItem,
    web::{
        AppState,
        errors::{ApiResponse, ApiResult, WebError},
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageGenerationRequest {
    pub prompt: String,
}

#[derive(Debug, Serialize)]
pub struct GeneratedImage {
    pub urls: Vec<String>,
    pub timestamp: i64,
    pub actual_prompt: String,
}

const MAX_QUERY_COUNT: usize = 60;
const QUERY_INTERVAL: Duration = Duration::from_secs(2);

fn build_result(results: &Vec<AliyunTaskResultItem>) -> GeneratedImage {
    let mut urls = vec![];
    let mut result_actual_prompt = String::new();

    for result in results {
        match result {
            AliyunTaskResultItem::Success {
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
        .image_generation_task(&request.prompt)
        .await?;

    // 每秒查询一次任务状态
    let mut interval = tokio::time::interval(QUERY_INTERVAL);
    let mut timeout = MAX_QUERY_COUNT;
    loop {
        interval.tick().await;
        timeout -= 1;
        let response = app_state
            .kernel()
            .query_image_generation_task(&task_id)
            .await?;

        if response.output.task_status == "SUCCEEDED" {
            if let Some(results) = response.output.results {
                return Ok(ApiResponse::success(build_result(&results)));
            }

            return Err(WebError::OtherError(format!(
                "Image generation failed: {}",
                response.output.message.unwrap_or_default()
            )));
        }

        if response.output.task_status == "FAILED" {
            return Err(WebError::OtherError(format!(
                "Image generation failed: {}",
                response.output.message.unwrap_or_default()
            )));
        }

        if timeout <= 0 {
            return Err(WebError::OtherError(format!(
                "Image generation timeout: {}",
                response.output.message.unwrap_or_default()
            )));
        }
    }
}
