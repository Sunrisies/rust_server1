use sqlx::Row;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, ApiResponse};
use crate::routes::AppState;

/// 添加权限
pub async fn add(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mname = params.get("mname").and_then(|v| v.as_str()).unwrap_or("");
    let mdesc = params.get("mdesc").and_then(|v| v.as_str());
    let uri = params.get("uri").and_then(|v| v.as_str());
    let method = params.get("method").and_then(|v| v.as_str());
    
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO tb_permission (id, mname, mdesc, uri, method, removecode) VALUES (?, ?, ?, ?, ?, '1')"
    )
    .bind(&id)
    .bind(mname)
    .bind(mdesc)
    .bind(uri)
    .bind(method)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "添加成功"
    })))
}

/// 更新权限
pub async fn update(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    let mname = params.get("mname").and_then(|v| v.as_str());
    let mdesc = params.get("mdesc").and_then(|v| v.as_str());
    let uri = params.get("uri").and_then(|v| v.as_str());
    let method = params.get("method").and_then(|v| v.as_str());
    
    sqlx::query(
        "UPDATE tb_permission SET mname = COALESCE(?, mname), mdesc = COALESCE(?, mdesc), uri = COALESCE(?, uri), method = COALESCE(?, method) WHERE id = ?"
    )
    .bind(mname)
    .bind(mdesc)
    .bind(uri)
    .bind(method)
    .bind(id)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "更新成功"
    })))
}

/// 删除权限
pub async fn delete(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_permission WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "删除成功"
    })))
}

/// 查询所有权限
pub async fn query_all(
    State(state): State<AppState>,
    Json(_params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT id, mname, mdesc, uri, method, parentid FROM tb_permission WHERE removecode = '1'"
    )
    .fetch_all(&state.db.pool)
    .await?;
    
    let result: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "mname": r.try_get::<Option<String>, _>("mname").ok().flatten(),
            "mdesc": r.try_get::<Option<String>, _>("mdesc").ok().flatten(),
            "uri": r.try_get::<Option<String>, _>("uri").ok().flatten(),
            "method": r.try_get::<Option<String>, _>("method").ok().flatten(),
            "parentid": r.try_get::<Option<String>, _>("parentid").ok().flatten(),
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