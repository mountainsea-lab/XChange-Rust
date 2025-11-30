use crate::client::binance::BinancePubClient;
use crate::client::binance_authed::BinanceAuthedClient;
use crate::client::binance_futures::BinanceFuturesAuthedClient;
use crate::client::binance_futures::BinanceFuturesClient;
use retrofit_rs::async_client::interceptors::AuthInterceptor;
use retrofit_rs::{Retrofit, RetrofitError};

pub mod binance;
pub mod binance_authed;
pub(crate) mod binance_futures;

pub struct BinanceClient {
    pub public: BinancePubClient,
    pub auth: Option<BinanceAuthedClient>,
    pub futures: Option<BinanceFuturesClient>,
    pub futures_authed: Option<BinanceFuturesAuthedClient>,
}

impl BinanceClient {
    /// 创建公开接口客户端
    pub fn new_public(base_url: &str) -> Result<Self, RetrofitError> {
        let retrofit = Retrofit::builder().base_url(base_url).build()?;

        let public = BinancePubClient::with_client(retrofit);

        Ok(Self {
            public,
            auth: None,
            futures: None,
            futures_authed: None,
        })
    }

    /// 创建带鉴权客户端
    pub fn new_authenticated(base_url: &str, api_key: &str) -> Result<Self, RetrofitError> {
        let retrofit = Retrofit::builder()
            .base_url(base_url)
            .add_interceptor(AuthInterceptor::api_key("X-MBX-APIKEY", api_key))
            .build()?;

        let public = BinancePubClient::with_client(retrofit.clone());
        let auth = BinanceAuthedClient::with_client(retrofit);

        Ok(Self {
            public,
            auth: Some(auth),
            futures: None,
            futures_authed: None,
        })
    }

    /// 单独构建 futures/inverse 客户端并 attach 到 self（在 Arc 之前调用）
    /// - futures_url: FUTURES_URL or INVERSE_FUTURES_URL
    pub fn new_authed(&mut self, futures_url: &str, api_key: &str) -> Result<(), RetrofitError> {
        let fut_retrofit = Retrofit::builder()
            .base_url(futures_url)
            .add_interceptor(AuthInterceptor::api_key("X-MBX-APIKEY", api_key))
            .build()?;

        let fut_pub = BinanceFuturesClient::with_client(fut_retrofit.clone());
        let fut_auth = BinanceFuturesAuthedClient::with_client(fut_retrofit);

        self.futures = Some(fut_pub);
        self.futures_authed = Some(fut_auth);
        Ok(())
    }
}
