use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use crate::redis::Redis;

/// Token消息（匹配Java版本的TokenMessage）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMessage {
    pub token: String,
    #[serde(rename = "establishTime")]
    pub establish_time: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub username: String,
    #[serde(rename = "usertype")]
    pub usertype: Option<i32>,
    #[serde(rename = "workspaceId")]
    pub workspace_id: Option<String>,
    #[serde(rename = "mqttUsername")]
    pub mqtt_username: Option<String>,
    #[serde(rename = "mqttPassword")]
    pub mqtt_password: Option<String>,
}

/// Session管理（匹配Java版本的Session）
pub struct Session {
    redis: Redis,
    expire_time: i64,
}

impl Session {
    pub fn new(redis: Redis) -> Self {
        Self {
            redis,
            expire_time: 1800, // 30分钟
        }
    }
    
    /// 保存session
    pub async fn save_session(&self, token_message: &TokenMessage) -> anyhow::Result<()> {
        let key = format!("login:user:{}", token_message.token);
        let value = serde_json::to_string(token_message)?;
        self.redis.set_ex(&key, &value, self.expire_time as u64).await?;
        Ok(())
    }
    
    /// 获取session
    pub async fn get_session(&self, token: &str) -> anyhow::Result<Option<TokenMessage>> {
        let key = format!("login:user:{}", token);
        match self.redis.get(&key).await? {
            Some(value) => {
                let token_message: TokenMessage = serde_json::from_str(&value)?;
                Ok(Some(token_message))
            }
            None => Ok(None),
        }
    }
    
    /// 删除session
    pub async fn delete_session(&self, token_message: &TokenMessage) -> anyhow::Result<()> {
        let key = format!("login:user:{}", token_message.token);
        self.redis.del(&key).await?;
        
        // 删除权限缓存
        let permission_key = format!("login:permission:{}", token_message.username);
        self.redis.del(&permission_key).await?;
        
        Ok(())
    }
    
    /// 重置token有效期
    pub async fn expire_session(&self, token_message: &TokenMessage) -> anyhow::Result<()> {
        let key = format!("login:user:{}", token_message.token);
        self.redis.set_ex(&key, "", self.expire_time as u64).await?;
        
        let permission_key = format!("login:permission:{}", token_message.username);
        self.redis.set_ex(&permission_key, "", self.expire_time as u64).await?;
        
        Ok(())
    }
    
    /// 保存用户权限
    pub async fn save_user_permissions(&self, username: &str, permissions: &[PermissionDTO]) -> anyhow::Result<()> {
        let key = format!("login:permission:{}", username);
        let value = serde_json::to_string(permissions)?;
        self.redis.set_ex(&key, &value, self.expire_time as u64).await?;
        Ok(())
    }
    
    /// 获取用户权限
    pub async fn get_user_permissions(&self, username: &str) -> anyhow::Result<Vec<PermissionDTO>> {
        let key = format!("login:permission:{}", username);
        match self.redis.get(&key).await? {
            Some(value) => {
                let permissions: Vec<PermissionDTO> = serde_json::from_str(&value)?;
                Ok(permissions)
            }
            None => Ok(Vec::new()),
        }
    }
}

/// 权限DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionDTO {
    pub name: Option<String>,
    pub uri: Option<String>,
    pub method: Option<String>,
}

/// 全局Session实例
static SESSION: OnceLock<Session> = OnceLock::new();

/// 初始化Session
pub fn init_session(redis: Redis) -> &'static Session {
    SESSION.get_or_init(|| Session::new(redis))
}

/// 获取Session实例
pub fn get_session() -> &'static Session {
    SESSION.get().expect("Session未初始化")
}
