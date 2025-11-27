use crate::client::binance::BinancePubClient;
use crate::client::binance_authed::BinanceAuthenticatedClient;
use retrofit_rs::async_client::interceptors::AuthInterceptor;
use retrofit_rs::{Retrofit, RetrofitError};

pub mod binance;
pub mod binance_authed;

pub struct BinanceClient {
    pub public: BinancePubClient,
    pub auth: Option<BinanceAuthenticatedClient>,
}

impl BinanceClient {
    /// 创建公开接口客户端
    pub fn new_public(base_url: &str) -> Result<Self, RetrofitError> {
        let retrofit = Retrofit::builder().base_url(base_url).build()?;

        let public = BinancePubClient::with_client(retrofit);

        Ok(Self { public, auth: None })
    }

    /// 创建带鉴权客户端
    pub fn new_authenticated(base_url: &str, api_key: &str) -> Result<Self, RetrofitError> {
        let retrofit = Retrofit::builder()
            .base_url(base_url)
            .add_interceptor(AuthInterceptor::api_key("X-MBX-APIKEY", api_key))
            .build()?;

        let public = BinancePubClient::with_client(retrofit.clone());
        let auth = BinanceAuthenticatedClient::with_client(retrofit);

        Ok(Self {
            public,
            auth: Some(auth),
        })
    }
}
