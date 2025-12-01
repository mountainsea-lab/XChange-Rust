use crate::dto::BinanceError;
use base64::{Engine as _, engine::general_purpose};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use std::sync::Arc;
use xchange_core::rescu::HttpError;
use xchange_core::rescu::params_digest::{BaseParamsDigest, HmacAlgorithm, ParamsDigest};

/// -------------------------
/// BinanceHmacDigest
/// -------------------------
pub struct BinanceHmacDigest {
    inner: Arc<BaseParamsDigest>,
}

impl BinanceHmacDigest {
    pub fn new(secret_base64: &str) -> Result<Self, BinanceError> {
        let base = BaseParamsDigest::new(secret_base64, HmacAlgorithm::Sha256)?;
        Ok(Self { inner: base })
    }

    /// 构建交易所特定输入字符串
    fn build_input_string(method: &str, query: &[(String, String)], body: Option<&str>) -> String {
        // 1. 构造 query_string（过滤 signature）
        let query_str = BaseParamsDigest::build_query_string(
            &query
                .iter()
                .filter(|(k, _)| k != "signature")
                .cloned()
                .collect::<Vec<_>>(),
        );

        // 2. Binance 特定规则
        match method.to_uppercase().as_str() {
            "GET" | "DELETE" => query_str,

            // POST 场景通常 body 参与签名
            "POST" => {
                if let Some(b) = body {
                    query_str + b
                } else {
                    query_str
                }
            }

            // 其他方法（PUT, PATCH...）——目前 Binance 不用
            _ => query_str,
        }
    }
}

impl ParamsDigest for BinanceHmacDigest {
    fn digest_params(
        &self,
        method: &str,
        query: &[(String, String)],
        body: Option<&str>,
    ) -> Result<String, HttpError> {
        if !["GET", "POST", "DELETE"].contains(&method) {
            return Err(HttpError::UnsupportedMethod(format!(
                "Unsupported method: {}",
                method
            )));
        }

        let input = Self::build_input_string(method, query, body);
        Ok(self.inner.digest_bytes(input.as_bytes()))
    }
}

pub struct BinanceEd25519Digest {
    signing_key: Arc<SigningKey>,
    verifying_key: Arc<VerifyingKey>,
}

impl BinanceEd25519Digest {
    pub fn new(secret_key_base64: &str) -> Result<Self, BinanceError> {
        let decoded = general_purpose::STANDARD
            .decode(secret_key_base64)
            .map_err(|e| BinanceError::InvalidKey(format!("Base64 decode error: {}", e)))?;

        // 确保长度是 32
        let secret_bytes: [u8; 32] = decoded
            .try_into()
            .map_err(|_| BinanceError::InvalidKey("Secret key must be 32 bytes".to_string()))?;

        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let verifying_key = VerifyingKey::from(&signing_key);

        Ok(Self {
            signing_key: Arc::new(signing_key),
            verifying_key: Arc::new(verifying_key),
        })
    }

    fn build_input_string(method: &str, query: &[(String, String)], body: Option<&str>) -> String {
        let query_str = query
            .iter()
            .filter(|(k, _)| k != "signature")
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        match method.to_ascii_uppercase().as_str() {
            "GET" | "DELETE" => query_str,
            "POST" | "PUT" => query_str + body.unwrap_or(""),
            _ => query_str,
        }
    }

    fn sign(&self, payload: &[u8]) -> String {
        let signing_key: &SigningKey = &self.signing_key; // Arc<SigningKey> 也可 deref
        let sig = signing_key.sign(payload);
        general_purpose::STANDARD.encode(sig.to_bytes())
    }
}
impl ParamsDigest for BinanceEd25519Digest {
    fn digest_params(
        &self,
        method: &str,
        query: &[(String, String)],
        body: Option<&str>,
    ) -> Result<String, HttpError> {
        let input = Self::build_input_string(method, query, body);
        Ok(self.sign(input.as_bytes()))
    }
}

/// 批量代理 Client 方法到 Service
/// Usage:
/// ```rust
/// delegate_client! {
///     client, // 组合对象字段名
///     {
///         ping => (),
///         get_exchange_info => BinanceExchangeInfo,
///     }
/// }
/// ```
#[macro_export]
macro_rules! delegate_client {
    ($client:ident, { $($fn_name:ident => $ret:ty),* $(,)? }) => {
        $(
            pub async fn $fn_name(&self) -> Result<$ret, crate::dto::BinanceError> {
                self.$client.$fn_name().await
            }
        )*
    };
}

pub mod account_service;
pub mod binance_account_service_raw;
pub mod binance_base_service;
mod market_data_service;
mod market_data_service_inner;
