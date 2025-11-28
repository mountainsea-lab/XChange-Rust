use base64;
use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha384, Sha512};
use std::sync::Arc;
use thiserror::Error;
use url::form_urlencoded;

use crate::rescu::HttpError;

/// -------------------------
/// 签名算法枚举
/// -------------------------
#[derive(Clone, Copy, Debug)]
pub enum HmacAlgorithm {
    Sha256,
    Sha384,
    Sha512,
}

/// -------------------------
/// 内部通用 trait，用于多态 Mac
/// -------------------------
trait MacTrait: Send + Sync {
    fn finalize_clone(&self, data: &[u8]) -> Vec<u8>;
}

/// -------------------------
/// BaseParamsDigest
/// -------------------------
#[derive(Clone)]
pub struct BaseParamsDigest {
    mac: Arc<dyn MacTrait>,
}

impl MacTrait for Hmac<Sha256> {
    fn finalize_clone(&self, data: &[u8]) -> Vec<u8> {
        let mut mac = self.clone();
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }
}

impl MacTrait for Hmac<Sha384> {
    fn finalize_clone(&self, data: &[u8]) -> Vec<u8> {
        let mut mac = self.clone();
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }
}

impl MacTrait for Hmac<Sha512> {
    fn finalize_clone(&self, data: &[u8]) -> Vec<u8> {
        let mut mac = self.clone();
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }
}

/// -------------------------
/// 错误类型
/// -------------------------
#[derive(Debug, Error)]
pub enum DigestError {
    #[error("Invalid base64 key: {0}")]
    InvalidKey(String),
}

/// -------------------------
/// 公共方法实现
/// -------------------------
impl BaseParamsDigest {
    /// 创建 BaseParamsDigest
    pub fn new(secret_base64: &str, algo: HmacAlgorithm) -> Result<Arc<Self>, DigestError> {
        let key_bytes =
            base64::decode(secret_base64).map_err(|e| DigestError::InvalidKey(e.to_string()))?;

        let mac: Arc<dyn MacTrait> = match algo {
            HmacAlgorithm::Sha256 => Arc::new(
                Hmac::<Sha256>::new_from_slice(&key_bytes)
                    .map_err(|e| DigestError::InvalidKey(e.to_string()))?,
            ),
            HmacAlgorithm::Sha384 => Arc::new(
                Hmac::<Sha384>::new_from_slice(&key_bytes)
                    .map_err(|e| DigestError::InvalidKey(e.to_string()))?,
            ),
            HmacAlgorithm::Sha512 => Arc::new(
                Hmac::<Sha512>::new_from_slice(&key_bytes)
                    .map_err(|e| DigestError::InvalidKey(e.to_string()))?,
            ),
        };

        Ok(Arc::new(Self { mac }))
    }

    /// 构建 query string
    pub fn build_query_string(params: &[(String, String)]) -> String {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(params.iter())
            .finish()
    }

    /// 对字节数组进行签名
    pub fn digest_bytes(&self, data: &[u8]) -> String {
        hex::encode(self.mac.finalize_clone(data))
    }

    /// 对字符串进行签名
    pub fn digest_str(&self, s: &str) -> String {
        self.digest_bytes(s.as_bytes())
    }

    /// 根据 HTTP 方法 + query string 构建签名（兼容 retrofit_rs）
    /// GET/DELETE -> query string, POST/PUT -> body
    pub fn sign_request(
        &self,
        method: &str,
        endpoint: &str,
        params: &[(String, String)],
        body: Option<&[u8]>,
    ) -> String {
        let payload = match method.to_uppercase().as_str() {
            "GET" | "DELETE" => {
                let qs = Self::build_query_string(params);
                format!("{}?{}", endpoint, qs)
            }
            "POST" | "PUT" => {
                if let Some(b) = body {
                    String::from_utf8_lossy(b).to_string()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        };

        self.digest_str(&payload)
    }
}
