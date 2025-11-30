use crate::client::binance_authed::BinanceAuthedClient;
use crate::client::binance_futures::BinanceFuturesAuthedClient;
use std::sync::Arc;
use xchange_core::rescu::params_digest::ParamsDigest;

pub struct BinanceBaseService {
    pub api_key: String,

    pub spot: Arc<BinanceAuthedClient>,
    pub futures: Option<Arc<BinanceFuturesAuthedClient>>,
    pub inverse_futures: Option<Arc<BinanceFuturesAuthedClient>>,

    pub digest: Arc<dyn ParamsDigest + Send + Sync>,
}
