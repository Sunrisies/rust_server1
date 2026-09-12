use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{EncodePublicKey, EncodePrivateKey, LineEnding};
use rand::thread_rng;
use std::sync::OnceLock;

/// RSA工具
pub struct RsaUtil {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl RsaUtil {
    /// 创建新的RSA工具
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let bits = 1024;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        
        Self {
            private_key,
            public_key,
        }
    }
    
    /// 获取公钥（PEM格式）
    pub fn get_public_key_pem(&self) -> String {
        self.public_key
            .to_public_key_pem(LineEnding::LF)
            .expect("failed to encode public key")
    }
    
    /// 解密数据
    pub fn decrypt(&self, encrypted_data: &str) -> Result<String, Box<dyn std::error::Error>> {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD;
        
        // 清理输入数据，移除可能的换行符和头尾标记
        let cleaned = encrypted_data
            .replace("-----BEGIN PUBLIC KEY-----", "")
            .replace("-----END PUBLIC KEY-----", "")
            .replace("-----BEGIN RSA PRIVATE KEY-----", "")
            .replace("-----END RSA PRIVATE KEY-----", "")
            .replace('\n', "")
            .replace('\r', "")
            .trim()
            .to_string();
        
        let data = STANDARD.decode(&cleaned)?;
        let decrypted_data = self.private_key.decrypt(Pkcs1v15Encrypt, &data)?;
        Ok(String::from_utf8(decrypted_data)?)
    }
}

/// 全局RSA工具实例
static RSA_UTIL: OnceLock<RsaUtil> = OnceLock::new();

/// 初始化RSA工具
pub fn init_rsa() -> &'static RsaUtil {
    RSA_UTIL.get_or_init(|| {
        let rsa_util = RsaUtil::new();
        tracing::info!("RSA密钥对生成完成");
        rsa_util
    })
}

/// 获取RSA工具实例
pub fn get_rsa_util() -> &'static RsaUtil {
    RSA_UTIL.get().expect("RSA工具未初始化")
}