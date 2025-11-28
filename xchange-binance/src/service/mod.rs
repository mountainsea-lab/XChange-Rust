use crate::dto::BinanceError;
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
    pub fn new(secret_base64: &str) -> Result<Arc<Self>, BinanceError> {
        let base = BaseParamsDigest::new(secret_base64, HmacAlgorithm::Sha256)?;
        Ok(Arc::new(Self { inner: base }))
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

// ===================== 使用示例 =====================
// let digest = BinanceHmacDigest::create(Some(api_secret_base64)).unwrap();
// let query = vec![("symbol".into(), "BTCUSDT".into()), ("timestamp".into(), "123456789".into())];
// let signature = digest.digest_request("POST", &query, Some("{\"side\":\"BUY\"}"))?;

mod binance_base_service;
