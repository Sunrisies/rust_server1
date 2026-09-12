use sqlx::Row;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, ApiResponse};
use crate::routes::AppState;

/// 查询所有用户
pub async fn query_all(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.get("page").and_then(|v| v.as_i64()).unwrap_or(1);
    let limit = params.get("limit").and_then(|v| v.as_i64()).unwrap_or(100);
    let keywords = params.get("keywords").and_then(|v| v.as_str());
    
    let mut query = String::from(
        "SELECT id, username, nickname, phone, logo, moduleids, remarks FROM tb_user WHERE 1=1"
    );
    let mut bind_values: Vec<String> = Vec::new();
    
    if let Some(kw) = keywords {
        if !kw.is_empty() {
            query.push_str(" AND (username LIKE ? OR nickname LIKE ?)");
            let pattern = format!("%{}%", kw);
            bind_values.push(pattern.clone());
            bind_values.push(pattern);
        }
    }
    
    query.push_str(" ORDER BY id DESC");
    
    if page > 0 && limit > 0 {
        let offset = (page - 1) * limit;
        query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
    }
    
    let rows: Vec<sqlx::mysql::MySqlRow> = if bind_values.is_empty() {
        sqlx::query(&query).fetch_all(&state.db.pool).await?
    } else {
        let mut q = sqlx::query(&query);
        for v in &bind_values {
            q = q.bind(v);
        }
        q.fetch_all(&state.db.pool).await?
    };
    
    let users: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "username": r.try_get::<Option<String>, _>("username").ok().flatten(),
            "nickname": r.try_get::<Option<String>, _>("nickname").ok().flatten(),
            "phone": r.try_get::<Option<String>, _>("phone").ok().flatten(),
            "logo": r.try_get::<Option<String>, _>("logo").ok().flatten(),
            "moduleids": r.try_get::<Option<String>, _>("moduleids").ok().flatten(),
            "remarks": r.try_get::<Option<String>, _>("remarks").ok().flatten(),
        })
    }).collect();
    
    let count = users.len() as i32;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "",
        "data": users,
        "count": count,
    })))
}

/// 添加用户
pub async fn add(
    State(state): State<AppState>,
    Json(user): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let username = user.get("username").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少username参数".to_string()))?;
    let password = user.get("password").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少password参数".to_string()))?;
    let nickname = user.get("nickname").and_then(|v| v.as_str());
    let phone = user.get("phone").and_then(|v| v.as_str());
    
    // 检查用户名是否已存在
    let exists: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0 FROM tb_user WHERE username = ?"
    )
    .bind(username)
    .fetch_one(&state.db.pool)
    .await?;
    
    if exists {
        return Ok(Json(serde_json::json!({
            "code": 0,
            "msg": "用户名已存在"
        })));
    }
    
    // 加密密码
    let hashed_password = bcrypt::hash(password, 4)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("密码加密失败: {}", e)))?;
    
    let user_id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO tb_user (id, username, password, nickname, phone, created, updated) VALUES (?, ?, ?, ?, ?, NOW(), NOW())"
    )
    .bind(&user_id)
    .bind(username)
    .bind(&hashed_password)
    .bind(nickname)
    .bind(phone)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "添加成功"
    })))
}

/// 更新用户
pub async fn update(
    State(state): State<AppState>,
    Json(user): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = user.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    let username = user.get("username").and_then(|v| v.as_str());
    let nickname = user.get("nickname").and_then(|v| v.as_str());
    let phone = user.get("phone").and_then(|v| v.as_str());
    
    sqlx::query(
        "UPDATE tb_user SET username = COALESCE(?, username), nickname = COALESCE(?, nickname), phone = COALESCE(?, phone), updated = NOW() WHERE id = ?"
    )
    .bind(username)
    .bind(nickname)
    .bind(phone)
    .bind(id)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "更新成功"
    })))
}

/// 删除用户
pub async fn delete(
    State(state): State<AppState>,
    Json(user): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = user.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_user WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "删除成功"
    })))
}

/// 更新用户资料
pub async fn update_profile(
    State(state): State<AppState>,
    Json(user): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    update(State(state), Json(user)).await
}

/// 更新用户密码
pub async fn update_password(
    State(state): State<AppState>,
    Json(data): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = data.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    let old_password = data.get("oldPassword").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少oldPassword参数".to_string()))?;
    let new_password = data.get("newPassword").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少newPassword参数".to_string()))?;
    
    let current_password: String = sqlx::query_scalar(
        "SELECT password FROM tb_user WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;
    
    let valid = if current_password.starts_with("$2") {
        bcrypt::verify(old_password, &current_password).unwrap_or(false)
    } else {
        current_password == old_password
    };
    
    if !valid {
        return Ok(Json(serde_json::json!({
            "code": 0,
            "msg": "旧密码错误"
        })));
    }
    
    let hashed_password = bcrypt::hash(new_password, 4)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("密码加密失败: {}", e)))?;
    
    sqlx::query("UPDATE tb_user SET password = ?, updated = NOW() WHERE id = ?")
        .bind(&hashed_password)
        .bind(id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "更新成功"
    })))
}

/// 绑定设备
pub async fn bind_boat(
    State(state): State<AppState>,
    Json(data): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = data.get("userId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少userId参数".to_string()))?;
    let node_id = data.get("nodeId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少nodeId参数".to_string()))?;
    
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO tb_user_node (id, userid, nodeid, removecode) VALUES (?, ?, ?, 1)"
    )
    .bind(&id)
    .bind(user_id)
    .bind(node_id)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "绑定成功"
    })))
}

/// 解除绑定设备
pub async fn unbind_boat(
    State(state): State<AppState>,
    Json(data): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = data.get("userId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少userId参数".to_string()))?;
    let node_id = data.get("nodeId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少nodeId参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_user_node WHERE userid = ? AND nodeid = ?")
        .bind(user_id)
        .bind(node_id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "解绑成功"
    })))
}