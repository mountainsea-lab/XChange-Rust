// use crate::binance::Binance;
// use crate::dto::BinanceError;
// use crate::dto::meta::binance_system::BinanceSystemStatus;
// use crate::dto::meta::binance_system::BinanceTime;
// use crate::dto::meta::exchange_info::BinanceExchangeInfo;
// use async_trait::async_trait;
// use std::sync::Arc;
// use xchange_core::rescu::HttpError;
// use xchange_core::rescu::resilient_http_client::ResilientHttpClient;
// use xchange_macros::api_get;
//
// pub struct BinanceClient {
//     pub client: Arc<ResilientHttpClient>, // 来自 xchange-core
//     pub base_url: String,
// }
//
// #[async_trait]
// impl Binance for BinanceClient {
//     #[api_get("/sapi/v1/system/status")]
//     async fn system_status(&self) -> Result<BinanceSystemStatus, BinanceError> {
//         unimplemented!()
//     }
//
//     #[api_get("/api/v3/ping")]
//     async fn ping(&self) -> Result<serde_json::Value, BinanceError> {
//         unimplemented!()
//     }
//
//     #[api_get("/api/v3/time")]
//     async fn time(&self) -> Result<BinanceTime, BinanceError> {
//         unimplemented!()
//     }
//
//     #[api_get("/api/v3/exchangeInfo")]
//     async fn exchange_info(&self) -> Result<BinanceExchangeInfo, BinanceError> {
//         unimplemented!()
//     }
//
//     #[api_get(
//         "/api/v3/klines",
//         query(symbol, interval, limit),
//         query(start_time, end_time)
//     )]
//     async fn klines(
//         &self,
//         symbol: &str,
//         interval: &str,
//         limit: Option<u32>,
//         start_time: Option<u64>,
//         end_time: Option<u64>,
//     ) -> Result<Vec<Vec<serde_json::Value>>, BinanceError> {
//         unimplemented!()
//     }
// }
