use sqlx::Row;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, ApiResponse};
use crate::routes::AppState;

/// 添加模块
pub async fn add(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let modulename = params.get("modulename").and_then(|v| v.as_str()).unwrap_or("");
    let moduledesc = params.get("moduledesc").and_then(|v| v.as_str());
    let moduleico = params.get("moduleico").and_then(|v| v.as_str());
    let routeurl = params.get("routeurl").and_then(|v| v.as_str());
    
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO tb_module (id, modulename, moduledesc, moduleico, routeurl, removecode) VALUES (?, ?, ?, ?, ?, '1')"
    )
    .bind(&id)
    .bind(modulename)
    .bind(moduledesc)
    .bind(moduleico)
    .bind(routeurl)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "添加成功"
    })))
}

/// 更新模块
pub async fn update(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    let modulename = params.get("modulename").and_then(|v| v.as_str());
    let moduledesc = params.get("moduledesc").and_then(|v| v.as_str());
    let moduleico = params.get("moduleico").and_then(|v| v.as_str());
    let routeurl = params.get("routeurl").and_then(|v| v.as_str());
    
    sqlx::query(
        "UPDATE tb_module SET modulename = COALESCE(?, modulename), moduledesc = COALESCE(?, moduledesc), moduleico = COALESCE(?, moduleico), routeurl = COALESCE(?, routeurl) WHERE id = ?"
    )
    .bind(modulename)
    .bind(moduledesc)
    .bind(moduleico)
    .bind(routeurl)
    .bind(id)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "更新成功"
    })))
}

/// 删除模块
pub async fn delete(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_module WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "删除成功"
    })))
}

/// 查询所有模块
pub async fn query_all(
    State(state): State<AppState>,
    Json(_params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT id, modulename, moduledesc, moduleico, routeurl, removecode FROM tb_module WHERE removecode = '1'"
    )
    .fetch_all(&state.db.pool)
    .await?;
    
    let result: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "modulename": r.try_get::<Option<String>, _>("modulename").ok().flatten(),
            "moduledesc": r.try_get::<Option<String>, _>("moduledesc").ok().flatten(),
            "moduleico": r.try_get::<Option<String>, _>("moduleico").ok().flatten(),
            "routeurl": r.try_get::<Option<String>, _>("routeurl").ok().flatten(),
            "removecode": r.try_get::<Option<String>, _>("removecode").ok().flatten(),
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

/// 添加模块权限
pub async fn add_permission(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mid = params.get("mid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少mid参数".to_string()))?;
    let pid = params.get("pid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少pid参数".to_string()))?;
    
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO tb_module_permission (id, mid, pid) VALUES (?, ?, ?)"
    )
    .bind(&id)
    .bind(mid)
    .bind(pid)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "添加成功"
    })))
}

/// 删除模块权限
pub async fn delete_permission(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mid = params.get("mid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少mid参数".to_string()))?;
    let pid = params.get("pid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少pid参数".to_string()))?;
    
    sqlx::query("DELETE FROM tb_module_permission WHERE mid = ? AND pid = ?")
        .bind(mid)
        .bind(pid)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "删除成功"
    })))
}

/// 查询模块权限
pub async fn query_permissions(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mid = params.get("mid").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少mid参数".to_string()))?;
    
    let rows: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT p.id, p.mname, p.mdesc, p.uri, p.method FROM tb_permission p INNER JOIN tb_module_permission mp ON p.id = mp.pid WHERE mp.mid = ?"
    )
    .bind(mid)
    .fetch_all(&state.db.pool)
    .await?;
    
    let result: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "mname": r.try_get::<Option<String>, _>("mname").ok().flatten(),
            "mdesc": r.try_get::<Option<String>, _>("mdesc").ok().flatten(),
            "uri": r.try_get::<Option<String>, _>("uri").ok().flatten(),
            "method": r.try_get::<Option<String>, _>("method").ok().flatten(),
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