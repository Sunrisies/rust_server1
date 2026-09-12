use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::JwtConfig;

/// JWT Claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,       // 用户ID
    pub username: String,  // 用户名
    pub exp: i64,          // 过期时间
    pub iat: i64,          // 签发时间
}

/// JWT工具
#[derive(Clone)]
pub struct JwtUtil {
    secret: String,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expire_hours: i64,
}

impl JwtUtil {
    pub fn new(config: &JwtConfig) -> Self {
        Self {
            secret: config.secret.clone(),
            encoding_key: EncodingKey::from_secret(config.secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(config.secret.as_bytes()),
            expire_hours: config.expire_hours as i64,
        }
    }
    
    /// 获取密钥
    pub fn secret_key(&self) -> &str {
        &self.secret
    }

    /// 生成JWT Token
    pub fn generate_token(&self, user_id: &str, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::hours(self.expire_hours)).timestamp(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
    }

    /// 验证JWT Token
    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}
