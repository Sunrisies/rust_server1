use axum::{extract::State, Json};
use sqlx::Row;

use crate::error::AppError;
use crate::routes::AppState;

/// 添加白名单
pub async fn add(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let ip_address = params.get("ipAddress").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少ipAddress参数".to_string()))?;
    let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let desc = params.get("desc").and_then(|v| v.as_str()).unwrap_or("");
    
    let create_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let whitelist_data = serde_json::json!({
        "ipAddress": ip_address,
        "name": name,
        "desc": desc,
        "createTime": create_time,
    });
    
    state.redis.set(
        &format!("whitelist:{}", ip_address),
        &whitelist_data.to_string()
    ).await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "添加成功"
    })))
}

/// 删除白名单
pub async fn remove(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let ip_address = params.get("ipAddress").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少ipAddress参数".to_string()))?;
    
    state.redis.del(&format!("whitelist:{}", ip_address)).await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "删除成功"
    })))
}

/// 查询所有白名单
pub async fn query_all(
    State(state): State<AppState>,
    Json(_params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "",
        "data": [],
        "count": 0,
    })))
}