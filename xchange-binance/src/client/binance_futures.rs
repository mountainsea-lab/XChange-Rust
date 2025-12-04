use crate::dto::meta::exchange_info::BinanceExchangeInfo;
use retrofit_rs::{Query, Retrofit, RetrofitError, api, get};

#[api("https://fapi.binance.com")]
pub trait BinanceFuturesAuthed {
    /// Exchange info
    #[get("/fapi/v1/exchangeInfo")]
    async fn exchange_info(&self) -> Result<BinanceExchangeInfo, RetrofitError>;

    #[get("/fapi/v3/klines")]
    #[allow(non_snake_case)]
    async fn klines(
        &self,
        symbol: Query<&str>,
        interval: Query<&str>,
        limit: Option<Query<u16>>,
        startTime: Option<Query<u64>>,
        endTime: Option<Query<u64>>,
    ) -> Result<Vec<Vec<serde_json::Value>>, RetrofitError>;
}

impl BinanceFuturesAuthedClient {
    pub fn retrofit(&self) -> &Retrofit {
        &self.client
    }
}
