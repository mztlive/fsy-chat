/// 阿里云图像生成API模块
/// 实现了与阿里云AI图像生成服务的交互功能
use rig::image_generation::{
    ImageGenerationError, ImageGenerationRequest, ImageGenerationResponse,
};
use std::convert::From;

use crate::aliyun::Client;

use super::schemes::{
    AliyunImageGenerationRequest, AliyunImageGenerationResponse,
    AliyunImageGenerationSuccessResponse, AliyunTaskQueryResponse,
};

/// 阿里云图像生成模型
/// 实现rig框架的ImageGenerationModel trait
#[derive(Clone)]
pub struct ImageGenerationModel {
    /// 阿里云API客户端
    client: Client,
    /// 使用的模型名称
    model: String,
}

/// ImageGenerationModel的构造和辅助方法
impl ImageGenerationModel {
    /// 创建新的ImageGenerationModel实例
    ///
    /// # 参数
    /// * `client` - 阿里云API客户端
    /// * `model` - 要使用的模型名称，例如"wanx2.1-t2i-turbo"
    pub fn new(client: Client, model: String) -> Self {
        Self { client, model }
    }

    /// 查询图像生成任务
    ///
    /// # 参数
    /// * `task_id` - 图像生成任务的ID
    ///
    /// # 返回
    /// * 成功 - 包含任务查询结果的AliyunTaskQueryResponse
    pub async fn query_task(
        &self,
        task_id: &str,
    ) -> Result<AliyunTaskQueryResponse, ImageGenerationError> {
        let response = self
            .client
            .get(&format!("api/v1/tasks/{}", task_id))
            .send()
            .await?;

        let body = response.text().await?;

        tracing::debug!("阿里云图像生成任务查询结果: {}", body);
        let response: AliyunTaskQueryResponse = serde_json::from_str(&body)?;

        Ok(response)
    }

    /// 图像生成方法
    /// 将通用请求转换为阿里云特定请求，调用API并处理响应
    ///
    /// ❗IMPORTANT: 阿里云API的图像生成接口是异步的，
    /// 因此需要后续查询任务结果来获取实际的图像数据。
    ///
    /// # 参数
    /// * `request` - 通用图像生成请求
    ///
    /// # 返回
    /// * 成功 - 包含响应数据的ImageGenerationResponse
    /// * 错误 - 包含错误信息的ImageGenerationError
    pub async fn image_generation_task(
        &self,
        request: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse<AliyunImageGenerationSuccessResponse>, ImageGenerationError>
    {
        // 将通用请求转换为阿里云特定请求
        let mut request = AliyunImageGenerationRequest::from(request);
        // 使用配置的模型名称
        request.model = self.model.clone();

        // 调用阿里云API
        let response = self
            .client
            .post("api/v1/services/aigc/text2image/image-synthesis")
            .header("X-DashScope-Async", "enable") // 启用异步模式
            .json(&request)
            .send()
            .await?;

        // 解析响应
        let body = response.text().await?;
        tracing::debug!("阿里云图像生成任务响应: {}", body);
        let response: AliyunImageGenerationResponse = serde_json::from_str(&body)?;

        // 处理不同类型的响应
        match response {
            // 对于成功响应，返回包含响应数据的结果
            // 注意：实际图像数据为空，因为这是异步API，需要后续查询结果
            AliyunImageGenerationResponse::Success(success) => Ok(ImageGenerationResponse {
                image: vec![],
                response: success,
            }),
            // 对于错误响应，返回包含错误信息的错误
            AliyunImageGenerationResponse::Error(error) => {
                Err(ImageGenerationError::ProviderError(error.message))
            }
        }
    }
}

/// 实现rig框架的ImageGenerationModel trait
/// 提供标准化的图像生成接口
impl rig::image_generation::ImageGenerationModel for ImageGenerationModel {
    /// 定义响应类型为我们的阿里云响应枚举类型
    type Response = AliyunImageGenerationSuccessResponse;

    /// 图像生成方法
    /// 将通用请求转换为阿里云特定请求，调用API并处理响应
    ///
    /// ❗IMPORTANT: 阿里云API的图像生成接口是异步的，
    /// 因此需要后续查询任务结果来获取实际的图像数据。
    ///
    /// # 参数
    /// * `request` - 通用图像生成请求
    ///
    /// # 返回
    /// * 成功 - 包含响应数据的ImageGenerationResponse
    /// * 错误 - 包含错误信息的ImageGenerationError
    async fn image_generation(
        &self,
        request: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse<Self::Response>, ImageGenerationError> {
        self.image_generation_task(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rig::image_generation::ImageGenerationModel as ImageGenerationModelTrait;

    #[tokio::test]
    async fn test_image_generation() {
        let client = Client::from_env();
        let model = ImageGenerationModel::new(client, "wanx2.1-t2i-turbo".to_string());

        let request = ImageGenerationRequest {
            prompt: "A beautiful landscape with a river and mountains".to_string(),
            width: 1024,
            height: 1024,
            additional_params: None,
        };

        let response = model.image_generation(request).await.unwrap();
        println!("{:?}", response);
    }
}
