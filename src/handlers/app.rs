use axum::{extract::State, Json};
use sqlx::Row;

use crate::error::AppError;
use crate::routes::AppState;

/// 添加APP记录
pub async fn add_record(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let appdesc = params.get("appdesc").and_then(|v| v.as_str());
    let apptype = params.get("apptype").and_then(|v| v.as_str());
    let version = params.get("version").and_then(|v| v.as_str());
    let downloadurl = params.get("downloadurl").and_then(|v| v.as_str());
    
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO tb_app_record (id, appdesc, apptype, version, downloadurl, createtime, updatetime, removecode) VALUES (?, ?, ?, ?, ?, NOW(), NOW(), '1')"
    )
    .bind(&id)
    .bind(appdesc)
    .bind(apptype)
    .bind(version)
    .bind(downloadurl)
    .execute(&state.db.pool)
    .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "添加成功"
    })))
}

/// 删除APP记录
pub async fn delete_record(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = params.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("缺少id参数".to_string()))?;
    
    sqlx::query("UPDATE tb_app_record SET removecode = '0' WHERE id = ?")
        .bind(id)
        .execute(&state.db.pool)
        .await?;
    
    Ok(Json(serde_json::json!({
        "code": 1,
        "msg": "删除成功"
    })))
}

/// 查询所有APP记录
pub async fn query_all(
    State(state): State<AppState>,
    Json(_params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows: Vec<sqlx::mysql::MySqlRow> = sqlx::query(
        "SELECT id, appdesc, apptype, version, downloadurl, createtime FROM tb_app_record WHERE removecode = '1'"
    )
    .fetch_all(&state.db.pool)
    .await?;
    
    let result: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.try_get::<String, _>("id").unwrap_or_default(),
            "appdesc": r.try_get::<Option<String>, _>("appdesc").ok().flatten(),
            "apptype": r.try_get::<Option<String>, _>("apptype").ok().flatten(),
            "version": r.try_get::<Option<String>, _>("version").ok().flatten(),
            "downloadurl": r.try_get::<Option<String>, _>("downloadurl").ok().flatten(),
            "createtime": r.try_get::<Option<String>, _>("createtime").ok().flatten(),
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

/// 查询最新APP版本
pub async fn query_new_version(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let apptype = params.get("apptype").and_then(|v| v.as_str());
    
    let row: Option<sqlx::mysql::MySqlRow> = if let Some(ptype) = apptype {
        sqlx::query(
            "SELECT id, appdesc, apptype, version, downloadurl, createtime FROM tb_app_record WHERE removecode = '1' AND apptype = ? ORDER BY createtime DESC LIMIT 1"
        )
        .bind(ptype)
        .fetch_optional(&state.db.pool)
        .await?
    } else {
        sqlx::query(
            "SELECT id, appdesc, apptype, version, downloadurl, createtime FROM tb_app_record WHERE removecode = '1' ORDER BY createtime DESC LIMIT 1"
        )
        .fetch_optional(&state.db.pool)
        .await?
    };
    
    if let Some(r) = row {
        Ok(Json(serde_json::json!({
            "code": 1,
            "msg": "",
            "data": {
                "id": r.try_get::<String, _>("id").unwrap_or_default(),
                "appdesc": r.try_get::<Option<String>, _>("appdesc").ok().flatten(),
                "apptype": r.try_get::<Option<String>, _>("apptype").ok().flatten(),
                "version": r.try_get::<Option<String>, _>("version").ok().flatten(),
                "downloadurl": r.try_get::<Option<String>, _>("downloadurl").ok().flatten(),
                "createtime": r.try_get::<Option<String>, _>("createtime").ok().flatten(),
            }
        })))
    } else {
        Ok(Json(serde_json::json!({
            "code": 1,
            "msg": "暂无数据",
            "data": null
        })))
    }
}