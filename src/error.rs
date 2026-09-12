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
                (StatusCode::OK, "服务器内部错误".to_string())
            }
            AppError::Redis(e) => {
                tracing::error!("Redis错误: {}", e);
                (StatusCode::OK, "服务器内部错误".to_string())
            }
            AppError::Auth(msg) => (StatusCode::OK, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::OK, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::OK, msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::OK, msg.clone()),
            AppError::Internal(e) => {
                tracing::error!("内部错误: {}", e);
                (StatusCode::OK, "服务器内部错误".to_string())
            }
        };

        // 匹配Java版本的错误响应格式
        let body = Json(json!({
            "code": 0,
            "msg": message,
        }));

        (status, body).into_response()
    }
}

/// 统一响应结构（匹配Java版本的LayUiTableTemplateLay）
#[derive(Debug, serde::Serialize)]
pub struct ApiResponse<T: serde::Serialize> {
    pub code: i32,  // 1表示成功，0表示失败
    #[serde(rename = "msg")]
    pub message: String,
    pub data: Option<T>,
    pub count: Option<i32>,
    pub obj: Option<serde_json::Value>,
}

impl<T: serde::Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 1,
            message: "success".to_string(),
            data: Some(data),
            count: None,
            obj: None,
        }
    }

    pub fn success_with_count(data: T, count: i32) -> Self {
        Self {
            code: 1,
            message: "success".to_string(),
            data: Some(data),
            count: Some(count),
            obj: None,
        }
    }

    pub fn error(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
            count: None,
            obj: None,
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
