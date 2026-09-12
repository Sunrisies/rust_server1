use sqlx::mysql::MySqlPoolOptions;
use sqlx::Row;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // 解析URL
    let base_url = database_url.split('?').next().unwrap_or(&database_url);
    let without_scheme = base_url.strip_prefix("mysql://").unwrap_or(base_url);
    let at_pos = without_scheme.rfind('@').unwrap();
    let user_pass = &without_scheme[..at_pos];
    let host_db = &without_scheme[at_pos + 1..];
    let colon_pos = user_pass.find(':').unwrap();
    let username = &user_pass[..colon_pos];
    let password = &user_pass[colon_pos + 1..].replace("%40", "@");
    let slash_pos = host_db.find('/').unwrap();
    let host_port = &host_db[..slash_pos];
    let database = &host_db[slash_pos + 1..];
    let colon_pos = host_port.rfind(':').unwrap();
    let host = &host_port[..colon_pos];
    let port: u16 = host_port[colon_pos + 1..].parse().unwrap();
    
    let options = sqlx::mysql::MySqlConnectOptions::new()
        .host(host)
        .port(port)
        .username(username)
        .password(&password)
        .database(database)
        .ssl_mode(sqlx::mysql::MySqlSslMode::Disabled);
    
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .expect("Failed to connect");
    
    println!("========================================");
    println!("  数据库表结构检查");
    println!("========================================");
    println!();
    
    // 检查表是否存在
    let tables = vec![
        "tb_node",
        "tb_node_location",
        "tb_node_and_node_parameter",
        "tb_user_node",
        "block_plan",
    ];
    
    for table in &tables {
        let query = format!("SHOW TABLES LIKE '{}'", table);
        let result = sqlx::query(&query)
            .fetch_optional(&pool)
            .await;
        
        match result {
            Ok(Some(_)) => {
                println!("✅ 表 {} 存在", table);
                
                // 检查表结构
                let desc_query = format!("DESCRIBE {}", table);
                if let Ok(rows) = sqlx::query(&desc_query).fetch_all(&pool).await {
                    println!("   字段:");
                    for row in rows {
                        let field: String = row.get("Field");
                        let type_: String = row.get("Type");
                        println!("     - {}: {}", field, type_);
                    }
                }
            }
            Ok(None) => println!("❌ 表 {} 不存在", table),
            Err(e) => println!("❌ 查询表 {} 失败: {}", table, e),
        }
        println!();
    }
}
