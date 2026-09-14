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
    
    // 与Java版本一致的SQL查询字段
    let mut query = String::from(
        "SELECT id, username, moduleids, logo, phone, created, updated, nickname, img, remarks,
         user_type as userType, workspace_id as workspaceId, mqtt_username as mqttUsername,
         mqtt_password as mqttPassword, history_url, realtime_url
         FROM tb_user WHERE removecode = 1"
    );
    let mut bind_values: Vec<String> = Vec::new();
    
    // 与 ManageUserMapper.xml 一致：keywords 同时匹配用户名、昵称和备注。
    if let Some(kw) = keywords {
        if !kw.is_empty() {
            query.push_str(" AND (username LIKE ? OR nickname LIKE ? OR remarks LIKE ?)");
            let pattern = format!("%{}%", kw);
            bind_values.push(pattern.clone());
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
        let created = r.try_get::<Option<chrono::NaiveDateTime>, _>("created")
            .ok().flatten().map(|v| v.format("%Y-%m-%d %H:%M:%S").to_string());
        let updated = r.try_get::<Option<chrono::NaiveDateTime>, _>("updated")
            .ok().flatten().map(|v| v.format("%Y-%m-%d %H:%M:%S").to_string());
        let module_ids = r.try_get::<Option<String>, _>("moduleids").ok().flatten();
        
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "username": r.try_get::<Option<String>, _>("username").ok().flatten(),
            "moduleids": module_ids,
            "modulenames": null,
            "logo": r.try_get::<Option<String>, _>("logo").ok().flatten(),
            "phone": r.try_get::<Option<String>, _>("phone").ok().flatten(),
            "created": created,
            "updated": updated,
            "nickname": r.try_get::<Option<String>, _>("nickname").ok().flatten(),
            "img": r.try_get::<Option<String>, _>("img").ok().flatten(),
            "remarks": r.try_get::<Option<String>, _>("remarks").ok().flatten(),
            "removecode": 1,
            "userType": r.try_get::<Option<i32>, _>("userType").ok().flatten(),
            "workspaceId": r.try_get::<Option<String>, _>("workspaceId").ok().flatten(),
            "mqttUsername": r.try_get::<Option<String>, _>("mqttUsername").ok().flatten(),
            "mqttPassword": r.try_get::<Option<String>, _>("mqttPassword").ok().flatten(),
            "history_url": r.try_get::<Option<String>, _>("history_url").ok().flatten(),
            "realtime_url": r.try_get::<Option<String>, _>("realtime_url").ok().flatten(),
            "page": null,
            "limit": null,
            "keywords": null,
            "createTime": created,
            "updateTime": updated,
        })
    }).collect();
    
    // 查询总数（不带分页）
    let mut count_query = String::from(
        "SELECT COUNT(*) as total FROM tb_user WHERE removecode = 1"
    );
    let mut count_bind_values: Vec<String> = Vec::new();
    
    if let Some(kw) = params.get("keywords").and_then(|v| v.as_str()) {
        if !kw.is_empty() {
            count_query.push_str(" AND (username LIKE ? OR nickname LIKE ? OR remarks LIKE ?)");
            let pattern = format!("%{}%", kw);
            count_bind_values.push(pattern.clone());
            count_bind_values.push(pattern.clone());
            count_bind_values.push(pattern);
        }
    }
    
    let total_count: i32 = if count_bind_values.is_empty() {
        sqlx::query_scalar(&count_query)
            .fetch_one(&state.db.pool)
            .await?
    } else {
        let mut q = sqlx::query_scalar(&count_query);
        for v in &count_bind_values {
            q = q.bind(v);
        }
        q.fetch_one(&state.db.pool).await?
    };
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "",
        "data": users,
        "count": total_count,
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
    
    // Java update 先检查用户名是否被其他用户占用。
    if let Some(name) = username {
        let duplicate: bool = sqlx::query_scalar(
            "SELECT COUNT(*) > 0 FROM tb_user WHERE username = ? AND id <> ?"
        )
        .bind(name)
        .bind(id)
        .fetch_one(&state.db.pool)
        .await?;
        if duplicate {
            return Ok(Json(serde_json::json!({
                "code": 0,
                "msg": "用户名已被其他用户使用"
            })));
        }
    }
    
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

/// 更新用户资料和密码
pub async fn update_profile_and_password(
    State(state): State<AppState>,
    Json(user): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    // 先更新资料
    let result = update_profile(State(state.clone()), Json(user.clone())).await?;
    
    // 如果有新密码，更新密码
    if let Some(password) = user.get("password").and_then(|v| v.as_str()) {
        if !password.is_empty() {
            let id = user.get("id").and_then(|v| v.as_str())
                .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
            
            let hashed_password = bcrypt::hash(password, 4)
                .map_err(|e| AppError::Internal(anyhow::anyhow!("密码加密失败: {}", e)))?;
            
            sqlx::query("UPDATE tb_user SET password = ?, updated = NOW() WHERE id = ?")
                .bind(&hashed_password)
                .bind(id)
                .execute(&state.db.pool)
                .await?;
        }
    }
    
    Ok(result)
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

/// 绑定公共权限
pub async fn bind_common_permission(
    State(state): State<AppState>,
    Json(data): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = data.get("userId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少userId参数".to_string()))?;
    let permission_id = data.get("permissionId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少permissionId参数".to_string()))?;
    
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO tb_user_and_cpermission (id, uid, cpid) VALUES (?, ?, ?)"
    )
    .bind(&id)
    .bind(user_id)
    .bind(permission_id)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "绑定成功"
    })))
}

/// 解除绑定公共权限
pub async fn unbind_common_permission(
    State(state): State<AppState>,
    Json(data): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = data.get("userId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少userId参数".to_string()))?;
    let permission_id = data.get("permissionId").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少permissionId参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_user_and_cpermission WHERE uid = ? AND cpid = ?")
        .bind(user_id)
        .bind(permission_id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "解绑成功"
    })))
}

/// 根据用户名查询用户
pub async fn query_by_username(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let username = params.get("username").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少username参数".to_string()))?;
    
    let users: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT id, username, nickname, phone, logo FROM tb_user WHERE username = ?"
    )
    .bind(username)
    .fetch_all(&state.db.pool)
    .await?;
    
    let result: Vec<serde_json::Value> = users.iter().map(|r| {
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "username": r.try_get::<Option<String>, _>("username").ok().flatten(),
            "nickname": r.try_get::<Option<String>, _>("nickname").ok().flatten(),
            "phone": r.try_get::<Option<String>, _>("phone").ok().flatten(),
            "logo": r.try_get::<Option<String>, _>("logo").ok().flatten(),
        })
    }).collect();
    
    // Java queryUserByUserName 返回裸 List，不包 LayUiTableTemplateLay。
    Ok(Json(serde_json::Value::Array(result)))
}

/// 获取登录用户详细信息
pub async fn query_user_detail(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let username = params.get("username").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少username参数".to_string()))?;
    
    let users: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT id, username, nickname, phone, logo, moduleids, remarks FROM tb_user WHERE username = ?"
    )
    .bind(username)
    .fetch_all(&state.db.pool)
    .await?;
    
    let result: Vec<serde_json::Value> = users.iter().map(|r| {
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
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "查询成功",
        "data": result,
    })))
}