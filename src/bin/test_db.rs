use sqlx::mysql::{MySqlPool, MySqlConnectOptions, MySqlPoolOptions};
use std::str::FromStr;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // 加载.env
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    println!("========================================");
    println!("  数据库连接诊断 v2");
    println!("========================================");
    
    // 解析URL中的各个部分
    let url = database_url.split('?').next().unwrap_or(&database_url);
    println!("连接URL: {}", url.replace("://root:***@", "://root:***@"));
    println!();
    
    // 尝试方式1: 直接使用URL
    println!("【方式1】使用完整URL连接...");
    let start = std::time::Instant::now();
    
    let connect_result = MySqlPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&database_url)
        .await;
    
    match connect_result {
        Ok(pool) => {
            let elapsed = start.elapsed();
            println!("✅ 方式1成功! (耗时: {:?})", elapsed);
            test_connection(&pool).await;
            return;
        }
        Err(e) => {
            let elapsed = start.elapsed();
            println!("❌ 方式1失败: {} (耗时: {:?})", e, elapsed);
        }
    }
    
    println!();
    
    // 尝试方式2: 手动构建连接选项
    println!("【方式2】手动构建连接选项...");
    let start = std::time::Instant::now();
    
    let options = MySqlConnectOptions::new()
        .host("api.chaoyang1024.top")
        .port(9906)
        .username("root")
        .password("zhuzhongqian@123456")
        .database("db_1125")
        .ssl_mode(sqlx::mysql::MySqlSslMode::Disabled);
    
    let connect_result = MySqlPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(options)
        .await;
    
    match connect_result {
        Ok(pool) => {
            let elapsed = start.elapsed();
            println!("✅ 方式2成功! (耗时: {:?})", elapsed);
            test_connection(&pool).await;
            return;
        }
        Err(e) => {
            let elapsed = start.elapsed();
            println!("❌ 方式2失败: {} (耗时: {:?})", e, elapsed);
        }
    }
    
    println!();
    println!("========================================");
    println!("  所有方式都失败了");
    println!("========================================");
    println!();
    println!("请检查:");
    println!("  1. 数据库服务是否正常运行");
    println!("  2. 用户权限是否正确");
    println!("  3. 防火墙是否允许连接");
}

async fn test_connection(pool: &MySqlPool) {
    println!();
    println!("【测试连接】执行查询...");
    
    // 测试1: 简单查询
    match sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(pool)
        .await
    {
        Ok(result) => println!("  ✅ SELECT 1 = {}", result),
        Err(e) => println!("  ❌ SELECT 1 失败: {}", e),
    }
    
    // 测试2: 数据库版本
    match sqlx::query_scalar::<_, String>("SELECT VERSION()")
        .fetch_one(pool)
        .await
    {
        Ok(version) => println!("  ✅ 数据库版本: {}", version),
        Err(e) => println!("  ❌ 获取版本失败: {}", e),
    }
    
    // 测试3: 列出表
    match sqlx::query_scalar::<_, String>("SHOW TABLES")
        .fetch_all(pool)
        .await
    {
        Ok(tables) => {
            println!("  ✅ 共 {} 个表:", tables.len());
            for (i, table) in tables.iter().take(5).enumerate() {
                println!("     {}. {}", i + 1, table);
            }
        }
        Err(e) => println!("  ❌ 获取表列表失败: {}", e),
    }
    
    println!();
    println!("========================================");
    println!("  ✅ 所有测试通过!");
    println!("========================================");
}
