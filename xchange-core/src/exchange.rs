use crate::TimeUnit::Milliseconds;
use crate::client::resilience_registries::ResilienceRegistries;
use crate::dto::meta::exchange_metadata::ExchangeMetaData;
use crate::error::exchange_error::{ExchangeError, NotYetImplementedForExchangeError};
use crate::exchange_specification::ExchangeSpecification;
use crate::instrument::Instrument;
use crate::service::account::account_service::AccountService;
use crate::service::marketdata::market_data_service::MarketDataService;
use crate::service::trade::trade_service::TradeService;
use crate::utils::time_nonce::TimeNonce;
use async_trait::async_trait;
use std::fmt;
use std::sync::Arc;

#[async_trait]
pub trait Exchange: Send + Sync {
    const USE_SANDBOX: &'static str = "Use_Sandbox";

    fn exchange_specification(&self) -> Arc<ExchangeSpecification>;

    fn exchange_meta_data(&self) -> Arc<ExchangeMetaData>;

    fn exchange_instruments(&self) -> Arc<Vec<Arc<dyn Instrument + Send + Sync>>>;

    fn nonce_factory(&self) -> Arc<TimeNonce>;

    fn default_exchange_specification(&self) -> Arc<ExchangeSpecification>;

    fn apply_specification(&mut self, spec: ExchangeSpecification);

    fn market_data_service(&self) -> Arc<dyn MarketDataService + Send + Sync>;

    fn trade_service(&self) -> Arc<dyn TradeService + Send + Sync>;

    fn account_service(&self) -> Arc<dyn AccountService + Send + Sync>;

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

/// ----------------- BaseExchange -----------------
pub struct BaseExchange {
    spec: Arc<ExchangeSpecification>,
    meta_data: Arc<ExchangeMetaData>,
    nonce_factory: Arc<TimeNonce>,

    market_service: Option<Arc<dyn MarketDataService + Send + Sync>>,
    trade_service: Option<Arc<dyn TradeService + Send + Sync>>,
    account_service: Option<Arc<dyn AccountService + Send + Sync>>,
}

impl BaseExchange {
    pub fn new(default_spec: ExchangeSpecification, meta_data: ExchangeMetaData) -> Self {
        Self {
            spec: Arc::new(default_spec),
            meta_data: Arc::new(meta_data),
            nonce_factory: Arc::new(TimeNonce::new(Milliseconds)),
            market_service: None,
            trade_service: None,
            account_service: None,
        }
    }
    pub fn apply_specification(&mut self, spec: ExchangeSpecification) {
        // 合并默认值
        let merged_spec = self.merge_default_specification(spec);
        self.spec = Arc::new(merged_spec);

        // 可选 JSON 初始化占位（加载 ExchangeMetaData）
        if self.spec.should_load_remote_meta_data {
            println!("Remote init placeholder (async load can be implemented here)");
            // 这里可以用 tokio::spawn(async { self.remote_init().await; });
        }

        // 初始化服务，子类可以覆盖 init_services
        self.init_services();
    }

    /// 合并默认值
    fn merge_default_specification(&self, spec: ExchangeSpecification) -> ExchangeSpecification {
        let default_spec = self.default_exchange_specification();

        ExchangeSpecification {
            exchange_name: spec.exchange_name.or(default_spec.exchange_name.clone()),
            exchange_description: spec
                .exchange_description
                .or(default_spec.exchange_description.clone()),
            ssl_uri: spec.ssl_uri.or(default_spec.ssl_uri.clone()),
            host: spec.host.or(default_spec.host.clone()),
            plain_text_uri: spec.plain_text_uri.or(default_spec.plain_text_uri.clone()),
            exchange_specific_parameters: {
                let mut merged = default_spec.exchange_specific_parameters.clone();
                merged.extend(spec.exchange_specific_parameters.clone());
                merged
            },
            should_load_remote_meta_data: spec.should_load_remote_meta_data,
            // 剩余字段使用结构体更新语法继承默认值
            ..(*default_spec).clone()
        }
    }

    /// 抽象方法，子类实现
    fn init_services(&mut self) {
        // 默认空实现，具体交易所实现覆盖
        println!("BaseExchange.init_services placeholder, should be overridden");
    }

    /// 可选 JSON 加载占位
    pub fn load_exchange_meta_data_from_json(&mut self, _json_data: &str) {
        // 占位：可以实现 JSON 解析逻辑，填充 meta_data
        // self.meta_data = Arc::new(parsed_meta_data);
        println!("JSON metadata loading placeholder");
    }
}

#[async_trait]
impl Exchange for BaseExchange {
    fn exchange_specification(&self) -> Arc<ExchangeSpecification> {
        self.spec.clone()
    }

    fn exchange_meta_data(&self) -> Arc<ExchangeMetaData> {
        self.meta_data.clone()
    }

    fn exchange_instruments(&self) -> Arc<Vec<Arc<dyn Instrument + Send + Sync>>> {
        todo!()
        // self.instruments.clone()
    }

    fn nonce_factory(&self) -> Arc<TimeNonce> {
        self.nonce_factory.clone()
    }

    fn default_exchange_specification(&self) -> Arc<ExchangeSpecification> {
        self.spec.clone()
    }

    fn apply_specification(&mut self, spec: ExchangeSpecification) {
        let merged_spec = self.merge_default_specification(spec);
        self.spec = Arc::new(merged_spec);

        // 可选 JSON 初始化占位
        if self.spec.should_load_remote_meta_data {
            tokio::spawn(async move {
                // async remote init placeholder
                println!("Remote init placeholder");
            });
        }

        // 初始化服务，留给子类实现
        self.init_services();
    }

    fn market_data_service(&self) -> Arc<dyn MarketDataService + Send + Sync> {
        self.market_service
            .as_ref()
            .expect("MarketDataService not initialized")
            .clone()
    }

    fn trade_service(&self) -> Arc<dyn TradeService + Send + Sync> {
        self.trade_service
            .as_ref()
            .expect("TradeService not initialized")
            .clone()
    }

    fn account_service(&self) -> Arc<dyn AccountService + Send + Sync> {
        self.account_service
            .as_ref()
            .expect("AccountService not initialized")
            .clone()
    }

    async fn remote_init(&mut self) -> Result<(), ExchangeError> {
        println!(
            "No remote initialization implemented for {:?}",
            self.spec.exchange_name
        );
        Ok(())
    }
}

// ------------------------
// Display 实现
// ------------------------
impl fmt::Display for BaseExchange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.spec.exchange_name.as_deref().unwrap_or("BaseExchange");
        write!(f, "{}#{:p}", name, self)
    }
}
