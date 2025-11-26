use crate::rescu::HttpError;
use base64;
use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha512};
use std::sync::Arc;

/// 签名 trait
pub trait ParamsDigest: Send + Sync {
    /// 对 query 参数签名
    fn digest(&self, query: &[(String, String)]) -> Result<String, HttpError>;
}

/// 支持的 HMAC 算法
#[derive(Clone, Copy)]
pub enum HmacAlgorithm {
    Sha256,
    Sha512,
}

/// 内部通用 trait，用于多态 Mac
trait MacTrait: Send + Sync {
    fn finalize_clone(&self) -> Vec<u8>;
}

/// 包装 Hmac<Sha256> 或 Hmac<Sha512>
pub struct BaseParamsDigest {
    mac: Box<dyn MacTrait>,
}

impl MacTrait for Hmac<Sha256> {
    fn finalize_clone(&self) -> Vec<u8> {
        self.clone().finalize().into_bytes().to_vec()
    }
}

impl MacTrait for Hmac<Sha512> {
    fn finalize_clone(&self) -> Vec<u8> {
        self.clone().finalize().into_bytes().to_vec()
    }
}

impl BaseParamsDigest {
    /// 创建一个新的签名器，secret_base64 必须是 base64 编码
    pub fn new(secret_base64: &str, algo: HmacAlgorithm) -> Result<Arc<Self>, HttpError> {
        let key_bytes = base64::decode(secret_base64)
            .map_err(|e| HttpError::InvalidKey(format!("Invalid base64 key: {}", e)))?;

        let mac: Box<dyn MacTrait> = match algo {
            HmacAlgorithm::Sha256 => Hmac::<Sha256>::new_from_slice(&key_bytes)
                .map(|h| Box::new(h) as Box<dyn MacTrait>)
                .map_err(|e| HttpError::InvalidKey(format!("HMAC init failed: {}", e)))?,
            HmacAlgorithm::Sha512 => Hmac::<Sha512>::new_from_slice(&key_bytes)
                .map(|h| Box::new(h) as Box<dyn MacTrait>)
                .map_err(|e| HttpError::InvalidKey(format!("HMAC init failed: {}", e)))?,
        };

        Ok(Arc::new(Self { mac }))
    }

    /// 拼接 query 参数
    fn build_query_string(params: &[(String, String)]) -> String {
        params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }
}

impl ParamsDigest for BaseParamsDigest {
    fn digest(&self, params: &[(String, String)]) -> Result<String, HttpError> {
        // 拼接 query
        let query_str = Self::build_query_string(params);

        // 克隆 Mac 进行计算，避免多线程冲突
        let mac_bytes = self.mac.finalize_clone();

        Ok(hmac_sha256(query_str.as_bytes(), &mac_bytes))
    }
}

/// 辅助函数：计算 HMAC-SHA256
fn hmac_sha256(data: &[u8], key: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("Hmac init failed");
    mac.update(data);
    hex::encode(mac.finalize().into_bytes())
}

// ===================== 使用示例 =====================
// let digest = BaseParamsDigest::new(api_secret_base64, HmacAlgorithm::Sha256)?;
// let query = vec![("symbol".into(), "BTCUSDT".into()), ("timestamp".into(), "123456789".into())];
// let signature = digest.digest(&query)?;
