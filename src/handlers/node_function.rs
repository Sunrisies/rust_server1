use axum::{extract::State, Json};
use sqlx::Row;

use crate::error::AppError;
use crate::routes::AppState;

/// 添加船只功能
pub async fn add(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let function_type = params.get("type").and_then(|v| v.as_str());
    let code = params.get("code").and_then(|v| v.as_str());
    let remark = params.get("remark").and_then(|v| v.as_str());
    
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO tb_node_function (id, name, type, code, remark, `on`) VALUES (?, ?, ?, ?, ?, '1')"
    )
    .bind(&id)
    .bind(name)
    .bind(function_type)
    .bind(code)
    .bind(remark)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "添加成功"
    })))
}

/// 更新船只功能
pub async fn update(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    let name = params.get("name").and_then(|v| v.as_str());
    let function_type = params.get("type").and_then(|v| v.as_str());
    let code = params.get("code").and_then(|v| v.as_str());
    let remark = params.get("remark").and_then(|v| v.as_str());
    
    sqlx::query(
        "UPDATE tb_node_function SET name = COALESCE(?, name), type = COALESCE(?, type), code = COALESCE(?, code), remark = COALESCE(?, remark) WHERE id = ?"
    )
    .bind(name)
    .bind(function_type)
    .bind(code)
    .bind(remark)
    .bind(id)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "更新成功"
    })))
}

/// 删除船只功能
pub async fn delete(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_node_function WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "删除成功"
    })))
}

/// 查询船只功能
pub async fn query(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let keywords = params.get("keywords").and_then(|v| v.as_str());
    
    let mut query_str = String::from("SELECT id, name, type, code, remark, `on` FROM tb_node_function WHERE 1=1");
    let mut bind_values: Vec<String> = Vec::new();
    
    if let Some(kw) = keywords {
        if !kw.is_empty() {
            query_str.push_str(" AND (name LIKE ? OR remark LIKE ?)");
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
            "type": r.try_get::<Option<String>, _>("type").ok().flatten(),
            "code": r.try_get::<Option<String>, _>("code").ok().flatten(),
            "remark": r.try_get::<Option<String>, _>("remark").ok().flatten(),
            "on": r.try_get::<Option<String>, _>("on").ok().flatten(),
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

/// 根据船只ID查询功能
pub async fn query_by_node_id(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let node_id = params.get("id").and_then(|v| v.as_str()).unwrap_or("");
    
    let rows: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT nf.id, nf.name, nf.type, nf.code, nf.remark, nf.`on` 
         FROM tb_node_function nf 
         INNER JOIN tb_node_and_function nf2 ON nf.id = nf2.fid 
         WHERE nf2.nid = ?"
    )
    .bind(node_id)
    .fetch_all(&state.db.pool)
    .await?;
    
    let result: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "name": r.try_get::<Option<String>, _>("name").ok().flatten(),
            "type": r.try_get::<Option<String>, _>("type").ok().flatten(),
            "code": r.try_get::<Option<String>, _>("code").ok().flatten(),
            "remark": r.try_get::<Option<String>, _>("remark").ok().flatten(),
            "on": r.try_get::<Option<String>, _>("on").ok().flatten(),
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

/// 添加船只功能关联
pub async fn add_binding(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let nid = params.get("nid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少nid参数".to_string()))?;
    let fid = params.get("fid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少fid参数".to_string()))?;
    
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO tb_node_and_function (id, nid, fid) VALUES (?, ?, ?)"
    )
    .bind(&id)
    .bind(nid)
    .bind(fid)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "绑定成功"
    })))
}

/// 删除船只功能关联
pub async fn delete_binding(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let nid = params.get("nid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少nid参数".to_string()))?;
    let fid = params.get("fid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少fid参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_node_and_function WHERE nid = ? AND fid = ?")
        .bind(nid)
        .bind(fid)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "解绑成功"
    })))
}