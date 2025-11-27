// 允许这些参数属性存在
#![allow(non_snake_case)]
#![allow(unused_attributes)]

use crate::client::binance::Binance;
use async_trait::async_trait;
use std::sync::Arc;
use xchange_core::rescu::HttpError;
use xchange_core::rescu::resilient_http_client::ResilientHttpClient;
use xchange_macros::api_get;
use crate::dto::BinanceError;

pub mod binance;

pub struct BinanceClient {
    pub client: Arc<ResilientHttpClient>,
    pub base_url: String,
}

impl BinanceClient {
    /// 构造函数
    pub fn new(client: Arc<ResilientHttpClient>, base_url: impl Into<String>) -> Self {
        Self {
            client,
            base_url: base_url.into(),
        }
    }
}

/// 将 trait 方法委托给 ResilientHttpClient 执行
#[async_trait]
impl Binance for BinanceClient {
    #[api_get("/sapi/v1/system/status")]
    async fn system_status(&self) -> Result<BinanceSystemStatus, BinanceError> {
        // 宏展开会生成内部调用 self.client.execute(...) 的逻辑
    }

    #[api_get("/api/v3/ping")]
    async fn ping(&self) -> Result<serde_json::Value, BinanceError> {}

    #[api_get("/api/v3/time")]
    async fn time(&self) -> Result<BinanceTime, BinanceError> {}

    #[api_get("/api/v3/exchangeInfo")]
    async fn exchange_info(&self) -> Result<BinanceExchangeInfo, BinanceError> {}

    #[api_get(
        "/api/v3/klines",
        query(symbol, interval, limit, start_time, end_time)
    )]
    async fn klines(
        &self,
        symbol: &str,
        interval: &str,
        limit: Option<u32>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<Vec<Vec<serde_json::Value>>, BinanceError> {
        // 函数体保持原逻辑
    }
}
