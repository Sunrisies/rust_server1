use axum::{
    extract::State,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::database::Database;
use crate::error::{AppError, ApiResponse};
use crate::jwt::JwtUtil;
use crate::redis::Redis;

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub redis: Redis,
    pub jwt: JwtUtil,
}

/// 创建路由
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 健康检查
        .route("/health", get(health_check))
        .route("/api/health", get(health_check))
        
        // 认证相关
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        
        // 用户相关
        .route("/user/list", get(list_users))
        .route("/user/add", post(add_user))
        .route("/user/update", put(update_user))
        .route("/user/delete", delete(delete_user))
        
        // 船只相关
        .route("/node/list", get(list_nodes))
        .route("/node/add", post(add_node))
        .route("/node/update", put(update_node))
        .route("/node/delete", delete(delete_node))
        
        .with_state(state)
}

// ==================== 健康检查 ====================

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "rust_hover",
        "version": "0.1.0"
    }))
}

// ==================== 认证相关 ====================

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
    pub username: String,
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // 查询用户
    let user: (String, String, String) = sqlx::query_as(
        "SELECT id, username, password FROM tb_user WHERE username = ?"
    )
    .bind(&req.username)
    .fetch_optional(&state.db.pool)
    .await?
    .ok_or_else(|| AppError::Auth("用户不存在".to_string()))?;

    // 验证密码 (支持BCrypt和明文)
    let password_valid = if user.2.starts_with("$2") {
        // BCrypt格式的密码
        bcrypt::verify(&req.password, &user.2).unwrap_or(false)
    } else {
        // 明文密码（兼容旧数据）
        user.2 == req.password
    };
    
    if !password_valid {
        return Err(AppError::Auth("密码错误".to_string()));
    }

    // 生成Token
    let token = state.jwt.generate_token(&user.0, &user.1)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Token生成失败: {}", e)))?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.0,
        username: user.1,
    }))
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub nickname: Option<String>,
}

async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    // 检查用户名是否已存在
    let exists: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0 FROM tb_user WHERE username = ?"
    )
    .bind(&req.username)
    .fetch_one(&state.db.pool)
    .await?;

    if exists {
        return Err(AppError::BadRequest("用户名已存在".to_string()));
    }

    // 生成用户ID
    let user_id = uuid::Uuid::new_v4().to_string();

    // 插入用户
    sqlx::query(
        "INSERT INTO tb_user (id, username, password, nickname, created) VALUES (?, ?, ?, ?, NOW())"
    )
    .bind(&user_id)
    .bind(&req.username)
    .bind(&req.password)
    .bind(&req.nickname)
    .execute(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "user_id": user_id,
        "username": req.username,
    }))))
}

// ==================== 用户相关 ====================

#[derive(Debug, FromRow, Serialize)]
pub struct UserItem {
    pub id: String,
    pub username: String,
    pub nickname: Option<String>,
    pub phone: Option<String>,
}

async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<UserItem>>>, AppError> {
    let users: Vec<UserItem> = sqlx::query_as(
        "SELECT id, username, nickname, phone FROM tb_user"
    )
    .fetch_all(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse::success(users)))
}

#[derive(Deserialize)]
pub struct AddUserRequest {
    pub username: String,
    pub password: String,
    pub nickname: Option<String>,
    pub phone: Option<String>,
}

async fn add_user(
    State(state): State<AppState>,
    Json(req): Json<AddUserRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let user_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO tb_user (id, username, password, nickname, phone, created) VALUES (?, ?, ?, ?, ?, NOW())"
    )
    .bind(&user_id)
    .bind(&req.username)
    .bind(&req.password)
    .bind(&req.nickname)
    .bind(&req.phone)
    .execute(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "user_id": user_id,
    }))))
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub id: String,
    pub nickname: Option<String>,
    pub phone: Option<String>,
}

async fn update_user(
    State(state): State<AppState>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    sqlx::query(
        "UPDATE tb_user SET nickname = ?, phone = ?, updated = NOW() WHERE id = ?"
    )
    .bind(&req.nickname)
    .bind(&req.phone)
    .bind(&req.id)
    .execute(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "updated": true,
    }))))
}

async fn delete_user(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let id = params.get("id")
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;

    sqlx::query("DELETE FROM tb_user WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "deleted": true,
    }))))
}

// ==================== 船只相关 ====================

#[derive(Debug, FromRow, Serialize)]
pub struct NodeItem {
    pub id: String,
    pub sname: String,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub state: Option<String>,
    pub task_id: Option<String>,
}

async fn list_nodes(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<NodeItem>>>, AppError> {
    let nodes: Vec<NodeItem> = sqlx::query_as(
        "SELECT id, sname, ip, port, state, taskid as task_id FROM tb_node"
    )
    .fetch_all(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse::success(nodes)))
}

#[derive(Deserialize)]
pub struct AddNodeRequest {
    pub sname: String,
    pub ip: Option<String>,
    pub port: Option<String>,
}

async fn add_node(
    State(state): State<AppState>,
    Json(req): Json<AddNodeRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let node_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO tb_node (id, sname, ip, port, state) VALUES (?, ?, ?, ?, 'offline')"
    )
    .bind(&node_id)
    .bind(&req.sname)
    .bind(&req.ip)
    .bind(&req.port)
    .execute(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "node_id": node_id,
    }))))
}

#[derive(Deserialize)]
pub struct UpdateNodeRequest {
    pub id: String,
    pub sname: Option<String>,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub state: Option<String>,
}

async fn update_node(
    State(state): State<AppState>,
    Json(req): Json<UpdateNodeRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    sqlx::query(
        "UPDATE tb_node SET sname = COALESCE(?, sname), ip = COALESCE(?, ip), port = COALESCE(?, port), state = COALESCE(?, state) WHERE id = ?"
    )
    .bind(&req.sname)
    .bind(&req.ip)
    .bind(&req.port)
    .bind(&req.state)
    .bind(&req.id)
    .execute(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "updated": true,
    }))))
}

async fn delete_node(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let id = params.get("id")
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;

    sqlx::query("DELETE FROM tb_node WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "deleted": true,
    }))))
}
