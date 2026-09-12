use sqlx::mysql::{MySqlConnectOptions, MySqlPool, MySqlPoolOptions};
use std::time::Duration;
use tracing::info;

use crate::config::DatabaseConfig;

/// 数据库连接池
#[derive(Debug, Clone)]
pub struct Database {
    pub pool: MySqlPool,
}

impl Database {
    /// 创建新的数据库连接
    pub async fn new(config: &DatabaseConfig) -> anyhow::Result<Self> {
        info!("正在连接数据库...");

        // 解析URL获取各个部分
        let url = &config.url;
        let options = Self::parse_url(url)?;

        let pool = MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect_with(options)
            .await?;

        info!("数据库连接成功");

        Ok(Self { pool })
    }

    /// URL解码
    fn url_decode(s: &str) -> String {
        s.replace("%40", "@")
            .replace("%23", "#")
            .replace("%26", "&")
            .replace("%3D", "=")
            .replace("%3F", "?")
            .replace("%2F", "/")
    }

    /// 从URL解析连接选项（处理密码中的特殊字符）
    fn parse_url(url: &str) -> anyhow::Result<MySqlConnectOptions> {
        // 移除查询参数
        let base_url = url.split('?').next().unwrap_or(url);
        
        // 解析 scheme://user:password@host:port/database
        let without_scheme = base_url
            .strip_prefix("mysql://")
            .unwrap_or(base_url);
        
        // 找到最后一个 @ 分隔符（因为密码中可能包含 @）
        let at_pos = without_scheme
            .rfind('@')
            .ok_or_else(|| anyhow::anyhow!("URL格式错误: 缺少@符号"))?;
        
        let user_pass = &without_scheme[..at_pos];
        let host_db = &without_scheme[at_pos + 1..];
        
        // 解析 user:password
        let colon_pos = user_pass
            .find(':')
            .ok_or_else(|| anyhow::anyhow!("URL格式错误: 缺少用户名密码分隔符"))?;
        
        let username = &user_pass[..colon_pos];
        // 对密码进行URL解码
        let password = Self::url_decode(&user_pass[colon_pos + 1..]);
        
        // 解析 host:port/database
        let slash_pos = host_db.find('/');
        let (host_port, database) = if let Some(pos) = slash_pos {
            (&host_db[..pos], Some(&host_db[pos + 1..]))
        } else {
            (host_db, None)
        };
        
        let (host, port) = if let Some(colon_pos) = host_port.rfind(':') {
            let host = &host_port[..colon_pos];
            let port: u16 = host_port[colon_pos + 1..]
                .parse()
                .map_err(|_| anyhow::anyhow!("端口号格式错误"))?;
            (host, port)
        } else {
            (host_port, 3306)
        };
        
        info!("  主机: {}:{}", host, port);
        info!("  用户: {}", username);
        info!("  数据库: {:?}", database);
        
        let mut options = MySqlConnectOptions::new()
            .host(host)
            .port(port)
            .username(username)
            .password(&password)
            .ssl_mode(sqlx::mysql::MySqlSslMode::Disabled);
        
        if let Some(db) = database {
            options = options.database(db);
        }
        
        Ok(options)
    }

    /// 测试数据库连接
    pub async fn ping(&self) -> anyhow::Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// 获取连接池大小
    pub fn pool_size(&self) -> u32 {
        self.pool.options().get_max_connections()
    }
}

/// 数据库初始化
pub async fn init_database(config: &DatabaseConfig) -> anyhow::Result<Database> {
    let db = Database::new(config).await?;
    
    // 测试连接
    db.ping().await?;
    
    tracing::info!("数据库连接池配置: 最大连接数={}", db.pool_size());

    Ok(db)
}
