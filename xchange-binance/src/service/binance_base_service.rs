use crate::binance_time_provider::BinanceTimeProvider;
use crate::dto::BinanceError;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;
use xchange_core::TimeUnit;
use xchange_core::client::ResilienceRegistries;
use xchange_core::dto::meta::exchange_metadata::ExchangeMetaData;
use xchange_core::exchange::BaseExchange;
use xchange_core::exchange_specification::ExchangeSpecification;
use xchange_core::service::marketdata::market_data_service::MarketDataService;
use xchange_core::utils::time_nonce::TimeNonce;

/// ----------------- BinanceExchange -----------------
pub struct BinanceExchange {
    /// 组合 BaseExchange
    pub base_exchange: Arc<BaseExchange>,

    /// 异步安全的 Binance 时间提供者
    pub timestamp_provider: Arc<Mutex<BinanceTimeProvider>>,

    /// 共享 Resilience 集合（重试/限流）
    pub resilience_registries: Arc<ResilienceRegistries>,
}

impl BinanceExchange {
    /// 构建 BinanceExchange
    pub async fn new(spec: Arc<ExchangeSpecification>) -> Result<Self, BinanceError> {
        let resilience_registries = Arc::new(ResilienceRegistries::new());

        // 初始化 BaseExchange
        let base_exchange = Arc::new(BaseExchange {
            spec: spec.clone(),
            meta_data: Arc::new(ExchangeMetaData::default()),
            nonce_factory: Arc::new(TimeNonce::new(TimeUnit::Milliseconds)),
            market_service: None,
            trade_service: None,
            account_service: None,
        });

        // 初始化异步 Binance 时间提供者
        let timestamp_provider = Arc::new(Mutex::new(BinanceTimeProvider::new(
            spec.resilience.clone(),
            resilience_registries.clone(),
        )));

        let mut exchange = Self {
            base_exchange,
            timestamp_provider,
            resilience_registries,
        };

        // 初始化服务
        exchange.init_services().await?;

        Ok(exchange)
    }

    /// 初始化具体服务
    async fn init_services(&mut self) -> Result<(), BinanceError> {
        let base = self.base_exchange.clone();
        let registries = self.resilience_registries.clone();

        // todo Market service
        // let market_service: Arc<dyn MarketDataService + Send + Sync> = Arc::new(
        //     crate::services::BinanceMarketDataService::new(base.clone(), registries.clone()),
        // );
        //
        // // Trade service
        // let trade_service: Arc<dyn TradeService + Send + Sync> = Arc::new(
        //     crate::services::BinanceTradeService::new(base.clone(), registries.clone()),
        // );
        //
        // // Account service
        // let account_service: Arc<dyn AccountService + Send + Sync> = Arc::new(
        //     crate::services::BinanceAccountService::new(base.clone(), registries.clone()),
        // );
        //
        // // 注入服务到 BaseExchange
        // let mut base = Arc::get_mut(&mut self.base_exchange).unwrap();
        // base.market_service = Some(market_service);
        // base.trade_service = Some(trade_service);
        // base.account_service = Some(account_service);

        Ok(())
    }

    /// 异步获取服务器时间（10 分钟缓存）
    pub async fn get_server_time(&self) -> Result<i64, BinanceError> {
        let mut provider = self.timestamp_provider.lock().await;
        provider
            .delta_server_time(|| async {
                // fetch closure，可以是 Retrofit/Rust HTTP 客户端请求
                crate::services::binance_time_fetch().await
            })
            .await
    }

    /// 远程初始化 Exchange Metadata
    pub async fn remote_init(&self) -> Result<(), BinanceError> {
        // todo: 远程初始化 MetaData
        // let market_service = self
        //     .base_exchange
        //     .market_service
        //     .as_ref()
        //     .ok_or(BinanceError::ServiceNotInitialized("MarketService"))?
        //     .clone();
        //
        // let account_service = self
        //     .base_exchange
        //     .account_service
        //     .as_ref()
        //     .ok_or(BinanceError::ServiceNotInitialized("AccountService"))?
        //     .clone();
        //
        // // 调用市场信息接口
        // let exchange_info = market_service.get_exchange_info().await?;
        //
        // // 获取账户资产信息（非 sandbox）
        // let assets = account_service.get_asset_details().await.ok();
        //
        // // 更新 BaseExchange MetaData
        // let mut meta = Arc::get_mut(&mut self.base_exchange.meta_data).unwrap();
        // meta.update_from_exchange_info(exchange_info, assets);

        Ok(())
    }

    /// 是否使用 Futures
    pub fn is_futures_enabled(&self) -> bool {
        matches!(self.base_exchange.spec.exchange_type.as_str(), "FUTURES")
    }

    /// 是否使用 Portfolio Margin
    pub fn is_portfolio_margin(&self) -> bool {
        matches!(
            self.base_exchange.spec.exchange_type.as_str(),
            "PORTFOLIO_MARGIN"
        )
    }

    /// 是否使用 Sandbox
    pub fn using_sandbox(&self) -> bool {
        self.base_exchange.spec.use_sandbox
    }
}
