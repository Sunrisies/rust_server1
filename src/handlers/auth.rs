use sqlx::Row;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, ApiResponse};
use crate::routes::AppState;
use tracing::{info, error};

/// 登录请求
#[derive(Deserialize)]
pub struct LoginMessage {
    #[serde(rename = "userName", alias = "username", default)]
    pub username: String,
    #[serde(rename = "passWord", alias = "password", default)]
    pub password: String,
    #[serde(rename = "verifyCode", default)]
    pub verify_code: Option<String>,
    #[serde(rename = "userKey", default)]
    pub user_key: Option<String>,
}

/// 登录响应
#[derive(Serialize)]
pub struct LoginReturnMessage {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub username: String,
    #[serde(rename = "nickName")]
    pub nickname: Option<String>,
    pub logo: Option<String>,
    pub permission: Vec<serde_json::Value>,
    pub token: String,
    pub modules: Vec<serde_json::Value>,
    #[serde(rename = "history_url")]
    pub history_url: Option<String>,
    #[serde(rename = "realtime_url")]
    pub realtime_url: Option<String>,
    #[serde(rename = "workspaceId")]
    pub workspace_id: Option<String>,
}

/// 登录
pub async fn login(
    State(state): State<AppState>,
    Json(login_message): Json<LoginMessage>,
) -> Result<Json<serde_json::Value>, AppError> {
    // 参数检查
    if login_message.username.is_empty() || login_message.password.is_empty() {
        return Ok(Json(serde_json::json!({
            "code": 0,
            "msg": "参数异常..."
        })));
    }
    
    // Java 版本要求 passWord 必须是 RSA 加密内容，解密失败直接返回解析异常。
    let decrypted_pwd = match crate::rsa_util::get_rsa_util().decrypt(&login_message.password) {
        Ok(pwd) if !pwd.trim().is_empty() => pwd,
        _ => {
            return Ok(Json(serde_json::json!({
                "code": 0,
                "msg": "解析异常...",
                "data": null,
                "count": null,
                "obj": null,
            })));
        }
    };
    // 查询用户
    let user_row: Option<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT id, username, password, nickname, logo, moduleids, history_url, realtime_url, workspace_id FROM tb_user WHERE username = ?"
    )
    .bind(&login_message.username)
    .fetch_optional(&state.db.pool)
    .await?;
    
    let user_row = match user_row {
        Some(r) => r,
        None => {
            return Ok(Json(serde_json::json!({
                "code": 0,
                "msg": "当前账号不存在"
            })));
        }
    };
    
    // 提取字段
    let user_id: String = user_row.try_get("id")?;
    let username: String = user_row.try_get("username")?;
    let password: String = user_row.try_get("password")?;
    let nickname: Option<String> = user_row.try_get("nickname").ok();
    let logo: Option<String> = user_row.try_get("logo").ok();
    let moduleids: Option<String> = user_row.try_get("moduleids").ok();
    let history_url: Option<String> = user_row.try_get("history_url").ok();
    let realtime_url: Option<String> = user_row.try_get("realtime_url").ok();
    let workspace_id: Option<String> = user_row.try_get("workspace_id").ok();
    info!("查询{},---{}",password,decrypted_pwd);
    
    // 验证密码
    let password_valid = if password.starts_with("$2") {
        bcrypt::verify(&decrypted_pwd, &password).unwrap_or(false)
    } else {
        password == decrypted_pwd
    };
    
    if !password_valid {
        return Ok(Json(serde_json::json!({
            "code": 0,
            "msg": "密码错误"
        })));
    }
    
    // 生成token
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let token_content = format!("{},HOVERWEAPON,", now);
    let token = crate::rsa_util::get_rsa_util().encrypt(&token_content)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Token生成失败: {}", e)))?;
    
    // 保存session
    let now_str = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let token_message = crate::session::TokenMessage {
        token: token.clone(),
        establish_time: now_str,
        user_id: user_id.clone(),
        username: username.clone(),
        usertype: None,
        workspace_id: workspace_id.clone(),
        mqtt_username: None,
        mqtt_password: None,
    };
    let _ = crate::session::get_session().save_session(&token_message).await;
    
    // 查询用户模块
    let mut modules = Vec::new();
    if let Some(ref module_ids) = moduleids {
        for module_id in module_ids.split(',') {
            let module_id = module_id.trim();
            if !module_id.is_empty() {
                if let Ok(Some(m)) = sqlx::query_as::<_, (String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)>(
                    "SELECT id, modulename, moduledesc, moduleico, routeurl, removecode FROM tb_module WHERE id = ? AND removecode = '1'"
                )
                .bind(module_id)
                .fetch_optional(&state.db.pool)
                .await
                {
                    modules.push(serde_json::json!({
                        "id": m.0,
                        "modulename": m.1,
                        "moduledesc": m.2,
                        "moduleico": m.3,
                        "routeurl": m.4,
                        "removecode": m.5,
                    }));
                }
            }
        }
    }
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "获取成功",
        "data": {
            "userId": user_id,
            "userName": username,
            "nickName": nickname,
            "logo": logo,
            "permission": [],
            "token": token,
            "modules": modules,
            "history_url": history_url,
            "realtime_url": realtime_url,
            "workspaceId": workspace_id,
        }
    })))
}

/// 获取公钥
pub async fn get_public_key() -> Result<Json<serde_json::Value>, AppError> {
    let public_key = crate::rsa_util::get_rsa_util().get_public_key_base64();
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "公钥信息",
        "data": public_key,
        "obj": "key",
    })))
}

/// 验证token
pub async fn verify_token(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let token = params.get("token").map(|s| s.as_str()).unwrap_or("");
    
    if token.is_empty() {
        return Ok(Json(serde_json::json!({
            "success": false,
            "message": "token为空"
        })));
    }
    
    match crate::session::get_session().get_session(token).await {
        Ok(Some(_)) => {
            Ok(Json(serde_json::json!({
                "success": true,
                "message": "ok"
            })))
        }
        _ => {
            Ok(Json(serde_json::json!({
                "success": false,
                "message": "token无效或已过期"
            })))
        }
    }
}

/// 退出登录
pub async fn logout(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, AppError> {
    // Java 从请求头 token 读取；query 仅作为兼容兜底。
    let token = headers
        .get("token")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    
    if !token.is_empty() {
        if let Ok(Some(token_message)) = crate::session::get_session().get_session(token).await {
            let _ = crate::session::get_session().delete_session(&token_message).await;
        }
    }
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "登出成功！"
    })))
}

/// 注册
pub async fn register(
    State(state): State<AppState>,
    Json(user): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let username = user.get("username").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少username参数".to_string()))?;
    let password = user.get("password").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少password参数".to_string()))?;
    let nickname = user.get("nickname").and_then(|v| v.as_str());
    
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
        "INSERT INTO tb_user (id, username, password, nickname, created, updated) VALUES (?, ?, ?, ?, NOW(), NOW())"
    )
    .bind(&user_id)
    .bind(username)
    .bind(&hashed_password)
    .bind(nickname)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "注册成功",
        "data": {
            "user_id": user_id,
            "username": username,
        }
    })))
}