mod config;
mod database;
mod redis;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info, error};

use crate::config::AppConfig;
use crate::database::init_database;
use crate::redis::init_redis;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("========================================");
    info!("  平台后端 - Rust版本");
    info!("========================================");

    // 加载配置
    let config = AppConfig::from_env();
    info!("配置加载完成");
    info!("  服务器地址: {}", config.server_addr());
    info!("  数据库: {}", mask_password(&config.database.url));
    info!("  Redis: {}", config.redis.url);

    // 初始化数据库
    match init_database(&config.database).await {
        Ok(db) => {
            info!("数据库初始化成功");
            info!("  连接池大小: {}", db.pool_size());
        }
        Err(e) => {
            error!("数据库初始化失败: {}", e);
            std::process::exit(1);
        }
    }

    // 初始化Redis
    match init_redis(&config.redis).await {
        Ok(redis) => {
            info!("Redis初始化成功");
            
            // 测试Redis操作
            if let Err(e) = redis.set("test:ping", "pong").await {
                error!("Redis测试写入失败: {}", e);
            } else if let Ok(Some(value)) = redis.get("test:ping").await {
                info!("  Redis测试读取: {}", value);
                let _ = redis.del("test:ping").await;
            }
        }
        Err(e) => {
            error!("Redis初始化失败: {}", e);
            std::process::exit(1);
        }
    }

    info!("========================================");
    info!("所有服务初始化完成，准备启动服务器...");
    info!("========================================");

    // TODO: 启动Web服务器
    // let app = create_router(db, redis, config);
    // let listener = tokio::net::TcpListener::bind(config.server_addr()).await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

/// 隐藏密码
fn mask_password(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            let prefix = &url[..colon_pos + 1];
            let suffix = &url[at_pos..];
            return format!("{}***{}", prefix, suffix);
        }
    }
    url.to_string()
}
