use sqlx::Row;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, ApiResponse};
use crate::routes::AppState;

/// 查询参数
pub async fn query(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let node_id = params.get("nodeId").and_then(|v| v.as_str()).unwrap_or("");
    
    if node_id.is_empty() {
        // 查询所有参数
        let rows: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
            "SELECT id, name, code, hcode FROM tb_node_parameter"
        )
        .fetch_all(&state.db.pool)
        .await?;
        
        let result: Vec<serde_json::Value> = rows.iter().map(|r| {
            serde_json::json!({
                "id": r.try_get::<String, _>("id").unwrap_or_default(),
                "name": r.try_get::<Option<String>, _>("name").ok().flatten(),
                "code": r.try_get::<Option<String>, _>("code").ok().flatten(),
                "hcode": r.try_get::<Option<String>, _>("hcode").ok().flatten(),
            })
        }).collect();
        
        let count = result.len() as i32;
        
        Ok(Json(serde_json::json!({
            "code": 1,
            "msg": "",
            "data": result,
            "count": count,
        })))
    } else {
        // 根据节点ID查询
        let rows: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
            "SELECT id, nodeid, npid FROM tb_node_and_node_parameter WHERE nodeid = ?"
        )
        .bind(node_id)
        .fetch_all(&state.db.pool)
        .await?;
        
        let result: Vec<serde_json::Value> = rows.iter().map(|r| {
            serde_json::json!({
                "id": r.try_get::<String, _>("id").unwrap_or_default(),
                "node_id": r.try_get::<Option<String>, _>("nodeid").ok().flatten(),
                "npid": r.try_get::<Option<String>, _>("npid").ok().flatten(),
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
}

/// 添加参数
pub async fn add(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let code = params.get("code").and_then(|v| v.as_str());
    let hcode = params.get("hcode").and_then(|v| v.as_str());
    
    let param_id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO tb_node_parameter (id, name, code, hcode) VALUES (?, ?, ?, ?)"
    )
    .bind(&param_id)
    .bind(name)
    .bind(code)
    .bind(hcode)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "添加成功"
    })))
}

/// 更新参数
pub async fn update(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    let name = params.get("name").and_then(|v| v.as_str());
    let code = params.get("code").and_then(|v| v.as_str());
    let hcode = params.get("hcode").and_then(|v| v.as_str());
    
    sqlx::query(
        "UPDATE tb_node_parameter SET name = COALESCE(?, name), code = COALESCE(?, code), hcode = COALESCE(?, hcode) WHERE id = ?"
    )
    .bind(name)
    .bind(code)
    .bind(hcode)
    .bind(id)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "更新成功"
    })))
}

/// 删除参数
pub async fn delete(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_node_parameter WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "删除成功"
    })))
}

/// 绑定参数
pub async fn bind(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let node_id = params.get("nodeId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少nodeId参数".to_string()))?;
    let np_id = params.get("npId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少npId参数".to_string()))?;
    
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO tb_node_and_node_parameter (id, nodeid, npid) VALUES (?, ?, ?)"
    )
    .bind(&id)
    .bind(node_id)
    .bind(np_id)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "绑定成功"
    })))
}

/// 解绑参数
pub async fn unbind(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let node_id = params.get("nodeId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少nodeId参数".to_string()))?;
    let np_id = params.get("npId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少npId参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_node_and_node_parameter WHERE nodeid = ? AND npid = ?")
        .bind(node_id)
        .bind(np_id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "解绑成功"
    })))
}