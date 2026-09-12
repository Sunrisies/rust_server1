use axum::{extract::State, Json};
use futures_util::StreamExt;

use crate::error::AppError;
use crate::routes::AppState;

/// 上传Logo
pub async fn upload_logo(
    State(state): State<AppState>,
    mut multipart: axum::extract::Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut file_name = String::new();
    let mut file_data: Vec<u8> = Vec::new();
    
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            file_name = field.file_name().unwrap_or("").to_string();
            let data = field.bytes().await.unwrap_or_default();
            file_data = data.to_vec();
        }
    }
    
    if file_name.is_empty() {
        return Ok(Json(serde_json::json!({
            "code": 0,
            "msg": "未选择文件"
        })));
    }
    
    // 生成文件路径
    let timestamp = chrono::Local::now().timestamp_millis();
    let upload_dir = format!("uploads/logo/{}", timestamp);
    std::fs::create_dir_all(&upload_dir).map_err(|e| {
        AppError::Internal(anyhow::anyhow!("创建目录失败: {}", e))
    })?;
    
    let file_path = format!("{}/{}", upload_dir, file_name);
    std::fs::write(&file_path, &file_data).map_err(|e| {
        AppError::Internal(anyhow::anyhow!("保存文件失败: {}", e))
    })?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "上传成功",
        "data": {
            "id": timestamp.to_string(),
            "url": format!("/{}", file_path),
        }
    })))
}

/// 上传图标
pub async fn upload_ico(
    State(state): State<AppState>,
    multipart: axum::extract::Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    upload_logo(State(state), multipart).await
}

/// 上传APP
pub async fn upload_app(
    State(state): State<AppState>,
    multipart: axum::extract::Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    upload_logo(State(state), multipart).await
}