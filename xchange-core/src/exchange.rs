use crate::TimeUnit::Milliseconds;
use crate::ValueFactory;
use crate::client::ResilienceRegistries;
use crate::dto::meta::exchange_metadata::ExchangeMetaData;
use crate::error::exchange_error::{ExchangeError, NotYetImplementedForExchangeError};
use crate::exchange_specification::ExchangeSpecification;
use crate::instrument::Instrument;
use crate::service::account::account_service::AccountService;
use crate::service::marketdata::market_data_service::MarketDataService;
use crate::service::trade::trade_service::TradeService;
use crate::utils::time_nonce::TimeNonce;
use async_trait::async_trait;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExchangeType {
    Spot,
    Futures,
    Inverse,
    PortfolioMargin,
}

impl fmt::Display for ExchangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExchangeType::Spot => "SPOT",
            ExchangeType::Futures => "FUTURES",
            ExchangeType::Inverse => "INVERSE",
            ExchangeType::PortfolioMargin => "PORTFOLIO_MARGIN",
        };
        write!(f, "{}", s)
    }
}

#[async_trait]
pub trait Exchange: Send + Sync {
    const USE_SANDBOX: &'static str = "Use_Sandbox";

    fn exchange_specification(&self) -> Arc<ExchangeSpecification>;

    fn exchange_meta_data(&self) -> Arc<ExchangeMetaData>;

    fn exchange_instruments(&self) -> Arc<Vec<Arc<dyn Instrument + Send + Sync>>>;

    fn nonce_factory(&self) -> Arc<dyn ValueFactory<u64>>;

    fn default_exchange_specification(&self) -> Arc<ExchangeSpecification>;

    fn apply_specification(&mut self, spec: ExchangeSpecification) -> Result<(), ExchangeError>;

    fn market_data_service(
        &self,
    ) -> Result<Arc<dyn MarketDataService + Send + Sync>, ExchangeError>;

    fn trade_service(&self) -> Result<Arc<dyn TradeService + Send + Sync>, ExchangeError>;

    fn account_service(&self) -> Result<Arc<dyn AccountService + Send + Sync>, ExchangeError>;

    async fn remote_init(&mut self) -> Result<(), ExchangeError> {
        Ok(())
    }

    async fn resilience_registries(&self) -> Result<Arc<ResilienceRegistries>, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message(
            "Resilience features not implemented".to_string(),
        )
        .into())
    }
}

pub struct BaseExchange {
    /// ExchangeSpecification，使用 RwLock 保证可动态修改
    pub spec: Arc<RwLock<ExchangeSpecification>>,
    pub meta_data: Arc<RwLock<ExchangeMetaData>>,
    pub nonce_factory: Arc<dyn ValueFactory<u64> + Send + Sync>,

    pub market_service: RwLock<Option<Arc<dyn MarketDataService + Send + Sync>>>,
    pub trade_service: RwLock<Option<Arc<dyn TradeService + Send + Sync>>>,
    pub account_service: RwLock<Option<Arc<dyn AccountService + Send + Sync>>>,
}

impl BaseExchange {
    pub fn new(default_spec: ExchangeSpecification, meta_data: ExchangeMetaData) -> Self {
        Self {
            spec: Arc::new(RwLock::new(default_spec)),
            meta_data: Arc::new(RwLock::new(meta_data)),
            nonce_factory: Arc::new(TimeNonce::new(Milliseconds)),
            market_service: RwLock::new(None),
            trade_service: RwLock::new(None),
            account_service: RwLock::new(None),
        }
    }

    /// 应用新的 ExchangeSpecification，线程安全
    pub async fn apply_specification(&self, spec: ExchangeSpecification) {
        // 合并默认值
        let merged_spec = self.merge_default_specification(spec);

        // 修改 RwLock 内部值
        let mut write_spec = self.spec.write();
        *write_spec = merged_spec;

        // 可选 JSON 初始化占位（加载 ExchangeMetaData）
        if write_spec.should_load_remote_meta_data {
            println!("Remote init placeholder (async load can be implemented here)");
            // 可异步 spawn tokio 任务加载 meta_data
        }

        // 初始化服务，子类可覆盖 init_services
        self.init_services().await;
    }

    /// 合并默认值（默认值来源可自定义）
    pub fn merge_default_specification(
        &self,
        spec: ExchangeSpecification,
    ) -> ExchangeSpecification {
        let default_spec_guard = self.spec.read(); // RwLockReadGuard<ExchangeSpecification>

        // Guard 已经可以当作 &ExchangeSpecification 使用
        let default_spec_ref: &ExchangeSpecification = &*default_spec_guard;

        ExchangeSpecification {
            exchange_name: spec
                .exchange_name
                .or(default_spec_ref.exchange_name.clone()),
            exchange_description: spec
                .exchange_description
                .or(default_spec_ref.exchange_description.clone()),
            ssl_uri: spec.ssl_uri.or(default_spec_ref.ssl_uri.clone()),
            host: spec.host.or(default_spec_ref.host.clone()),
            plain_text_uri: spec
                .plain_text_uri
                .or(default_spec_ref.plain_text_uri.clone()),
            exchange_specific_parameters: {
                let mut merged = default_spec_ref.exchange_specific_parameters.clone();
                merged.extend(spec.exchange_specific_parameters.clone());
                merged
            },
            should_load_remote_meta_data: spec.should_load_remote_meta_data,
            ..default_spec_ref.clone() // 这里直接用 clone，不再解引用
        }
    }

    /// 默认空实现，具体交易所可覆盖
    async fn init_services(&self) {
        println!(
            "BaseExchange.init_services placeholder, should be overridden by concrete exchange"
        );
    }

    /// 返回默认 ExchangeSpecification（可覆盖）
    fn default_exchange_specification(&self) -> ExchangeSpecification {
        self.spec.write().clone()
    }
}
