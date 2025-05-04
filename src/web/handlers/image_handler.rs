use std::time::Duration;

use axum::{Json, Router, extract::State, routing::post};
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

pub async fn image_generation(
    State(app_state): State<AppState>,
    Json(request): Json<ImageGenerationRequest>,
) -> ApiResult<String> {
    let task_id = app_state
        .kernel()
        .image_generation_task(&request.prompt)
        .await?;

    // 每秒查询一次任务状态
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut timeout = 30;
    loop {
        interval.tick().await;
        timeout -= 1;
        let response = app_state
            .kernel()
            .query_image_generation_task(&task_id)
            .await?;

        if response.output.task_status == "SUCCEEDED" {
            if let Some(results) = response.output.results {
                if let Some(result) = results.first() {
                    match result {
                        AliyunTaskResultItem::Success { url, .. } => {
                            return Ok(ApiResponse::success(url.clone()));
                        }
                        _ => {}
                    }
                }
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
