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
use crate::handlers::{auth, user, node, parameter, permission, module, whitelist, app, node_function, node_permission, block_plan, dockyard, mango, file_upload};

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
        
        // ==================== 认证模块 (/login/register/*) ====================
        .route("/login/register/login", post(auth::login))
        .route("/login/register/getPublicKey", get(auth::get_public_key))
        .route("/login/register/verifyToken", post(auth::verify_token))
        .route("/login/register/logout", post(auth::logout))
        .route("/login/register/register", post(auth::register))
        
        // ==================== 用户管理 (/admin/user/*) ====================
        .route("/admin/user/queryAll", post(user::query_all))
        .route("/admin/user/add", post(user::add))
        .route("/admin/user/update", post(user::update))
        .route("/admin/user/deleteUser", post(user::delete))
        .route("/admin/user/update/account/profile", post(user::update_profile))
        .route("/admin/user/update/account/password", post(user::update_password))
        .route("/admin/user/update/account/profileAndPasswrd", post(user::update_profile_and_password))
        .route("/admin/user/bindBoat", post(user::bind_boat))
        .route("/admin/user/notBindboat", post(user::unbind_boat))
        .route("/admin/user/bindCommonPermission", post(user::bind_common_permission))
        .route("/admin/user/notBindCommonPermission", post(user::unbind_common_permission))
        .route("/admin/user/queryUserByUserName", post(user::query_by_username))
        .route("/admin/user/queryUserByUserNameX", post(user::query_user_detail))
        
        // ==================== 船只管理 (/admin/node/*) ====================
        .route("/admin/node/queryAllNode", post(node::query_all))
        .route("/admin/node/queryNodeAll", post(node::query_all))
        .route("/admin/node/queryNodeByUser", post(node::query_by_user))
        .route("/admin/node/addNode", post(node::add))
        .route("/admin/node/updataNode", post(node::update))
        .route("/admin/node/deteleNode", post(node::delete))
        .route("/admin/node/updateNodeLocation", post(node::update_location))
        
        // ==================== 参数管理 (/admin/parameter/*) ====================
        .route("/admin/parameter/queryParameter", post(parameter::query))
        .route("/admin/parameter/queryParameterAll", post(parameter::query))
        .route("/admin/parameter/addParameter", post(parameter::add))
        .route("/admin/parameter/updateParameter", post(parameter::update))
        .route("/admin/parameter/deleteParameter", post(parameter::delete))
        .route("/admin/parameter/bindingParameterAndNode", post(parameter::bind))
        .route("/admin/parameter/unbindParameterAndNode", post(parameter::unbind))
        
        // ==================== 权限管理 (/admin/permission/*) ====================
        .route("/admin/permission/add", post(permission::add))
        .route("/admin/permission/updatePermission", post(permission::update))
        .route("/admin/permission/deletePermission", post(permission::delete))
        .route("/admin/permission/queryAll", post(permission::query_all))
        
        // ==================== 模块管理 (/admin/module/*) ====================
        .route("/admin/module/add", post(module::add))
        .route("/admin/module/updateModule", post(module::update))
        .route("/admin/module/deleteModule", post(module::delete))
        .route("/admin/module/queryAll", post(module::query_all))
        .route("/admin/module/addModulePermission", post(module::add_permission))
        .route("/admin/module/deleteModulePermission", post(module::delete_permission))
        .route("/admin/module/queryModulePermission", post(module::query_permissions))
        
        // ==================== 白名单管理 (/admin/whitelist/*) ====================
        .route("/admin/whitelist/add", post(whitelist::add))
        .route("/admin/whitelist/remove", post(whitelist::remove))
        .route("/admin/whitelist/queryAll", post(whitelist::query_all))
        
        // ==================== APP管理 (/admin/app/*) ====================
        .route("/admin/app/addRecord", post(app::add_record))
        .route("/admin/app/deleteRecord", post(app::delete_record))
        .route("/admin/app/queryAll", post(app::query_all))
        .route("/admin/app/queryNewVersion", post(app::query_new_version))
        
        // ==================== 船只功能管理 (/admin/nodeFuncation/*) ====================
        .route("/admin/nodeFuncation/addNodeFuncation", post(node_function::add))
        .route("/admin/nodeFuncation/updateNodeFuncation", post(node_function::update))
        .route("/admin/nodeFuncation/deleteNodeFuncation", post(node_function::delete))
        .route("/admin/nodeFuncation/queryNodeFuncation", post(node_function::query))
        .route("/admin/nodeFuncation/queryNodeFuncationById", post(node_function::query_by_node_id))
        .route("/admin/nodeFuncation/addNodeFunAndNode", post(node_function::add_binding))
        .route("/admin/nodeFuncation/deleteNodeFunAndNodeByNodeIdAndFunId", post(node_function::delete_binding))
        
        // ==================== 船只权限管理 (/admin/nodePermissions/*) ====================
        .route("/admin/nodePermissions/addNodePermission", post(node_permission::add))
        .route("/admin/nodePermissions/updateNodePermission", post(node_permission::update))
        .route("/admin/nodePermissions/deleteNodePermission", post(node_permission::delete))
        .route("/admin/nodePermissions/queryNodePermissionsAll", post(node_permission::query_all))
        .route("/admin/nodePermissions/queryNodePermissionsByNodeId", post(node_permission::query_by_node_id))
        .route("/admin/nodePermissions/addNodePerAndNode", post(node_permission::add_binding))
        .route("/admin/nodePermissions/deleteNodePerAndNodeByNodeIdAndPerId", post(node_permission::delete_binding))
        
        // ==================== 区块任务 (/blockPlan/*) ====================
        .route("/blockPlan/addOrUpdateBlockPlan", post(block_plan::add_or_update))
        .route("/blockPlan/deleteBlockPlan", post(block_plan::delete))
        .route("/blockPlan/queryBlockPlanById", post(block_plan::query_by_id))
        .route("/blockPlan/queryBlockPlanByBoatId", post(block_plan::query_by_boat_id))
        
        // ==================== 船坞任务 (/mavlinkOperation/*) ====================
        .route("/mavlinkOperation/addDockyard", post(dockyard::add))
        .route("/mavlinkOperation/queryDockyard", post(dockyard::query))
        .route("/mavlinkOperation/querySingleDockyard", post(dockyard::query_single))
        .route("/mavlinkOperation/deleteDockyard", post(dockyard::delete))
        .route("/mavlinkOperation/updateDockyard", post(dockyard::update))
        
        // ==================== MongoDB (/mango/*) ====================
        .route("/mango/find", post(mango::find))
        .route("/mango/findHistoricalData", post(mango::find_historical_data))
        .route("/mango/findNum", post(mango::find_num))
        .route("/mango/shell", post(mango::shell))
        
        // ==================== 文件上传 (/fileserver/upload/*) ====================
        .route("/fileserver/upload/uploadLogo", post(file_upload::upload_logo))
        .route("/fileserver/upload/uploadIco", post(file_upload::upload_ico))
        .route("/fileserver/upload/uploadApp", post(file_upload::upload_app))
        
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