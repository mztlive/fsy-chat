use std::convert::From;

use crate::aliyun::{
    Client,
    scheme::{AliyunError, AsyncGenerationOutput, GenerationRequest},
};

use super::schemes::Text2VideoGenerationRequest;

// pub type Text2VideoGenerationRequest = GenerationRequest<Text2VideoInput, Text2VideoParameters>;

/// 阿里云图像生成模型
/// 实现rig框架的ImageGenerationModel trait
#[derive(Clone)]
pub struct VideoGenerationModel {
    /// 阿里云API客户端
    client: Client,
    /// 使用的模型名称
    model: String,
}

/// ImageGenerationModel的构造和辅助方法
impl VideoGenerationModel {
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
    /// * 成功 - 包含响应数据的AsyncGenerationOutput
    /// * 错误 - 包含错误信息的AliyunError
    pub async fn create_task(
        &self,
        request: Text2VideoGenerationRequest,
    ) -> Result<AsyncGenerationOutput, AliyunError> {
        // 将通用请求转换为阿里云特定请求
        let mut request = GenerationRequest::from(request);
        // 使用配置的模型名称
        request.model = self.model.clone();

        self.client
            .async_generate_task(
                request,
                "api/v1/services/aigc/video-generation/video-synthesis",
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::aliyun::media::schemes::{Text2VideoInput, Text2VideoParameters};

    use super::*;

    #[tokio::test]
    async fn test_video_generation() {
        let client = Client::from_env();
        let model = VideoGenerationModel::new(client, "wanx2.1-t2v-turbo".to_string());

        let request = Text2VideoGenerationRequest {
            input: Text2VideoInput {
                prompt: "A beautiful landscape with a river and mountains".to_string(),
            },
            parameters: Text2VideoParameters {
                size: Some("1024*1024".to_string()),
                duration: Some(5),
                prompt_extend: Some(true),
                seed: Some(123456),
            },
        };
        let response = model.create_task(request).await.unwrap();
        println!("{:?}", response);
    }
}
