use crate::binance_time_provider::BinanceTimeProvider;
use crate::dto::BinanceError;
use parking_lot::RwLock;
use std::sync::Arc;
use xchange_core::ValueFactory;
use xchange_core::client::ResilienceRegistries;
use xchange_core::dto::meta::exchange_metadata::ExchangeMetaData;
use xchange_core::exchange::{BaseExchange, Exchange, ExchangeType};
use xchange_core::exchange_specification::ExchangeSpecification;
use xchange_core::instrument::Instrument;
use xchange_core::service::account::account_service::AccountService;
use xchange_core::service::marketdata::market_data_service::MarketDataService;
use xchange_core::service::trade::trade_service::TradeService;

// ----------------- 常量 -----------------
pub const EXCHANGE_TYPE_KEY: &str = "Exchange_Type";

pub const SPOT_URL: &str = "https://api.binance.com";
pub const FUTURES_URL: &str = "https://fapi.binance.com";
pub const INVERSE_FUTURES_URL: &str = "https://dapi.binance.com";
pub const PORTFOLIO_MARGIN_URL: &str = "https://papi.binance.com";

pub const SANDBOX_SPOT_URL: &str = "https://testnet.binance.vision";
pub const SANDBOX_FUTURES_URL: &str = "https://testnet.binancefuture.com";
pub const SANDBOX_INVERSE_FUTURES_URL: &str = "https://testnet.binancefuture.com";

// ----------------- BinanceExchange -----------------
pub struct BinanceExchange {
    /// 完全拥有 BaseExchange，spec 可动态修改
    pub base: Arc<BaseExchange>,

    /// 异步缓存服务器时间
    pub timestamp_provider: Arc<BinanceTimeProvider>,

    /// Resilience 注册表
    pub resilience_registries: Arc<ResilienceRegistries>,
}

impl BinanceExchange {
    /// --------------------------
    /// 构造函数，初始化阶段调用 apply_specification
    /// --------------------------
    pub async fn new(mut spec: ExchangeSpecification) -> Result<Self, BinanceError> {
        // 调整 host/ssl_uri/port
        Self::conclude_host_params(&mut spec);

        // 初始化 resilience
        let resilience_registries = Arc::new(ResilienceRegistries::new());

        // 初始化 BaseExchange
        let base = Arc::new(BaseExchange {
            spec: Arc::new(RwLock::new(spec.clone())),
            meta_data: Arc::new(RwLock::new(ExchangeMetaData::default())),
            nonce_factory: Arc::new(BinanceTimeProvider::new(
                spec.resilience,
                resilience_registries.clone(),
            )),
            market_service: None,
            trade_service: None,
            account_service: None,
        });

        // 初始化 timestamp_provider
        let timestamp_provider = Arc::new(BinanceTimeProvider::new(
            spec.resilience,
            resilience_registries.clone(),
        ));

        Ok(Self {
            base,
            timestamp_provider,
            resilience_registries,
        })
    }

    /// --------------------------
    /// 根据 ExchangeType / sandbox 调整 host/ssl_uri
    /// --------------------------
    fn conclude_host_params(spec: &mut ExchangeSpecification) {
        if let Some(exchange_type) = spec.exchange_specific_parameters.get(EXCHANGE_TYPE_KEY) {
            let ssl_uri = match exchange_type {
                ExchangeType::Spot => {
                    if spec.use_sandbox {
                        SANDBOX_SPOT_URL
                    } else {
                        SPOT_URL
                    }
                }
                ExchangeType::Futures => {
                    if spec.use_sandbox {
                        SANDBOX_FUTURES_URL
                    } else {
                        FUTURES_URL
                    }
                }
                ExchangeType::Inverse => {
                    if spec.use_sandbox {
                        SANDBOX_INVERSE_FUTURES_URL
                    } else {
                        INVERSE_FUTURES_URL
                    }
                }
                ExchangeType::PortfolioMargin => PORTFOLIO_MARGIN_URL,
            };
            spec.ssl_uri = Some(ssl_uri.to_string());
        }
    }

    /// --------------------------
    /// 工具方法：FUTURES?
    /// --------------------------
    pub async fn futures_enabled(&self) -> bool {
        let spec = self.base.spec.read();
        matches!(
            spec.exchange_specific_parameters.get(EXCHANGE_TYPE_KEY),
            Some(ExchangeType::Futures)
        )
    }

    /// --------------------------
    /// 工具方法：PORTFOLIO?
    /// --------------------------
    pub async fn portfolio_margin_enabled(&self) -> bool {
        let spec = self.base.spec.read();
        matches!(
            spec.exchange_specific_parameters.get(EXCHANGE_TYPE_KEY),
            Some(ExchangeType::PortfolioMargin)
        )
    }

    /// --------------------------
    /// 是否已认证
    /// --------------------------
    pub async fn authenticated(&self) -> bool {
        let spec = self.base.spec.read();
        spec.api_key.is_some() && spec.secret_key.is_some()
    }

    /// --------------------------
    /// 动态应用新的 ExchangeSpecification（线程安全）
    /// --------------------------
    pub async fn apply_specification(&self, mut spec: ExchangeSpecification) {
        Self::conclude_host_params(&mut spec);
        let mut spec_lock = self.base.spec.write();
        *spec_lock = spec;
    }

    /// 获取 Resilience 注册表（可懒初始化）
    pub fn get_resilience_registries(&self) -> Arc<ResilienceRegistries> {
        self.resilience_registries.clone()
    }
}

// ----------------- Exchange trait impl -----------------
#[async_trait::async_trait]
impl Exchange for BinanceExchange {
    fn exchange_specification(&self) -> Arc<ExchangeSpecification> {
        Arc::new(self.base.spec.write().clone())
    }

    fn exchange_meta_data(&self) -> Arc<ExchangeMetaData> {
        Arc::new(self.base.meta_data.write().clone())
    }

    fn exchange_instruments(&self) -> Arc<Vec<Arc<dyn Instrument + Send + Sync>>> {
        Arc::new(vec![])
    }

    fn nonce_factory(&self) -> Arc<dyn ValueFactory<u64>> {
        self.base.nonce_factory.clone()
    }
    //@Override
    //   public ExchangeSpecification getDefaultExchangeSpecification() {
    //     ExchangeSpecification spec = new ExchangeSpecification(this.getClass());
    //     spec.setSslUri(SPOT_URL);
    //     spec.setHost("www.binance.com");
    //     spec.setPort(80);
    //     spec.setExchangeName("Binance");
    //     spec.setExchangeDescription("Binance Exchange.");
    //     spec.setExchangeSpecificParametersItem(EXCHANGE_TYPE, SPOT);
    //     spec.setExchangeSpecificParametersItem(USE_SANDBOX, false);
    //     AuthUtils.setApiAndSecretKey(spec, "binance");
    //     return spec;
    //   }
    fn default_exchange_specification(&self) -> Arc<ExchangeSpecification> {
        todo!()
    }

    fn apply_specification(&mut self, mut spec: ExchangeSpecification) {
        Self::conclude_host_params(&mut spec);
        let mut spec_lock = self.base.spec.write(); // 注意：没有 .await
        *spec_lock = spec;
    }

    fn market_data_service(&self) -> Arc<dyn MarketDataService + Send + Sync> {
        self.base.market_service.as_ref().unwrap().clone()
    }

    fn trade_service(&self) -> Arc<dyn TradeService + Send + Sync> {
        self.base.trade_service.as_ref().unwrap().clone()
    }

    fn account_service(&self) -> Arc<dyn AccountService + Send + Sync> {
        self.base.account_service.as_ref().unwrap().clone()
    }
}
