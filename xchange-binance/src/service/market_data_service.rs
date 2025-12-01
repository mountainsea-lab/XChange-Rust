use crate::binance_exchange::BinanceExchange;
use crate::dto::BinanceError;
use crate::dto::meta::binance_system::BinanceTime;
use crate::dto::meta::exchange_info::BinanceExchangeInfo;
use crate::service::market_data_service_inner::MarketDataInner;
use async_trait::async_trait;
use std::sync::Arc;
use xchange_core::dto::meta::ExchangeHealth;
use xchange_core::service::BaseService;
use xchange_core::service::marketdata::market_data_service::MarketDataService;

/// Binance Market Data Service
pub struct BinanceMarketDataService {
    inner: Arc<MarketDataInner>,
}

impl BinanceMarketDataService {
    /// 构造函数
    pub fn new(exchange: Arc<BinanceExchange>) -> Result<Self, BinanceError> {
        let client = MarketDataInner::new(exchange)?;
        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// 获取系统状态（占位方法）
    pub async fn get_system_status(&self) -> Result<String, BinanceError> {
        // TODO: 调用 client 获取系统状态
        unimplemented!("get_system_status not implemented yet")
    }

    // 调用宏批量代理方法
    delegate_client! {
        inner, {
            ping => (),
            binance_time => BinanceTime,
            exchange_info => BinanceExchangeInfo,
            future_exchange_info => BinanceExchangeInfo,
        }
    }
}

/// 实现 MarketDataService trait
#[async_trait]
impl MarketDataService for BinanceMarketDataService {
    /// 默认实现 ExchangeHealth
    async fn exchange_health(&self) -> ExchangeHealth {
        // TODO: 通过 get_system_status 调用 client，返回实际状态
        // 暂时返回占位值
        ExchangeHealth::Offline
    }

    // 其他 trait 方法可以继续占位实现
}
impl BaseService for BinanceMarketDataService {}
