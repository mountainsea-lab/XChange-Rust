use crate::dto::meta::binance_system::{BinanceSystemStatus, BinanceTime};
use crate::dto::meta::exchange_info::BinanceExchangeInfo;
use retrofit_rs::{Body, Header, Query, Retrofit, RetrofitError, api, get, post};

#[api("https://api.binance.com")]
pub trait BinanceAuthed {
    /// Fetch system status
    #[get("/sapi/v1/system/status")]
    async fn system_status(&self) -> Result<BinanceSystemStatus, RetrofitError>;

    /// Ping
    #[get("/api/v3/ping")]
    async fn ping(&self) -> Result<serde_json::Value, RetrofitError>;

    /// Get server time
    #[get("/api/v3/time")]
    async fn time(&self) -> Result<BinanceTime, RetrofitError>;

    /// Exchange info
    #[get("/api/v3/exchangeInfo")]
    async fn exchange_info(&self) -> Result<BinanceExchangeInfo, RetrofitError>;

    #[get("/api/v3/order")]
    async fn order_status(
        &self,
        symbol: Query<&str>,
        order_id: Query<u64>,
        orig_client_order_id: Query<&str>,
        recv_window: Query<u64>,
        timestamp: Query<u64>,
    ) -> Result<serde_json::Value, RetrofitError>;

    #[get("/api/v3/klines")]
    #[allow(non_snake_case)]
    async fn klines(
        &self,
        symbol: Query<&str>,
        interval: Query<&str>,
        limit: Query<u16>,
        startTime: Query<u64>,
        endTime: Query<u64>,
    ) -> Result<Vec<Vec<serde_json::Value>>, RetrofitError>;

    // DELETE, PUT 等方法同理
}

impl BinanceAuthedClient {
    pub fn retrofit(&self) -> &Retrofit {
        &self.client
    }
}
