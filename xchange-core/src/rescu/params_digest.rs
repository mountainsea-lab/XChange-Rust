// use crate::rescu::HttpError;
// use crate::rescu::resilient_http_client::RequestBuilder;
// use base64;
// use hmac::{Hmac, Mac};
// use sha2::{Sha256, Sha384, Sha512};
// use std::sync::Arc;
//
// /// -------------------------
// /// 签名 trait
// /// -------------------------
// pub trait ParamsDigest: Send + Sync {
//     /// 根据 RequestBuilder 对请求进行签名
//     fn digest_request(&self, req: &RequestBuilder) -> Result<String, HttpError>;
// }
//
// /// -------------------------
// /// HMAC 算法枚举
// /// -------------------------
// #[derive(Clone, Copy)]
// pub enum HmacAlgorithm {
//     // Sha1,
//     Sha256,
//     Sha384,
//     Sha512,
//     // Md5,
// }
//
// /// -------------------------
// /// 内部通用 trait，用于多态 Mac
// /// -------------------------
// trait MacTrait: Send + Sync {
//     fn finalize_clone(&self, data: &[u8]) -> Vec<u8>;
// }
//
// /// -------------------------
// /// BaseParamsDigest
// /// -------------------------
// pub struct BaseParamsDigest {
//     mac: Box<dyn MacTrait>,
// }
//
// // impl MacTrait for Hmac<Sha1> {
// //     fn finalize_clone(&self, data: &[u8]) -> Vec<u8> {
// //         let mut mac = self.clone();
// //         mac.update(data);
// //         mac.finalize().into_bytes().to_vec()
// //     }
// // }
//
// impl MacTrait for Hmac<Sha256> {
//     fn finalize_clone(&self, data: &[u8]) -> Vec<u8> {
//         let mut mac = self.clone();
//         mac.update(data);
//         mac.finalize().into_bytes().to_vec()
//     }
// }
//
// impl MacTrait for Hmac<Sha384> {
//     fn finalize_clone(&self, data: &[u8]) -> Vec<u8> {
//         let mut mac = self.clone();
//         mac.update(data);
//         mac.finalize().into_bytes().to_vec()
//     }
// }
//
// impl MacTrait for Hmac<Sha512> {
//     fn finalize_clone(&self, data: &[u8]) -> Vec<u8> {
//         let mut mac = self.clone();
//         mac.update(data);
//         mac.finalize().into_bytes().to_vec()
//     }
// }
//
// // impl MacTrait for Hmac<Md5> {
// //     fn finalize_clone(&self, data: &[u8]) -> Vec<u8> {
// //         let mut mac = self.clone();
// //         mac.update(data);
// //         mac.finalize().into_bytes().to_vec()
// //     }
// // }
//
// impl BaseParamsDigest {
//     /// 创建 BaseParamsDigest
//     pub fn new(secret_base64: &str, algo: HmacAlgorithm) -> Result<Arc<Self>, HttpError> {
//         let key_bytes = base64::decode(secret_base64)
//             .map_err(|e| HttpError::InvalidKey(format!("Invalid base64 key: {}", e)))?;
//
//         let mac: Box<dyn MacTrait> = match algo {
//             // HmacAlgorithm::Sha1 => Hmac::<Sha1>::new_from_slice(&key_bytes)
//             //     .map(|h| Box::new(h) as Box<dyn MacTrait>)
//             //     .map_err(|e| HttpError::InvalidKey(format!("HMAC init failed: {}", e)))?,
//             HmacAlgorithm::Sha256 => Hmac::<Sha256>::new_from_slice(&key_bytes)
//                 .map(|h| Box::new(h) as Box<dyn MacTrait>)
//                 .map_err(|e| HttpError::InvalidKey(format!("HMAC init failed: {}", e)))?,
//             HmacAlgorithm::Sha384 => Hmac::<Sha384>::new_from_slice(&key_bytes)
//                 .map(|h| Box::new(h) as Box<dyn MacTrait>)
//                 .map_err(|e| HttpError::InvalidKey(format!("HMAC init failed: {}", e)))?,
//             HmacAlgorithm::Sha512 => Hmac::<Sha512>::new_from_slice(&key_bytes)
//                 .map(|h| Box::new(h) as Box<dyn MacTrait>)
//                 .map_err(|e| HttpError::InvalidKey(format!("HMAC init failed: {}", e)))?,
//             // HmacAlgorithm::Md5 => Hmac::<Md5>::new_from_slice(&key_bytes)
//             //     .map(|h| Box::new(h) as Box<dyn MacTrait>)
//             //     .map_err(|e| HttpError::InvalidKey(format!("HMAC init failed: {}", e)))?,
//         };
//
//         Ok(Arc::new(Self { mac }))
//     }
//
//     /// 拼接 query 参数
//     pub fn build_query_string(params: &[(String, String)]) -> String {
//         params
//             .iter()
//             .map(|(k, v)| format!("{}={}", k, v))
//             .collect::<Vec<_>>()
//             .join("&")
//     }
//
//     /// 对任意数据进行 HMAC 签名，返回 hex 字符串
//     pub fn digest_bytes(&self, data: &[u8]) -> String {
//         hex::encode(self.mac.finalize_clone(data))
//     }
//
//     pub fn digest_str(&self, s: &str) -> String {
//         self.digest_bytes(s.as_bytes())
//     }
// }
