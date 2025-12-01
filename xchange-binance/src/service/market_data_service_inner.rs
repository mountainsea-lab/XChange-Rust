use crate::binance_exchange::BinanceExchange;
use crate::dto::BinanceError;
use crate::dto::meta::binance_system::BinanceTime;
use crate::dto::meta::exchange_info::BinanceExchangeInfo;
use crate::service::binance_base_service::BinanceBaseService;
use std::sync::Arc;

/// 公共封装层：Binance Market Data 客户端
pub struct MarketDataInner {
    base: Arc<BinanceBaseService>,
}
impl MarketDataInner {
    pub fn new(exchange: Arc<BinanceExchange>) -> Result<Self, BinanceError> {
        let base = BinanceBaseService::new(exchange.clone())
            .map_err(|e| BinanceError::ServiceNotInitialized(e.to_string()))?;

        Ok(Self {
            base: Arc::new(base),
        })
    }

    pub async fn ping(&self) -> Result<(), BinanceError> {
        // TODO: 调用 exchange API
        unimplemented!("ping not implemented yet")
    }

    pub async fn binance_time(&self) -> Result<BinanceTime, BinanceError> {
        // TODO: 调用 exchange API
        unimplemented!("binance_time not implemented yet")
    }

    pub async fn exchange_info(&self) -> Result<BinanceExchangeInfo, BinanceError> {
        // TODO: 调用 exchange API，并加 retry / rate limiter
        unimplemented!("get_exchange_info not implemented yet")
    }

    pub async fn future_exchange_info(&self) -> Result<BinanceExchangeInfo, BinanceError> {
        // TODO: 调用 exchange API，并加 retry / rate limiter
        unimplemented!("get_exchange_info not implemented yet")
    }
}
