use crate::binance_time_provider::BinanceTimeProvider;
use crate::dto::BinanceError;
use crate::service::account_service::BinanceAccountService;
use crate::service::market_data_service::BinanceMarketDataService;
use parking_lot::RwLock;
use std::sync::{Arc, Weak};
use xchange_core::ValueFactory;
use xchange_core::client::ResilienceRegistries;
use xchange_core::dto::meta::exchange_metadata::ExchangeMetaData;
use xchange_core::error::exchange_error::{ExchangeError, ExchangeUnavailableError};
use xchange_core::exchange::{BaseExchange, Exchange, ExchangeType};
use xchange_core::exchange_specification::{ExchangeParam, ExchangeSpecification};
use xchange_core::instrument::Instrument;
use xchange_core::service::account::account_service::AccountService;
use xchange_core::service::marketdata::market_data_service::MarketDataService;
use xchange_core::service::trade::trade_service::TradeService;
use xchange_core::utils::auth_utils::AuthUtils;

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
    self_arc: Weak<BinanceExchange>,

    /// 完全拥有 BaseExchange，spec 可动态修改
    pub base: Arc<BaseExchange>,

    /// 异步缓存服务器时间
    pub timestamp_provider: Arc<BinanceTimeProvider>,

    /// Resilience 注册表
    pub resilience_registries: Arc<ResilienceRegistries>,
}

impl BinanceExchange {
    pub async fn new() -> Result<Self, BinanceError> {
        let resilience_registries = Arc::new(ResilienceRegistries::new());
        let spec = Self::default_exchange_specification();
        let exchange = Self {
            base: Arc::new(BaseExchange {
                spec: Arc::new(RwLock::new(spec.clone())),
                meta_data: Arc::new(RwLock::new(ExchangeMetaData::default())),
                nonce_factory: Arc::new(BinanceTimeProvider::new(
                    spec.resilience,
                    resilience_registries.clone(),
                )),
                market_service: RwLock::new(None),
                trade_service: RwLock::new(None),
                account_service: RwLock::new(None),
            }),
            timestamp_provider: Arc::new(BinanceTimeProvider::new(
                spec.resilience,
                resilience_registries.clone(),
            )),
            resilience_registries,
            self_arc: Weak::new(), // 之后再设置 weak_self
        };
        Ok(exchange)
    }

    /// 默认初始化（开箱即用）
    pub async fn default() -> Result<Arc<Self>, BinanceError> {
        let spec = Self::default_exchange_specification();
        Self::with_specification(spec).await
    }

    pub async fn with_specification(
        mut spec: ExchangeSpecification,
    ) -> Result<Arc<Self>, BinanceError> {
        let resilience_registries = Arc::new(ResilienceRegistries::new());

        Self::conclude_host_params(&mut spec);

        // 占位 Arc<Self> 用于初始化服务
        let exchange = Arc::new_cyclic(|weak_self| {
            let base = Arc::new(BaseExchange {
                spec: Arc::new(RwLock::new(spec.clone())),
                meta_data: Arc::new(RwLock::new(ExchangeMetaData::default())),
                nonce_factory: Arc::new(BinanceTimeProvider::new(
                    spec.resilience,
                    resilience_registries.clone(),
                )),

                market_service: RwLock::new(None),
                trade_service: RwLock::new(None),
                account_service: RwLock::new(None),
            });

            let timestamp_provider = Arc::new(BinanceTimeProvider::new(
                spec.resilience,
                resilience_registries.clone(),
            ));

            Self {
                base,
                timestamp_provider,
                resilience_registries,
                self_arc: weak_self.clone(),
            }
        });

        exchange.init_services()?;

        Ok(exchange)
    }

    pub fn init_services(&self) -> Result<(), BinanceError> {
        let exchange_ref = self.clone_exchange_ref();

        // 1. 初始化 MarketDataService
        let market_service = BinanceMarketDataService::new(exchange_ref.clone())?;
        *self.base.market_service.write() = Some(Arc::new(market_service));

        // 2. 初始化 AccountService
        let account_service = BinanceAccountService::new(exchange_ref)?;
        *self.base.account_service.write() = Some(Arc::new(account_service));

        Ok(())
    }

    pub fn clone_exchange_ref(&self) -> Arc<BinanceExchange> {
        self.self_arc
            .upgrade()
            .expect("Exchange self_arc has been dropped unexpectedly")
    }

    /// --------------------------
    /// 根据 ExchangeType / sandbox 调整 host/ssl_uri
    /// --------------------------
    pub fn conclude_host_params(spec: &mut ExchangeSpecification) {
        if let Some(param) = spec.exchange_specific_parameters.get(EXCHANGE_TYPE_KEY) {
            if let ExchangeParam::ExchangeType(exchange_type) = param {
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
    }

    /// --------------------------
    /// 工具方法：FUTURES?
    /// --------------------------
    pub async fn futures_enabled(&self) -> bool {
        let spec = self.base.spec.read();
        matches!(
            spec.exchange_specific_parameters.get(EXCHANGE_TYPE_KEY),
            Some(ExchangeParam::ExchangeType(ExchangeType::Futures))
                | Some(ExchangeParam::ExchangeType(ExchangeType::PortfolioMargin))
        )
    }

    /// --------------------------
    /// 工具方法：PORTFOLIO?
    /// --------------------------
    pub async fn portfolio_margin_enabled(&self) -> bool {
        let spec = self.base.spec.read();
        matches!(
            spec.exchange_specific_parameters.get(EXCHANGE_TYPE_KEY),
            Some(ExchangeParam::ExchangeType(ExchangeType::PortfolioMargin))
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
    pub fn apply_specification(
        &mut self,
        mut spec: ExchangeSpecification,
    ) -> Result<(), BinanceError> {
        Self::conclude_host_params(&mut spec);

        // 更新 spec
        *self.base.spec.write() = spec;

        // 重新初始化依赖服务
        self.init_services()?;

        // // 更新 timestamp_provider
        // self.timestamp_provider = Arc::new(BinanceTimeProvider::new(
        //     self.base.spec.read().resilience.clone(),
        //     self.resilience_registries.clone(),
        // ));

        Ok(())
    }

    /// 获取 Resilience 注册表（可懒初始化）
    pub fn get_resilience_registries(&self) -> Arc<ResilienceRegistries> {
        self.resilience_registries.clone()
    }

    pub fn default_exchange_specification() -> ExchangeSpecification {
        let mut spec = ExchangeSpecification {
            exchange_name: Some("Binance".into()),
            exchange_description: Some("Binance Exchange.".into()),
            ssl_uri: Some(SPOT_URL.into()),
            host: Some("www.binance.com".into()),
            port: 80,

            // 其余 Option 字段保持默认
            ..ExchangeSpecification::builder().build()
        };

        // 设置交易所特定参数
        spec.exchange_specific_parameters
            .insert("EXCHANGE_TYPE".into(), ExchangeParam::String("SPOT".into()));
        spec.exchange_specific_parameters
            .insert("USE_SANDBOX".into(), ExchangeParam::Boolean(false));

        // 加载 API Key / Secret
        AuthUtils::set_api_and_secret_key(&mut spec, Some("binance"));

        spec
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

    fn default_exchange_specification(&self) -> Arc<ExchangeSpecification> {
        Arc::new(Self::default_exchange_specification())
    }

    /// --------------------------
    /// 动态应用新的 ExchangeSpecification（线程安全）
    /// --------------------------
    fn apply_specification(
        &mut self,
        mut spec: ExchangeSpecification,
    ) -> Result<(), ExchangeError> {
        Self::conclude_host_params(&mut spec);

        // 更新 spec
        *self.base.spec.write() = spec;

        // 重新初始化依赖服务
        self.init_services()?;

        // 更新 timestamp_provider
        self.timestamp_provider = Arc::new(BinanceTimeProvider::new(
            self.base.spec.read().resilience.clone(),
            self.resilience_registries.clone(),
        ));

        Ok(())
    }

    fn market_data_service(
        &self,
    ) -> Result<Arc<dyn MarketDataService + Send + Sync>, ExchangeError> {
        let guard = self.base.market_service.read();
        guard.as_ref().cloned().ok_or_else(|| {
            ExchangeUnavailableError::with_message("MarketDataService".to_string()).into()
        })
    }

    fn trade_service(&self) -> Result<Arc<dyn TradeService + Send + Sync>, ExchangeError> {
        let guard = self.base.trade_service.read();
        guard.as_ref().cloned().ok_or_else(|| {
            ExchangeUnavailableError::with_message("TradeService".to_string()).into()
        })
    }

    fn account_service(&self) -> Result<Arc<dyn AccountService + Send + Sync>, ExchangeError> {
        let guard = self.base.account_service.read();
        guard.as_ref().cloned().ok_or_else(|| {
            ExchangeUnavailableError::with_message("AccountService".to_string()).into()
        })
    }
}
