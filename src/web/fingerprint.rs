use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::session_manager::UserID;

use super::errors::ApiResponse;

/// 授权中间件
///
/// # 功能
/// - 验证请求头中的JWT令牌
/// - 从令牌中提取用户ID和账号信息
/// - 将用户信息注入到请求扩展中
///
/// # 参数
/// - state: 应用状态
/// - request: HTTP请求
/// - next: 下一个处理器
///
/// # 返回
/// - 如果验证成功,继续处理请求
/// - 如果验证失败,返回未授权错误
pub async fn authorization(mut request: Request, next: Next) -> Response {
    let fingerprint = request.headers().get("X-Fingerprint").cloned();

    match fingerprint {
        Some(fingerprint) => {
            if let Ok(fingerprint) = fingerprint.to_str() {
                request
                    .extensions_mut()
                    .insert(UserID(fingerprint.to_string()));

                return next.run(request).await;
            }

            return ApiResponse::<()>::internal_server_error("指纹为空".to_string())
                .into_response();
        }
        None => {
            return ApiResponse::<()>::internal_server_error("指纹为空".to_string())
                .into_response();
        }
    }
}
