// use reqwest::Method;
// use std::sync::Arc;
// use xchange_core::rescu::HttpError;
// use xchange_core::rescu::params_digest::{BaseParamsDigest, HmacAlgorithm, ParamsDigest};
// use xchange_core::rescu::resilient_http_client::RequestBuilder;
//
// /// -------------------------
// /// BinanceHmacDigest
// /// -------------------------
// pub struct BinanceHmacDigest {
//     inner: Arc<BaseParamsDigest>,
// }
//
// impl BinanceHmacDigest {
//     pub fn new(secret_base64: &str) -> Result<Arc<Self>, HttpError> {
//         let base = BaseParamsDigest::new(secret_base64, HmacAlgorithm::Sha256)?;
//         Ok(Arc::new(Self { inner: base }))
//     }
//
//     /// 构建交易所特定输入字符串
//     fn build_input_string(
//         method: &Method,
//         query: &[(String, String)],
//         body: Option<&str>,
//     ) -> String {
//         // 1. 构造 query_string（过滤 signature）
//         let query_str = BaseParamsDigest::build_query_string(
//             &query
//                 .iter()
//                 .filter(|(k, _)| k != "signature")
//                 .cloned()
//                 .collect::<Vec<_>>(),
//         );
//
//         // 2. Method → &str ("GET", "POST"...)
//         let m = method.as_str();
//
//         // 3. Binance 特定规则
//         match m {
//             "GET" | "DELETE" => query_str,
//
//             // POST 场景通常 body 参与签名
//             "POST" => {
//                 if let Some(b) = body {
//                     query_str + b
//                 } else {
//                     query_str
//                 }
//             }
//
//             // 其他方法（PUT, PATCH...）——目前 Binance 不用
//             _ => query_str,
//         }
//     }
// }
//
// impl ParamsDigest for BinanceHmacDigest {
//     fn digest_request(&self, req: &RequestBuilder) -> Result<String, HttpError> {
//         if !["GET", "POST", "DELETE"].contains(&req.method.as_str()) {
//             return Err(HttpError::UnsupportedMethod(format!(
//                 "Unsupported method: {}",
//                 req.method
//             )));
//         }
//
//         let input = Self::build_input_string(&req.method, &req.query, req.body.as_deref());
//         Ok(self.inner.digest_bytes(input.as_bytes()))
//     }
// }
//
// // ===================== 使用示例 =====================
// // let digest = BinanceHmacDigest::create(Some(api_secret_base64)).unwrap();
// // let query = vec![("symbol".into(), "BTCUSDT".into()), ("timestamp".into(), "123456789".into())];
// // let signature = digest.digest_request("POST", &query, Some("{\"side\":\"BUY\"}"))?;

mod binance_base_service;
