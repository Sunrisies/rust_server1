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
        .route("/node/detail", get(get_node))
        .route("/node/add", post(add_node))
        .route("/node/update", put(update_node))
        .route("/node/delete", delete(delete_node))
        .route("/node/updateLocation", put(update_node_location))
        .route("/node/queryAll", get(list_nodes))  // 兼容旧接口
        
        // 船只参数相关
        .route("/node/parameter/list", get(list_node_parameters))
        .route("/node/parameter/update", post(update_node_parameter))
        
        // 用户船只关联
        .route("/user/node/list", get(list_user_nodes))
        
        // 区块任务相关
        .route("/blockPlan/list", get(list_block_plans))
        .route("/blockPlan/add", post(add_block_plan))
        .route("/blockPlan/update", put(update_block_plan))
        .route("/blockPlan/delete", delete(delete_block_plan))
        
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
    pub describes: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct NodeDetail {
    pub id: String,
    pub sname: String,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub url: Option<String>,
    pub state: Option<String>,
    pub task_id: Option<String>,
    pub tcp_port: Option<String>,
    pub describes: Option<String>,
    pub remote_port: Option<String>,
}

async fn list_nodes(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<NodeItem>>>, AppError> {
    let nodes: Vec<NodeItem> = sqlx::query_as(
        "SELECT id, sname, ip, port, state, taskid as task_id, describes FROM tb_node"
    )
    .fetch_all(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse::success(nodes)))
}

async fn get_node(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<NodeDetail>>, AppError> {
    let id = params.get("id")
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;

    let node: NodeDetail = sqlx::query_as(
        "SELECT id, sname, ip, port, url, state, taskid as task_id, tcpPort as tcp_port, describes, remote_port FROM tb_node WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("船只不存在".to_string()))?;

    Ok(Json(ApiResponse::success(node)))
}

#[derive(Deserialize)]
pub struct AddNodeRequest {
    pub sname: String,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub describes: Option<String>,
}

async fn add_node(
    State(state): State<AppState>,
    Json(req): Json<AddNodeRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let node_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO tb_node (id, sname, ip, port, state, describes) VALUES (?, ?, ?, ?, 'offline', ?)"
    )
    .bind(&node_id)
    .bind(&req.sname)
    .bind(&req.ip)
    .bind(&req.port)
    .bind(&req.describes)
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
    pub describes: Option<String>,
}

async fn update_node(
    State(state): State<AppState>,
    Json(req): Json<UpdateNodeRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    sqlx::query(
        "UPDATE tb_node SET sname = COALESCE(?, sname), ip = COALESCE(?, ip), port = COALESCE(?, port), state = COALESCE(?, state), describes = COALESCE(?, describes) WHERE id = ?"
    )
    .bind(&req.sname)
    .bind(&req.ip)
    .bind(&req.port)
    .bind(&req.state)
    .bind(&req.describes)
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

#[derive(Deserialize)]
pub struct UpdateLocationRequest {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
}

async fn update_node_location(
    State(state): State<AppState>,
    Json(req): Json<UpdateLocationRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    // 插入位置记录 (使用实际表结构: id, latitude, longitude, update_time)
    let location_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO tb_node_location (id, latitude, longitude, update_time) VALUES (?, ?, ?, NOW())"
    )
    .bind(&location_id)
    .bind(req.latitude)
    .bind(req.longitude)
    .execute(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "updated": true,
    }))))
}

// ==================== 船只参数相关 ====================

#[derive(Debug, FromRow, Serialize)]
pub struct NodeParameterItem {
    pub id: String,
    pub node_id: String,
    pub name: String,
    pub value: Option<String>,
    pub code: Option<String>,
}

async fn list_node_parameters(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    let node_id = params.get("nodeId")
        .ok_or_else(|| AppError::BadRequest("缺少nodeId参数".to_string()))?;

    // 使用实际表结构: id, nodeid, npid, removecode
    let rows: Vec<(String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, nodeid, npid FROM tb_node_and_node_parameter WHERE nodeid = ?"
    )
    .bind(node_id)
    .fetch_all(&state.db.pool)
    .await?;

    // 转换为统一格式
    let result: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.0,
            "node_id": r.1,
            "npid": r.2,
        })
    }).collect();

    Ok(Json(ApiResponse::success(result)))
}

#[derive(Deserialize)]
pub struct UpdateParameterRequest {
    pub id: Option<String>,
    pub node_id: String,
    pub name: String,
    pub value: Option<String>,
    pub code: Option<String>,
}

async fn update_node_parameter(
    State(state): State<AppState>,
    Json(req): Json<UpdateParameterRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    if let Some(id) = &req.id {
        // 更新现有参数
        sqlx::query(
            "UPDATE tb_node_and_node_parameter SET value = ?, code = ? WHERE id = ?"
        )
        .bind(&req.value)
        .bind(&req.code)
        .bind(id)
        .execute(&state.db.pool)
        .await?;
    } else {
        // 插入新参数
        let param_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO tb_node_and_node_parameter (id, nodeid, name, value, code) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&param_id)
        .bind(&req.node_id)
        .bind(&req.name)
        .bind(&req.value)
        .bind(&req.code)
        .execute(&state.db.pool)
        .await?;
    }

    Ok(Json(ApiResponse::success(serde_json::json!({
        "updated": true,
    }))))
}

// ==================== 用户船只关联 ====================

#[derive(Debug, FromRow, Serialize)]
pub struct UserNodeItem {
    pub id: String,
    pub user_id: String,
    pub node_id: String,
    pub node_name: Option<String>,
}

async fn list_user_nodes(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    let user_id = params.get("userId")
        .ok_or_else(|| AppError::BadRequest("缺少userId参数".to_string()))?;

    // 使用实际表结构: id, userid, nodeid, removecode
    let rows: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT un.id, un.userid, un.nodeid 
         FROM tb_user_node un 
         WHERE un.userid = ?"
    )
    .bind(user_id)
    .fetch_all(&state.db.pool)
    .await?;

    // 获取船只名称
    let mut result = Vec::new();
    for r in rows {
        let node_name: Option<String> = sqlx::query_scalar(
            "SELECT sname FROM tb_node WHERE id = ?"
        )
        .bind(&r.2)
        .fetch_optional(&state.db.pool)
        .await?;
        
        result.push(serde_json::json!({
            "id": r.0,
            "user_id": r.1,
            "node_id": r.2,
            "node_name": node_name,
        }));
    }

    Ok(Json(ApiResponse::success(result)))
}

// ==================== 区块任务相关 ====================

#[derive(Debug, FromRow, Serialize)]
pub struct BlockPlanItem {
    pub id: String,
    pub boat_id: Option<String>,
    pub task_name: Option<String>,
    pub course_spacing: Option<f64>,
    pub course_angle: Option<f64>,
    pub complete_action: Option<String>,
    pub add_time: Option<String>,
}

async fn list_block_plans(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    let boat_id = params.get("boatId");
    
    // 简单查询，只返回基本信息
    let rows: Vec<(String, Option<String>, Option<String>)> = if let Some(boat_id) = boat_id {
        sqlx::query_as(
            "SELECT id, boat_id, task_name FROM block_plan WHERE boat_id = ?"
        )
        .bind(boat_id)
        .fetch_all(&state.db.pool)
        .await?
    } else {
        sqlx::query_as(
            "SELECT id, boat_id, task_name FROM block_plan"
        )
        .fetch_all(&state.db.pool)
        .await?
    };
    
    let plans: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.0,
            "boat_id": r.1,
            "task_name": r.2,
        })
    }).collect();

    Ok(Json(ApiResponse::success(plans)))
}

#[derive(Deserialize)]
pub struct AddBlockPlanRequest {
    pub boat_id: String,
    pub task_name: Option<String>,
    pub create_address: Option<String>,
    pub route: Option<String>,
    pub polygon: Option<String>,
    pub course_spacing: Option<f64>,
    pub course_angle: Option<f64>,
    pub complete_action: Option<String>,
}

async fn add_block_plan(
    State(state): State<AppState>,
    Json(req): Json<AddBlockPlanRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let plan_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO block_plan (id, boat_id, task_name, create_address, route, polygon, course_spacing, course_angle, complete_action, add_time) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, NOW())"
    )
    .bind(&plan_id)
    .bind(&req.boat_id)
    .bind(&req.task_name)
    .bind(&req.create_address)
    .bind(&req.route)
    .bind(&req.polygon)
    .bind(&req.course_spacing)
    .bind(&req.course_angle)
    .bind(&req.complete_action)
    .execute(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "plan_id": plan_id,
    }))))
}

#[derive(Deserialize)]
pub struct UpdateBlockPlanRequest {
    pub id: String,
    pub task_name: Option<String>,
    pub route: Option<String>,
    pub polygon: Option<String>,
    pub course_spacing: Option<f64>,
    pub course_angle: Option<f64>,
    pub complete_action: Option<String>,
}

async fn update_block_plan(
    State(state): State<AppState>,
    Json(req): Json<UpdateBlockPlanRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    sqlx::query(
        "UPDATE block_plan SET 
         task_name = COALESCE(?, task_name),
         route = COALESCE(?, route),
         polygon = COALESCE(?, polygon),
         course_spacing = COALESCE(?, course_spacing),
         course_angle = COALESCE(?, course_angle),
         complete_action = COALESCE(?, complete_action),
         update_time = NOW()
         WHERE id = ?"
    )
    .bind(&req.task_name)
    .bind(&req.route)
    .bind(&req.polygon)
    .bind(&req.course_spacing)
    .bind(&req.course_angle)
    .bind(&req.complete_action)
    .bind(&req.id)
    .execute(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "updated": true,
    }))))
}

async fn delete_block_plan(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let id = params.get("id")
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;

    sqlx::query("DELETE FROM block_plan WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "deleted": true,
    }))))
}
