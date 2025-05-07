use std::time::Duration;

use serde::de::DeserializeOwned;

use crate::{
    aliyun::scheme::{TaskOutput, TaskQueryResponse},
    kernel::Kernel,
    web::errors::WebError,
};

const MAX_QUERY_COUNT: usize = 60;
const QUERY_INTERVAL: Duration = Duration::from_secs(2);

pub(crate) async fn wait_generation_task<O, U>(
    kernel: &Kernel,
    task_id: &str,
) -> Result<TaskQueryResponse<O, U>, WebError>
where
    O: DeserializeOwned + TaskOutput,
    U: DeserializeOwned,
{
    // 每秒查询一次任务状态
    let mut interval = tokio::time::interval(QUERY_INTERVAL);
    let mut timeout = MAX_QUERY_COUNT;
    loop {
        interval.tick().await;
        timeout -= 1;
        let response = kernel.query_generation_task::<O, U>(&task_id).await?;

        if response.output.is_succeeded() {
            return Ok(response);
        }

        if response.output.is_failed() {
            return Err(WebError::OtherError(format!(
                "Image generation failed: {}",
                response.output.error_message()
            )));
        }

        if timeout <= 0 {
            return Err(WebError::OtherError(format!("Image generation timeout",)));
        }
    }
}
