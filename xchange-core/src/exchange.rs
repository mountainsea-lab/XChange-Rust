use crate::dto::meta::exchange_metadata::ExchangeMetaData;
use crate::error::exchange_error::{ExchangeError, NotYetImplementedForExchangeError};
use crate::exchange_specification::ExchangeSpecification;
use crate::instrument::Instrument;
use crate::service::account::account_service::AccountService;
use crate::service::marketdata::market_data_service::MarketDataService;
use crate::service::trade::trade_service::TradeService;
use async_trait::async_trait;
use std::sync::Arc;

/// Exchange trait
#[async_trait]
pub trait Exchange: Send + Sync {
    const USE_SANDBOX: &'static str = "Use_Sandbox";

    /// 获取 ExchangeSpecification
    fn exchange_specification(&self) -> Box<ExchangeSpecification<Self>>
    where
        Self: Sized;

    /// 获取 ExchangeMetaData
    fn exchange_meta_data(&self) -> &ExchangeMetaData;

    /// 获取所有交易对信息
    fn exchange_instruments(&self) -> Vec<Box<dyn Instrument>>;

    /// 获取默认的 ExchangeSpecification
    fn default_exchange_specification(&self) -> Box<ExchangeSpecification<Self>>
    where
        Self: Sized;

    /// 应用交易所特定的配置
    fn apply_specification(&mut self, spec: ExchangeSpecification<Self>)
    where
        Self: Sized;

    /// 获取市场数据服务
    fn get_market_data_service(&self) -> Arc<Box<dyn MarketDataService>>;

    /// 获取交易服务
    fn get_trade_service(&self) -> Arc<Box<dyn TradeService>>;

    /// 获取账户服务
    fn get_account_service(&self) -> Arc<Box<dyn AccountService>>;

    /// 获取 nonce 工厂
    // fn get_nonce_factory(&self) -> Arc<dyn SynchronizedValueFactory<u64>>;

    /// 远程初始化，提供默认实现
    async fn remote_init(&mut self) -> Result<(), ExchangeError> {
        Ok(()) // 默认返回成功，子类可以重载
    }

    /// 获取 resilience4j 注册表（用于重试策略、速率限制等）
    async fn get_resilience_registries(&self) -> Result<String, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message(
            "Resilience features not implemented".to_string(),
        )
        .into())
    }
}
