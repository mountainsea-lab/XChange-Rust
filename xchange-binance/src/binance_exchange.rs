// use std::sync::Arc;
// use xchange_core::exchange::BaseExchange;
// use xchange_core::exchange_specification::ExchangeSpecification;
//
// pub struct BinanceExchange {
//     pub base: BaseExchange,
//     pub timestamp_factory: Arc<dyn SynchronizedValueFactory<u64> + Send + Sync>,
// }
//
// impl BinanceExchange {
//     pub const SPOT_URL: &'static str = "https://api.binance.com";
//     pub const FUTURES_URL: &'static str = "https://fapi.binance.com";
//     pub const INVERSE_FUTURES_URL: &'static str = "https://dapi.binance.com";
//     pub const PORTFOLIO_MARGIN_URL: &'static str = "https://papi.binance.com";
//
//     pub const SANDBOX_SPOT_URL: &'static str = "https://testnet.binance.vision";
//     pub const SANDBOX_FUTURES_URL: &'static str = "https://testnet.binancefuture.com";
//     pub const SANDBOX_INVERSE_FUTURES_URL: &'static str = "https://testnet.binancefuture.com";
//
//     pub fn new() -> Self {
//         Self {
//             base: BaseExchange {
//                 exchange_spec: Arc::new(ExchangeSpecification::new()),
//                 exchange_meta_data: Arc::new(ExchangeMetaData),
//                 market_data_service: Arc::new(DummyMarketDataService {}),
//                 trade_service: Arc::new(DummyTradeService {}),
//                 account_service: Arc::new(DummyAccountService {}),
//             },
//             timestamp_factory: Arc::new(DummyTimestampFactory {}),
//         }
//     }
//
//     fn conclude_host_params(spec: &mut ExchangeSpecification) {
//         if let Some(Value::Spot) = spec.exchange_specific_parameters.get("Exchange_Type") {
//             if let Some(Value::Bool(true)) = spec.exchange_specific_parameters.get("Use_Sandbox") {
//                 spec.ssl_uri = Some(Self::SANDBOX_SPOT_URL.to_string());
//             } else {
//                 spec.ssl_uri = Some(Self::SPOT_URL.to_string());
//             }
//         }
//     }
//
//     pub fn using_sandbox(&self) -> bool {
//         if let Some(Value::Bool(b)) = self
//             .base
//             .exchange_spec
//             .exchange_specific_parameters
//             .get("Use_Sandbox")
//         {
//             *b
//         } else {
//             false
//         }
//     }
//
//     pub fn is_authenticated(&self) -> bool {
//         self.base.exchange_spec.api_key.is_some() && self.base.exchange_spec.secret_key.is_some()
//     }
// }
//
// // 用 OnceCell 实现静态 RESILIENCE_REGISTRIES
// static RESILIENCE_REGISTRIES: OnceCell<Arc<ResilienceRegistries>> = OnceCell::new();
//
// #[async_trait]
// impl Exchange for BinanceExchange {
//     fn default_exchange_specification(&self) -> ExchangeSpecification {
//         let mut spec = ExchangeSpecification::new();
//         spec.ssl_uri = Some(Self::SPOT_URL.to_string());
//         spec.host = Some("www.binance.com".to_string());
//         spec.port = Some(80);
//         spec.exchange_name = Some("Binance".to_string());
//         spec.exchange_description = Some("Binance Exchange.".to_string());
//         spec.exchange_specific_parameters
//             .insert("Exchange_Type".to_string(), Value::Spot);
//         spec.exchange_specific_parameters
//             .insert("Use_Sandbox".to_string(), Value::Bool(false));
//         spec
//     }
//
//     fn apply_specification(&mut self, mut spec: ExchangeSpecification) {
//         Self::conclude_host_params(&mut spec);
//         self.base.apply_specification(spec);
//     }
//
//     fn init_services(&mut self) {
//         let registries = self.get_resilience_registries();
//         self.timestamp_factory = Arc::new(DummyTimestampFactory {});
//         self.base.market_data_service = Arc::new(DummyMarketDataService {});
//         self.base.trade_service = Arc::new(DummyTradeService {});
//         self.base.account_service = Arc::new(DummyAccountService {});
//     }
//
//     fn get_resilience_registries(&self) -> Arc<ResilienceRegistries> {
//         RESILIENCE_REGISTRIES
//             .get_or_init(|| Arc::new(ResilienceRegistries {}))
//             .clone()
//     }
//
//     async fn remote_init(&mut self) -> Result<(), ExchangeError> {
//         // 占位实现，可以调用 Binance REST API 获取 ExchangeInfo 并更新 exchange_meta_data
//         println!("Remote init placeholder for BinanceExchange");
//         Ok(())
//     }
// }
