use crate::binance_exchange::BinanceExchange;
use crate::dto::BinanceError;
use crate::service::binance_base_service::BinanceBaseService;
use std::sync::Arc;

/// 公共封装层：Binance Market Data 客户端
pub struct MarketDataServiceInner {
    base: Arc<BinanceBaseService>,
}
impl MarketDataServiceInner {
    pub fn new(exchange: Arc<BinanceExchange>) -> Result<Self, BinanceError> {
        let base = BinanceBaseService::new(exchange.clone())
            .map_err(|e| BinanceError::ServiceNotInitialized(e.to_string()))?;

        Ok(Self {
            base: Arc::new(base),
        })
    }

    /// ping 方法占位
    pub async fn ping(&self) -> Result<(), BinanceError> {
        // TODO: 调用 exchange API
        unimplemented!("ping not implemented yet")
    }

    /// 获取 Binance 时间
    pub async fn binance_time(&self) -> Result<(), BinanceError> {
        // TODO: 调用 exchange API
        unimplemented!("binance_time not implemented yet")
    }

    /// 获取交易所信息
    pub async fn get_exchange_info(&self) -> Result<(), BinanceError> {
        // TODO: 调用 exchange API，并加 retry / rate limiter
        unimplemented!("get_exchange_info not implemented yet")
    }
}
