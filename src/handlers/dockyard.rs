use axum::{extract::State, Json};
use sqlx::Row;

use crate::error::AppError;
use crate::routes::AppState;

/// 添加船坞任务
pub async fn add(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let boat_id = params.get("boat_id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少boat_id参数".to_string()))?;
    let task_id = uuid::Uuid::new_v4().to_string();
    
    let task_data = serde_json::json!({
        "id": task_id,
        "boat_id": boat_id,
        "name": params.get("name").and_then(|v| v.as_str()).unwrap_or(""),
        "state": "1",
        "is_out": params.get("is_out").and_then(|v| v.as_bool()).unwrap_or(false),
        "is_stop": params.get("is_stop").and_then(|v| v.as_bool()).unwrap_or(false),
    });
    
    state.redis.set(
        &format!("dockyard:{}:{}", boat_id, task_id),
        &task_data.to_string()
    ).await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "添加成功",
        "data": {"id": task_id}
    })))
}

/// 查询船坞任务
pub async fn query(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let boat_id = params.get("boatId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少boatId参数".to_string()))?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "",
        "data": [],
        "count": 0,
    })))
}

/// 查询单个船坞任务
pub async fn query_single(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let boat_id = params.get("boatId").and_then(|v| v.as_str()).unwrap_or("");
    let id = params.get("id").and_then(|v| v.as_str()).unwrap_or("");
    
    let key = format!("dockyard:{}:{}", boat_id, id);
    match state.redis.get(&key).await? {
        Some(data) => {
            let task: serde_json::Value = serde_json::from_str(&data)?;
            Ok(Json(serde_json::json!({
                "code": 1,
                "msg": "",
                "data": [task],
            })))
        }
        None => {
            Ok(Json(serde_json::json!({
                "code": 1,
                "msg": "未找到任务",
                "data": [],
            })))
        }
    }
}

/// 删除船坞任务
pub async fn delete(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let boat_id = params.get("boatId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少boatId参数".to_string()))?;
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    
    let key = format!("dockyard:{}:{}", boat_id, id);
    state.redis.del(&key).await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "删除成功"
    })))
}

/// 更新船坞任务
pub async fn update(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let boat_id = params.get("boat_id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少boat_id参数".to_string()))?;
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    
    let key = format!("dockyard:{}:{}", boat_id, id);
    let task_data = serde_json::to_string(&params)?;
    state.redis.set(&key, &task_data).await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "更新成功"
    })))
}