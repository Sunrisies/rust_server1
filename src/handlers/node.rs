use axum::{extract::State, Json};
use sqlx::Row;

use crate::error::AppError;
use crate::routes::AppState;

const NODE_COLUMNS: &str = "id, sname, ip, port, url, urlname, url2, urlname2, url3, urlname3, sonorurl1, sonorurl1name, sonorurl2, sonorurl2name, imei, state, taskid, tcpPort, charge, describes, remote_port, fourthGCardNumber, fourthGCardNumberExpirationTime, rtkcardNumber, rtkcardNumberExpirationTime, soundBoardId, soundBoardName";

fn node_json(row: &sqlx::mysql::MySqlRow) -> serde_json::Value {
    let id = row.try_get::<String, _>("id").unwrap_or_default();
    let create_time = id.parse::<i64>().ok().and_then(|ms| {
        chrono::DateTime::from_timestamp_millis(ms)
            .map(|v| v.format("%Y-%m-%d %H:%M:%S").to_string())
    });

    serde_json::json!({
        "id": id,
        "sname": row.try_get::<Option<String>, _>("sname").ok().flatten(),
        "ip": row.try_get::<Option<String>, _>("ip").ok().flatten(),
        "port": row.try_get::<Option<String>, _>("port").ok().flatten(),
        "video": [],
        "url": row.try_get::<Option<String>, _>("url").ok().flatten(),
        "urlname": row.try_get::<Option<String>, _>("urlname").ok().flatten(),
        "url2": row.try_get::<Option<String>, _>("url2").ok().flatten(),
        "urlname2": row.try_get::<Option<String>, _>("urlname2").ok().flatten(),
        "url3": row.try_get::<Option<String>, _>("url3").ok().flatten(),
        "urlname3": row.try_get::<Option<String>, _>("urlname3").ok().flatten(),
        "sonorurl1": row.try_get::<Option<String>, _>("sonorurl1").ok().flatten(),
        "sonorurl1name": row.try_get::<Option<String>, _>("sonorurl1name").ok().flatten(),
        "sonorurl2": row.try_get::<Option<String>, _>("sonorurl2").ok().flatten(),
        "sonorurl2name": row.try_get::<Option<String>, _>("sonorurl2name").ok().flatten(),
        "imei": row.try_get::<Option<String>, _>("imei").ok().flatten(),
        "state": row.try_get::<Option<String>, _>("state").ok().flatten(),
        "taskid": row.try_get::<Option<String>, _>("taskid").ok().flatten(),
        "message": null,
        "removecode": "1",
        "charge": row.try_get::<Option<String>, _>("charge").ok().flatten(),
        "functions": [],
        "jurisdictions": [],
        "nodeParameters": [],
        "tcpPort": row.try_get::<Option<String>, _>("tcpPort").ok().flatten(),
        "describes": row.try_get::<Option<String>, _>("describes").ok().flatten(),
        "remote_port": row.try_get::<Option<String>, _>("remote_port").ok().flatten(),
        "fourthGCardNumber": row.try_get::<Option<String>, _>("fourthGCardNumber").ok().flatten(),
        "fourthGCardNumberExpirationTime": row.try_get::<Option<String>, _>("fourthGCardNumberExpirationTime").ok().flatten(),
        "rtkcardNumber": row.try_get::<Option<String>, _>("rtkcardNumber").ok().flatten(),
        "rtkcardNumberExpirationTime": row.try_get::<Option<String>, _>("rtkcardNumberExpirationTime").ok().flatten(),
        "soundBoardId": row.try_get::<Option<String>, _>("soundBoardId").ok().flatten(),
        "soundBoardName": row.try_get::<Option<String>, _>("soundBoardName").ok().flatten(),
        "createTime": create_time,
        "latitude": null,
        "longitude": null,
    })
}

fn string_param(params: &serde_json::Value, name: &str) -> Option<String> {
    params.get(name).and_then(|v| v.as_str()).filter(|v| !v.is_empty()).map(str::to_owned)
}

/// Java queryNodeBycondition：动态条件 + PageHelper 分页。
pub async fn query_all(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.get("page").and_then(|v| v.as_i64()).filter(|v| *v > 0).unwrap_or(1);
    let limit = params.get("limit").and_then(|v| v.as_i64()).filter(|v| *v > 0).unwrap_or(10);
    let mut where_sql = String::from(" FROM tb_node WHERE removecode = 1");
    let mut binds = Vec::<String>::new();

    for (field, column) in [
        ("id", "id"), ("ip", "ip"), ("port", "port"), ("url", "url"),
        ("imei", "imei"), ("state", "state"), ("taskid", "taskid"),
        ("remote_port", "remote_port"), ("soundBoardId", "soundBoardId"),
        ("soundBoardName", "soundBoardName"),
    ] {
        if let Some(value) = string_param(&params, field) {
            where_sql.push_str(&format!(" AND {} = ?", column));
            binds.push(value);
        }
    }
    if let Some(value) = string_param(&params, "sname") {
        where_sql.push_str(" AND sname LIKE ?");
        binds.push(format!("%{}%", value));
    }
    if let Some(value) = string_param(&params, "keywords") {
        where_sql.push_str(" AND (sname LIKE ? OR describes LIKE ?)");
        binds.push(format!("%{}%", value));
        binds.push(format!("%{}%", value));
    }

    let count_sql = format!("SELECT COUNT(*) {}", where_sql);
    let mut count_query = sqlx::query_scalar::<_, i64>(&count_sql);
    for value in &binds { count_query = count_query.bind(value); }
    let total = count_query.fetch_one(&state.db.pool).await?;

    let data_sql = format!("SELECT {} {} ORDER BY id DESC LIMIT ? OFFSET ?", NODE_COLUMNS, where_sql);
    let mut data_query = sqlx::query(&data_sql);
    for value in &binds { data_query = data_query.bind(value); }
    let rows = data_query.bind(limit).bind((page - 1) * limit).fetch_all(&state.db.pool).await?;
    let data: Vec<_> = rows.iter().map(node_json).collect();

    Ok(Json(serde_json::json!({ "code": 1, "msg": "", "data": data, "count": total })))
}

/// Java queryNodeAll：无分页查询。
pub async fn query_all_unpaged(
    State(state): State<AppState>,
    Json(_params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows = sqlx::query(&format!("SELECT {} FROM tb_node WHERE removecode = 1 ORDER BY id DESC", NODE_COLUMNS))
        .fetch_all(&state.db.pool).await?;
    let data: Vec<_> = rows.iter().map(node_json).collect();
    let count = data.len() as i64;
    Ok(Json(serde_json::json!({ "code": 1, "msg": "", "data": data, "count": count })))
}

/// Java queryNodeByUser：按 User.id 或 User.username 查询有效关联设备。
pub async fn query_by_user(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut sql = format!("SELECT n.* FROM tb_user u JOIN tb_user_node un ON u.id=un.userid JOIN tb_node n ON un.nodeid=n.id WHERE n.removecode=1 AND un.removecode=1 AND u.removecode=1");
    let mut binds = Vec::new();
    if let Some(id) = string_param(&params, "id").or_else(|| string_param(&params, "userId")) {
        sql.push_str(" AND u.id = ?"); binds.push(id);
    }
    if let Some(username) = string_param(&params, "username") {
        sql.push_str(" AND u.username = ?"); binds.push(username);
    }
    let mut query = sqlx::query(&sql);
    for value in &binds { query = query.bind(value); }
    let rows = query.fetch_all(&state.db.pool).await?;
    let data: Vec<_> = rows.iter().map(node_json).collect();
    let count = data.len() as i64;
    Ok(Json(serde_json::json!({ "code": 1, "msg": "", "data": data, "count": count })))
}

/// Java addNode：检查名称，时间戳 ID，state/removecode 默认值。
pub async fn add(
    State(state): State<AppState>,
    Json(node): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let sname = string_param(&node, "sname").unwrap_or_default();
    if !sname.is_empty() {
        let duplicate: Option<String> = sqlx::query_scalar("SELECT id FROM tb_node WHERE sname=? AND removecode=1 LIMIT 1")
            .bind(&sname).fetch_optional(&state.db.pool).await?;
        if duplicate.is_some() { return Ok(Json(serde_json::json!({ "code": 0, "msg": "船只名称已存在" }))); }
    }
    let id = chrono::Utc::now().timestamp_millis().to_string();
    sqlx::query("INSERT INTO tb_node (id,sname,ip,port,state,removecode,describes) VALUES (?,?,?,?, '1','1',?)")
        .bind(&id).bind(&sname).bind(string_param(&node,"ip")).bind(string_param(&node,"port"))
        .bind(string_param(&node,"describes")).execute(&state.db.pool).await?;
    Ok(Json(serde_json::json!({ "code": 1, "msg": "" })))
}

/// Java updataNode：只更新请求中非空的字段。
pub async fn update(
    State(state): State<AppState>,
    Json(node): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = string_param(&node, "id").ok_or_else(|| AppError::BadRequest("添加参数为null..".to_string()))?;
    let fields = [
        ("sname", "sname"), ("ip", "ip"), ("port", "port"), ("url", "url"),
        ("urlname", "urlname"), ("url2", "url2"), ("urlname2", "urlname2"),
        ("url3", "url3"), ("urlname3", "urlname3"), ("sonorurl1", "sonorurl1"),
        ("sonorurl1name", "sonorurl1name"), ("sonorurl2", "sonorurl2"),
        ("sonorurl2name", "sonorurl2name"), ("imei", "imei"), ("state", "state"),
        ("taskid", "taskid"), ("tcpPort", "tcpPort"), ("charge", "charge"),
        ("removecode", "removecode"), ("describes", "describes"), ("remote_port", "remote_port"),
        ("fourthGCardNumber", "fourthGCardNumber"), ("fourthGCardNumberExpirationTime", "fourthGCardExpirationTime"),
        ("rtkcardNumber", "rtkcardNumber"), ("rtkcardNumberExpirationTime", "rtkcardExpirationTime"),
        ("soundBoardId", "soundBoardId"), ("soundBoardName", "soundBoardName"),
    ];
    let mut assignments = Vec::new();
    let mut values = Vec::new();
    for (json_name, column) in fields {
        if let Some(value) = string_param(&node, json_name) {
            assignments.push(format!("{} = ?", column)); values.push(value);
        }
    }
    if assignments.is_empty() { return Ok(Json(serde_json::json!({ "code": 1, "msg": "" }))); }
    let sql = format!("UPDATE tb_node SET {} WHERE id = ?", assignments.join(", "));
    let mut query = sqlx::query(&sql);
    for value in &values { query = query.bind(value); }
    query.bind(id).execute(&state.db.pool).await?;
    Ok(Json(serde_json::json!({ "code": 1, "msg": "" })))
}

/// Java deteleNode：逻辑删除。
pub async fn delete(
    State(state): State<AppState>,
    Json(node): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = string_param(&node, "id").ok_or_else(|| AppError::BadRequest("添加参数为null..".to_string()))?;
    sqlx::query("UPDATE tb_node SET removecode='0' WHERE id=?").bind(id).execute(&state.db.pool).await?;
    Ok(Json(serde_json::json!({ "code": 1, "msg": "" })))
}

/// Java updateNodeLocation：通过 taskid 找设备 ID，再更新同一位置记录。
pub async fn update_location(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let taskid = string_param(&params, "taskid").ok_or_else(|| AppError::BadRequest("未找到对应的节点信息".to_string()))?;
    let latitude = params.get("latitude").and_then(|v| v.as_f64()).ok_or_else(|| AppError::BadRequest("经纬度值超出有效范围".to_string()))?;
    let longitude = params.get("longitude").and_then(|v| v.as_f64()).ok_or_else(|| AppError::BadRequest("经纬度值超出有效范围".to_string()))?;
    if !(-90.0..=90.0).contains(&latitude) || !(-180.0..=180.0).contains(&longitude) {
        return Ok(Json(serde_json::json!({ "code": 0, "msg": "经纬度值超出有效范围" })));
    }
    let node_id: Option<String> = sqlx::query_scalar("SELECT id FROM tb_node WHERE taskid=? LIMIT 1").bind(&taskid).fetch_optional(&state.db.pool).await?;
    let Some(node_id) = node_id else { return Ok(Json(serde_json::json!({ "code": 0, "msg": "未找到对应的节点信息" }))); };
    let affected = sqlx::query("INSERT INTO tb_node_location (id,latitude,longitude,update_time) VALUES (?,?,?,NOW()) ON DUPLICATE KEY UPDATE latitude=VALUES(latitude), longitude=VALUES(longitude), update_time=NOW()")
        .bind(node_id).bind(latitude).bind(longitude).execute(&state.db.pool).await?.rows_affected();
    Ok(Json(serde_json::json!({ "code": if affected > 0 { 1 } else { 0 }, "msg": if affected > 0 { "更新成功" } else { "更新失败" } })))
}