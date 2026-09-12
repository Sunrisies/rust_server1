use axum::{extract::State, Json};
use sqlx::Row;

use crate::error::AppError;
use crate::routes::AppState;

/// 添加船只权限
pub async fn add(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let code = params.get("code").and_then(|v| v.as_i64());
    let details = params.get("details").and_then(|v| v.as_str());
    
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO tb_node_jurisdiction (id, name, code, details) VALUES (?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(name)
    .bind(code)
    .bind(details)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "添加成功"
    })))
}

/// 更新船只权限
pub async fn update(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    let name = params.get("name").and_then(|v| v.as_str());
    let code = params.get("code").and_then(|v| v.as_i64());
    let details = params.get("details").and_then(|v| v.as_str());
    
    sqlx::query(
        "UPDATE tb_node_jurisdiction SET name = COALESCE(?, name), code = COALESCE(?, code), details = COALESCE(?, details) WHERE id = ?"
    )
    .bind(name)
    .bind(code)
    .bind(details)
    .bind(id)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "更新成功"
    })))
}

/// 删除船只权限
pub async fn delete(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_node_jurisdiction WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "删除成功"
    })))
}

/// 查询所有船只权限
pub async fn query_all(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let keywords = params.get("keywords").and_then(|v| v.as_str());
    
    let mut query_str = String::from("SELECT id, name, code, details FROM tb_node_jurisdiction WHERE 1=1");
    let mut bind_values: Vec<String> = Vec::new();
    
    if let Some(kw) = keywords {
        if !kw.is_empty() {
            query_str.push_str(" AND (name LIKE ? OR details LIKE ?)");
            let pattern = format!("%{}%", kw);
            bind_values.push(pattern.clone());
            bind_values.push(pattern);
        }
    }
    
    let rows: Vec<sqlx::mysql::MySqlRow> = if bind_values.is_empty() {
        sqlx::query(&query_str).fetch_all(&state.db.pool).await?
    } else {
        let mut q = sqlx::query(&query_str);
        for v in &bind_values {
            q = q.bind(v);
        }
        q.fetch_all(&state.db.pool).await?
    };
    
    let result: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "name": r.try_get::<Option<String>, _>("name").ok().flatten(),
            "code": r.try_get::<Option<i32>, _>("code").ok().flatten(),
            "details": r.try_get::<Option<String>, _>("details").ok().flatten(),
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

/// 根据船只ID查询权限
pub async fn query_by_node_id(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let node_id = params.get("id").and_then(|v| v.as_str()).unwrap_or("");
    
    let rows: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT nj.id, nj.name, nj.code, nj.details 
         FROM tb_node_jurisdiction nj 
         INNER JOIN tb_node_and_jurisdiction nj2 ON nj.id = nj2.jurid 
         WHERE nj2.nodeid = ?"
    )
    .bind(node_id)
    .fetch_all(&state.db.pool)
    .await?;
    
    let result: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "name": r.try_get::<Option<String>, _>("name").ok().flatten(),
            "code": r.try_get::<Option<i32>, _>("code").ok().flatten(),
            "details": r.try_get::<Option<String>, _>("details").ok().flatten(),
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

/// 添加船只权限关联
pub async fn add_binding(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let nodeid = params.get("nodeid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少nodeid参数".to_string()))?;
    let jurid = params.get("jurid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少jurid参数".to_string()))?;
    
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO tb_node_and_jurisdiction (id, nodeid, jurid) VALUES (?, ?, ?)"
    )
    .bind(&id)
    .bind(nodeid)
    .bind(jurid)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "绑定成功"
    })))
}

/// 删除船只权限关联
pub async fn delete_binding(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let nodeid = params.get("nodeid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少nodeid参数".to_string()))?;
    let jurid = params.get("jurid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少jurid参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_node_and_jurisdiction WHERE nodeid = ? AND jurid = ?")
        .bind(nodeid)
        .bind(jurid)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "解绑成功"
    })))
}