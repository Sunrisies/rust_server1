use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// 应用错误类型
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis错误: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("认证失败: {0}")]
    Auth(String),

    #[error("未找到资源: {0}")]
    NotFound(String),

    #[error("参数错误: {0}")]
    BadRequest(String),

    #[error("权限不足: {0}")]
    Forbidden(String),

    #[error("内部错误: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Database(e) => {
                tracing::error!("数据库错误: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string())
            }
            AppError::Redis(e) => {
                tracing::error!("Redis错误: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string())
            }
            AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::Internal(e) => {
                tracing::error!("内部错误: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string())
            }
        };

        let body = Json(json!({
            "code": status.as_u16(),
            "message": message,
        }));

        (status, body).into_response()
    }
}

/// 统一响应结构
#[derive(Debug, serde::Serialize)]
pub struct ApiResponse<T: serde::Serialize> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: serde::Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }
}

impl<T: serde::Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        Json(json!({
            "code": self.code,
            "message": self.message,
            "data": self.data,
        }))
        .into_response()
    }
}
