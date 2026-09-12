use axum::{
    extract::State,
    http::{header, Method, StatusCode},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use tower_http::cors::{CorsLayer, Any};

use crate::database::Database;
use crate::error::{AppError, ApiResponse};
use crate::jwt::JwtUtil;
use crate::redis::Redis;
use crate::handlers::{auth, user, node, parameter, permission, module};

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub redis: Redis,
    pub jwt: JwtUtil,
    pub mongodb: Option<crate::mongodb::MongoDB>,
}

/// 创建路由（完全匹配Java版本）
pub fn create_router(state: AppState) -> Router {
    // 配置CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .layer(cors)
        
        // ==================== 认证模块 (/register/*) ====================
        .route("/register/login", post(auth::login))
        .route("/register/getPublicKey", get(auth::get_public_key))
        .route("/register/verifyToken", post(auth::verify_token))
        .route("/register/logout", post(auth::logout))
        .route("/register/register", post(auth::register))
        
        // ==================== 用户管理 (/user/*) ====================
        .route("/user/queryAll", post(user::query_all))
        .route("/user/add", post(user::add))
        .route("/user/update", post(user::update))
        .route("/user/deleteUser", post(user::delete))
        .route("/user/update/account/profile", post(user::update_profile))
        .route("/user/update/account/password", post(user::update_password))
        .route("/user/bindBoat", post(user::bind_boat))
        .route("/user/notBindboat", post(user::unbind_boat))
        
        // ==================== 船只管理 (/node/*) ====================
        .route("/node/queryAllNode", post(node::query_all))
        .route("/node/queryNodeAll", post(node::query_all))
        .route("/node/queryNodeByUser", post(node::query_by_user))
        .route("/node/addNode", post(node::add))
        .route("/node/updataNode", post(node::update))
        .route("/node/deteleNode", post(node::delete))
        .route("/node/updateNodeLocation", post(node::update_location))
        
        // ==================== 参数管理 (/parameter/*) ====================
        .route("/parameter/queryParameter", post(parameter::query))
        .route("/parameter/queryParameterAll", post(parameter::query))
        .route("/parameter/addParameter", post(parameter::add))
        .route("/parameter/updateParameter", post(parameter::update))
        .route("/parameter/deleteParameter", post(parameter::delete))
        .route("/parameter/bindingParameterAndNode", post(parameter::bind))
        .route("/parameter/unbindParameterAndNode", post(parameter::unbind))
        
        // ==================== 权限管理 (/permission/*) ====================
        .route("/permission/add", post(permission::add))
        .route("/permission/updatePermission", post(permission::update))
        .route("/permission/deletePermission", post(permission::delete))
        .route("/permission/queryAll", post(permission::query_all))
        
        // ==================== 模块管理 (/module/*) ====================
        .route("/module/add", post(module::add))
        .route("/module/updateModule", post(module::update))
        .route("/module/deleteModule", post(module::delete))
        .route("/module/queryAll", post(module::query_all))
        .route("/module/addModulePermission", post(module::add_permission))
        .route("/module/deleteModulePermission", post(module::delete_permission))
        .route("/module/queryModulePermission", post(module::query_permissions))
        
        // ==================== 健康检查 ====================
        .route("/health", get(health_check))
        
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