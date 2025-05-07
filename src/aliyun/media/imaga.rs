/// 阿里云图像生成API模块
/// 实现了与阿里云AI图像生成服务的交互功能
use rig::image_generation::{
    ImageGenerationError, ImageGenerationRequest, ImageGenerationResponse,
};
use std::convert::From;

use crate::aliyun::{
    Client,
    scheme::{
        AliyunError, AsyncGenerationOutput, AsyncGenerationSuccessResponse,
        AsyncImageGenerationResponse, GenerationRequest,
    },
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
    ) -> Result<AsyncGenerationOutput, AliyunError> {
        // 将通用请求转换为阿里云特定请求
        let mut request = GenerationRequest::from(request);
        // 使用配置的模型名称
        request.model = self.model.clone();

        self.client
            .async_generate_task(request, "api/v1/services/aigc/text2image/image-synthesis")
            .await
    }
}

/// 实现rig框架的ImageGenerationModel trait
/// 提供标准化的图像生成接口
impl rig::image_generation::ImageGenerationModel for ImageGenerationModel {
    /// 定义响应类型为我们的阿里云响应枚举类型
    type Response = AsyncGenerationOutput;

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
        match self.image_generation_task(request).await {
            Ok(output) => Ok(ImageGenerationResponse {
                image: vec![],
                response: output,
            }),
            Err(e) => Err(ImageGenerationError::ProviderError(e.to_string())),
        }
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
