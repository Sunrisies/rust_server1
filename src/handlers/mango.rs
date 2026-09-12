use axum::{extract::State, Json};

use crate::error::AppError;
use crate::routes::AppState;

/// MongoDB条件查询
pub async fn find(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "",
        "data": [],
        "count": 0,
    })))
}

/// MongoDB查询历史数据
pub async fn find_historical_data(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "",
        "data": [],
        "count": 0,
    })))
}

/// MongoDB次数查询
pub async fn find_num(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "",
        "data": [],
        "count": 0,
    })))
}

/// 执行重启脚本
pub async fn shell() -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "执行成功",
        "data": {"result": "Success"}
    })))
}