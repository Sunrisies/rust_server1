use redis::aio::ConnectionManager;
use redis::Client;
use tracing::info;

use crate::config::RedisConfig;

/// Redis连接
#[derive(Clone)]
pub struct Redis {
    pub manager: ConnectionManager,
}

impl std::fmt::Debug for Redis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Redis").finish()
    }
}

impl Redis {
    /// 创建新的Redis连接
    pub async fn new(config: &RedisConfig) -> anyhow::Result<Self> {
        info!("正在连接Redis...");

        let client = Client::open(config.url.as_str())
            .map_err(|e| anyhow::anyhow!("Redis客户端创建失败: {}", e))?;

        let manager = ConnectionManager::new(client)
            .await
            .map_err(|e| anyhow::anyhow!("Redis连接失败: {}", e))?;

        info!("Redis连接成功");

        Ok(Self { manager })
    }

    /// 测试Redis连接
    pub async fn ping(&self) -> anyhow::Result<()> {
        let mut conn = self.manager.clone();
        let _: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .map_err(|e| anyhow::anyhow!("Redis ping失败: {}", e))?;
        Ok(())
    }

    /// 设置键值
    pub async fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let mut conn = self.manager.clone();
        let _: () = redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query_async(&mut conn)
            .await
            .map_err(|e| anyhow::anyhow!("Redis SET失败: {}", e))?;
        Ok(())
    }

    /// 获取值
    pub async fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        let mut conn = self.manager.clone();
        let result: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| anyhow::anyhow!("Redis GET失败: {}", e))?;
        Ok(result)
    }

    /// 删除键
    pub async fn del(&self, key: &str) -> anyhow::Result<()> {
        let mut conn = self.manager.clone();
        let _: () = redis::cmd("DEL")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| anyhow::anyhow!("Redis DEL失败: {}", e))?;
        Ok(())
    }

    /// 设置键值（带过期时间，秒）
    pub async fn set_ex(&self, key: &str, value: &str, seconds: u64) -> anyhow::Result<()> {
        let mut conn = self.manager.clone();
        let _: () = redis::cmd("SETEX")
            .arg(key)
            .arg(seconds)
            .arg(value)
            .query_async(&mut conn)
            .await
            .map_err(|e| anyhow::anyhow!("Redis SETEX失败: {}", e))?;
        Ok(())
    }

    /// 检查键是否存在
    pub async fn exists(&self, key: &str) -> anyhow::Result<bool> {
        let mut conn = self.manager.clone();
        let result: bool = redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| anyhow::anyhow!("Redis EXISTS失败: {}", e))?;
        Ok(result)
    }
}

/// 初始化Redis连接
pub async fn init_redis(config: &RedisConfig) -> anyhow::Result<Redis> {
    let redis = Redis::new(config).await?;
    
    // 测试连接
    redis.ping().await?;
    tracing::info!("Redis连接测试通过");

    Ok(redis)
}
