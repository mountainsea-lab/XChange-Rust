use crate::dto::meta::exchange_info::BinanceExchangeInfo;
use retrofit_rs::{Retrofit, RetrofitError, api, get};

#[api("https://fapi.binance.com")]
pub trait BinanceFuturesAuthed {
    /// Exchange info
    #[get("/fapi/v1/exchangeInfo")]
    async fn exchange_info(&self) -> Result<BinanceExchangeInfo, RetrofitError>;
}

impl BinanceFuturesAuthedClient {
    pub fn retrofit(&self) -> &Retrofit {
        &self.client
    }
}
