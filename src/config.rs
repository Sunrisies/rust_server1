use std::env;

/// 应用配置
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
}

/// 数据库配置
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

/// Redis配置
#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
}

/// 服务器配置
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// JWT配置
#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expire_hours: u64,
}

impl AppConfig {
    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        // 加载 .env 文件
        dotenvy::dotenv().ok();

        Self {
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .expect("DATABASE_URL must be set"),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "25".to_string())
                    .parse()
                    .unwrap_or(25),
            },
            redis: RedisConfig {
                url: env::var("REDIS_URL")
                    .expect("REDIS_URL must be set"),
            },
            server: ServerConfig {
                host: env::var("SERVER_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET")
                    .expect("JWT_SECRET must be set"),
                expire_hours: env::var("JWT_EXPIRE_HOURS")
                    .unwrap_or_else(|_| "24".to_string())
                    .parse()
                    .unwrap_or(24),
            },
        }
    }

    /// 获取服务器地址
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}
