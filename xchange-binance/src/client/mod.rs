use crate::binance_exchange::{FUTURES_URL, INVERSE_FUTURES_URL};
use crate::client::binance_futures::BinanceFuturesAuthedClient;
use crate::client::binance_spot::BinanceAuthedClient;
use retrofit_rs::async_client::interceptors::AuthInterceptor;
use retrofit_rs::{Retrofit, RetrofitError};
use std::sync::Arc;
use xchange_core::exchange::ExchangeType;

pub(crate) mod binance_futures;
pub mod binance_spot;

pub struct BinanceClient {
    /// Spot API（带鉴权，默认会创建）
    pub spot: Arc<BinanceAuthedClient>,

    /// USDT-M Futures（可选）
    pub futures: Option<Arc<BinanceFuturesAuthedClient>>,

    /// Inverse Futures（可选）
    pub futures_inverse: Option<Arc<BinanceFuturesAuthedClient>>,
}

#[derive(Debug, Clone)]
pub struct BinanceClientBuilder<'a> {
    base_url: &'a str,
    api_key: Option<&'a str>,
    exchange_type: ExchangeType,
}

impl<'a> BinanceClientBuilder<'a> {
    /// 创建 builder，默认 exchange_type = Spot
    pub fn new(base_url: &'a str) -> Self {
        Self {
            base_url,
            api_key: None,
            exchange_type: ExchangeType::Spot,
        }
    }

    /// 设置 API Key
    pub fn api_key(mut self, key: &'a str) -> Self {
        self.api_key = Some(key);
        self
    }

    /// 设置交易类型（Spot / Futures / Inverse / PortfolioMargin）
    pub fn exchange_type(mut self, t: ExchangeType) -> Self {
        self.exchange_type = t;
        self
    }

    /// 构建客户端
    pub fn build(self) -> Result<BinanceClient, RetrofitError> {
        // 内部辅助函数：创建 client
        fn make_client<T>(
            base_url: &str,
            api_key: Option<&str>,
            constructor: impl Fn(Retrofit) -> T,
        ) -> Result<Arc<T>, RetrofitError> {
            let mut builder = Retrofit::builder().base_url(base_url);
            if let Some(key) = api_key {
                builder = builder.add_interceptor(AuthInterceptor::api_key("X-MBX-APIKEY", key));
            }
            let retrofit = builder.build()?;
            Ok(Arc::new(constructor(retrofit)))
        }

        // ---------------------
        // 1) Spot client
        // ---------------------
        let spot = make_client(
            self.base_url,
            self.api_key,
            BinanceAuthedClient::with_client,
        )?;

        // ---------------------
        // 2) Futures / Inverse client
        // ---------------------
        let (futures, futures_inverse) = match self.exchange_type {
            ExchangeType::Futures | ExchangeType::PortfolioMargin => (
                Some(make_client(
                    FUTURES_URL,
                    self.api_key,
                    BinanceFuturesAuthedClient::with_client,
                )?),
                None,
            ),
            ExchangeType::Inverse => (
                None,
                Some(make_client(
                    INVERSE_FUTURES_URL,
                    self.api_key,
                    BinanceFuturesAuthedClient::with_client,
                )?),
            ),
            ExchangeType::Spot => (None, None),
        };

        Ok(BinanceClient {
            spot,
            futures,
            futures_inverse,
        })
    }
}
