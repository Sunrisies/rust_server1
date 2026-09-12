use sqlx::Row;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, ApiResponse};
use crate::routes::AppState;

/// 查询所有船只
pub async fn query_all(
    State(state): State<AppState>,
    Json(_params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT id, sname, ip, port, state, taskid, describes FROM tb_node"
    )
    .fetch_all(&state.db.pool)
    .await?;
    
    let nodes: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "sname": r.try_get::<Option<String>, _>("sname").ok().flatten(),
            "ip": r.try_get::<Option<String>, _>("ip").ok().flatten(),
            "port": r.try_get::<Option<String>, _>("port").ok().flatten(),
            "state": r.try_get::<Option<String>, _>("state").ok().flatten(),
            "taskid": r.try_get::<Option<String>, _>("taskid").ok().flatten(),
            "describes": r.try_get::<Option<String>, _>("describes").ok().flatten(),
        })
    }).collect();
    
    let count = nodes.len() as i32;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "",
        "data": nodes,
        "count": count,
    })))
}

/// 添加船只
pub async fn add(
    State(state): State<AppState>,
    Json(node): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let sname = node.get("sname").and_then(|v| v.as_str()).unwrap_or("");
    let ip = node.get("ip").and_then(|v| v.as_str());
    let port = node.get("port").and_then(|v| v.as_str());
    let describes = node.get("describes").and_then(|v| v.as_str());
    
    let node_id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO tb_node (id, sname, ip, port, state, describes) VALUES (?, ?, ?, ?, 'offline', ?)"
    )
    .bind(&node_id)
    .bind(sname)
    .bind(ip)
    .bind(port)
    .bind(describes)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "添加成功",
        "data": {"id": node_id}
    })))
}

/// 更新船只
pub async fn update(
    State(state): State<AppState>,
    Json(node): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = node.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    let sname = node.get("sname").and_then(|v| v.as_str());
    let ip = node.get("ip").and_then(|v| v.as_str());
    let port = node.get("port").and_then(|v| v.as_str());
    let state_val = node.get("state").and_then(|v| v.as_str());
    let describes = node.get("describes").and_then(|v| v.as_str());
    
    sqlx::query(
        "UPDATE tb_node SET sname = COALESCE(?, sname), ip = COALESCE(?, ip), port = COALESCE(?, port), state = COALESCE(?, state), describes = COALESCE(?, describes) WHERE id = ?"
    )
    .bind(sname)
    .bind(ip)
    .bind(port)
    .bind(state_val)
    .bind(describes)
    .bind(id)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "更新成功"
    })))
}

/// 删除船只
pub async fn delete(
    State(state): State<AppState>,
    Json(node): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = node.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_node WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "删除成功"
    })))
}

/// 根据用户查询船只
pub async fn query_by_user(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = params.get("id").or_else(|| params.get("userId"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    
    let rows: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT n.id, n.sname, n.ip, n.port, n.state, n.taskid, n.describes 
         FROM tb_node n 
         INNER JOIN tb_user_node un ON n.id = un.nodeid 
         WHERE un.userid = ? AND (un.removecode IS NULL OR un.removecode = 1)"
    )
    .bind(user_id)
    .fetch_all(&state.db.pool)
    .await?;
    
    let nodes: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "sname": r.try_get::<Option<String>, _>("sname").ok().flatten(),
            "ip": r.try_get::<Option<String>, _>("ip").ok().flatten(),
            "port": r.try_get::<Option<String>, _>("port").ok().flatten(),
            "state": r.try_get::<Option<String>, _>("state").ok().flatten(),
            "taskid": r.try_get::<Option<String>, _>("taskid").ok().flatten(),
            "describes": r.try_get::<Option<String>, _>("describes").ok().flatten(),
        })
    }).collect();
    
    let count = nodes.len() as i32;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "",
        "data": nodes,
        "count": count,
    })))
}

/// 更新船只位置
pub async fn update_location(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    let latitude = params.get("latitude").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let longitude = params.get("longitude").and_then(|v| v.as_f64()).unwrap_or(0.0);
    
    let location_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO tb_node_location (id, latitude, longitude, update_time) VALUES (?, ?, ?, NOW())"
    )
    .bind(&location_id)
    .bind(latitude)
    .bind(longitude)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "更新成功"
    })))
}