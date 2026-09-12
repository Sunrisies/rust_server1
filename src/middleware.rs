use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use crate::jwt::JwtUtil;

/// 从请求中提取Token
fn extract_token(request: &Request) -> Option<String> {
    // 1. 先从Authorization header获取
    if let Some(auth_header) = request.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }

    // 2. 从query参数获取 (用于WebSocket等场景)
    if let Some(query) = request.uri().query() {
        for param in query.split('&') {
            let mut parts = param.splitn(2, '=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                if key == "token" {
                    return Some(value.to_string());
                }
            }
        }
    }

    None
}

/// 认证中间件
pub async fn auth_middleware(
    State(jwt_util): State<JwtUtil>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_string();

    // 白名单路径 - 不需要认证
    let whitelist = [
        "/auth/login",
        "/auth/register",
        "/health",
        "/api/health",
    ];

    if whitelist.iter().any(|prefix| path.starts_with(prefix)) {
        return Ok(next.run(request).await);
    }

    // 提取Token
    let token = extract_token(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 验证Token
    match jwt_util.validate_token(&token) {
        Ok(claims) => {
            // 将用户信息添加到请求扩展中
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

/// 从请求扩展中获取用户信息
pub fn get_user_from_request(request: &Request) -> Option<&crate::jwt::Claims> {
    request.extensions().get::<crate::jwt::Claims>()
}
