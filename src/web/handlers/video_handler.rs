// pub async fn video_generation(
//     State(app_state): State<AppState>,
//     Json(request): Json<VideoGenerationRequest>,
// ) -> ApiResult<GeneratedVideo> {
//     let task_id = app_state
//         .kernel()
//         .video_generation_task(&request.prompt)
//         .await?;

//     let response = wait_generation_task::<VideoTaskQueryOutput, Text2VideoTaskUsage>(
//         &app_state.kernel(),
//         &task_id,
//     )
//     .await?;

//     match response.output.results {
//         Some(results) => Ok(ApiResponse::success(build_result(&results))),
//         None => Err(WebError::OtherError(
//             "Image generation failed: can not get results".to_string(),
//         )),
//     }
// }
