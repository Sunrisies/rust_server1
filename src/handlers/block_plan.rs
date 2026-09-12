use axum::{extract::State, Json};
use sqlx::Row;

use crate::error::AppError;
use crate::routes::AppState;

/// 添加或更新区块任务
pub async fn add_or_update(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str());
    let boat_id = params.get("boat_id").and_then(|v| v.as_str()).unwrap_or("");
    let task_name = params.get("task_name").and_then(|v| v.as_str());
    let polygon = params.get("polygon").and_then(|v| v.as_str());
    let route = params.get("route").and_then(|v| v.as_str());
    
    if let Some(plan_id) = id {
        // 更新
        sqlx::query(
            "UPDATE block_plan SET task_name = COALESCE(?, task_name), polygon = COALESCE(?, polygon), route = COALESCE(?, route), update_time = NOW() WHERE id = ?"
        )
        .bind(task_name)
        .bind(polygon)
        .bind(route)
        .bind(plan_id)
        .execute(&state.db.pool)
        .await?;
        
        Ok(Json(serde_json::json!({
            "code": 1,
            "msg": "更新成功"
        })))
    } else {
        // 新增
        let plan_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO block_plan (id, boat_id, task_name, polygon, route, add_time) VALUES (?, ?, ?, ?, ?, NOW())"
        )
        .bind(&plan_id)
        .bind(boat_id)
        .bind(task_name)
        .bind(polygon)
        .bind(route)
        .execute(&state.db.pool)
        .await?;
        
        Ok(Json(serde_json::json!({
            "code": 1,
            "msg": "添加成功",
            "data": {"id": plan_id}
        })))
    }
}

/// 删除区块任务
pub async fn delete(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    
    sqlx::query("DELETE FROM block_plan WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "删除成功"
    })))
}

/// 根据ID查询区块任务
pub async fn query_by_id(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    
    let row: Option<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT id, boat_id, task_name, polygon, route FROM block_plan WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await?;
    
    if let Some(r) = row {
        Ok(Json(serde_json::json!({
            "code": 1,
            "msg": "",
            "data": {
                "id": r.try_get::<String, _>("id").unwrap_or_default(),
                "boat_id": r.try_get::<Option<String>, _>("boat_id").ok().flatten(),
                "task_name": r.try_get::<Option<String>, _>("task_name").ok().flatten(),
                "polygon": r.try_get::<Option<String>, _>("polygon").ok().flatten(),
                "route": r.try_get::<Option<String>, _>("route").ok().flatten(),
            }
        })))
    } else {
        Ok(Json(serde_json::json!({
            "code": 0,
            "msg": "未找到任务"
        })))
    }
}

/// 根据船只ID查询区块任务
pub async fn query_by_boat_id(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let boat_id = params.get("boatId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少boatId参数".to_string()))?;
    
    let rows: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT id, boat_id, task_name FROM block_plan WHERE boat_id = ?"
    )
    .bind(boat_id)
    .fetch_all(&state.db.pool)
    .await?;
    
    let result: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "boat_id": r.try_get::<Option<String>, _>("boat_id").ok().flatten(),
            "task_name": r.try_get::<Option<String>, _>("task_name").ok().flatten(),
        })
    }).collect();
    
    let count = result.len() as i32;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "",
        "data": result,
        "count": count,
    })))
}