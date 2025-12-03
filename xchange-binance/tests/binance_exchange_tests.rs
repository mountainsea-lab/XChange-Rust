use std::sync::Arc;
use tokio;
use xchange_binance::binance_exchange::BinanceExchange;
use xchange_binance::dto::BinanceError;
use xchange_binance::service::account_service::BinanceAccountService;
use xchange_binance::service::market_data_service::BinanceMarketDataService;

async fn create_exchange() -> Result<Arc<BinanceExchange>, BinanceError> {
    BinanceExchange::default().await
}

#[tokio::test]
async fn test_default_initialization() {
    let exchange = create_exchange().await.expect("default() should succeed");

    // Arc<BinanceExchange>
    assert!(Arc::strong_count(&exchange) >= 1);

    // Base exchange exists
    assert!(exchange.base.spec.read().ssl_uri.is_some());

    // 服务必须被初始化
    assert!(
        exchange
            .base
            .market_service
            .read()
            .as_ref()
            .unwrap()
            .is::<BinanceMarketDataService>()
    );

    assert!(
        exchange
            .base
            .account_service
            .read()
            .as_ref()
            .unwrap()
            .is::<BinanceAccountService>()
    );
}
//
// #[tokio::test]
// async fn test_with_specification_and_exchange_type() {
//     let mut spec = ExchangeSpecification::builder().build();
//
//     spec.exchange_specific_parameters.insert(
//         "EXCHANGE_TYPE".into(),
//         ExchangeParam::ExchangeType(ExchangeType::Futures),
//     );
//
//     let exchange = BinanceExchange::with_specification(spec)
//         .await
//         .expect("should init");
//
//     let ssl_uri = exchange.base.spec.read().ssl_uri.clone().unwrap();
//     assert!(
//         ssl_uri.contains("fapi"),
//         "Futures 类型应自动切换到 FUTURES URL"
//     );
// }
//
// #[tokio::test]
// async fn test_init_services() {
//     let exchange = BinanceExchange::default()
//         .await
//         .expect("should init");
//
//     let market = exchange.market_data_service()
//         .expect("market service exists");
//
//     let account = exchange.account_service()
//         .expect("account service exists");
//
//     assert!(market.as_ref().is::<BinanceMarketDataService>());
//     assert!(account.as_ref().is::<BinanceAccountService>());
// }
//
// #[tokio::test]
// async fn test_apply_specification_reinitializes_services() {
//     let exchange = BinanceExchange::default()
//         .await
//         .expect("init ok");
//
//     let old_market = exchange.market_data_service().unwrap();
//     let old_account = exchange.account_service().unwrap();
//
//     // 修改 spec → Futures
//     let mut new_spec = ExchangeSpecification::builder().build();
//     new_spec.exchange_specific_parameters.insert(
//         "EXCHANGE_TYPE".into(),
//         ExchangeParam::ExchangeType(ExchangeType::Futures),
//     );
//
//     let mut exchange_mut = Arc::clone(&exchange);
//
//     // apply new spec
//     Arc::get_mut(&mut exchange_mut)
//         .unwrap()
//         .apply_specification(new_spec)
//         .expect("apply_spec ok");
//
//     let new_market = exchange_mut.market_data_service().unwrap();
//     let new_account = exchange_mut.account_service().unwrap();
//
//     assert!(
//         !Arc::ptr_eq(&old_market, &new_market),
//         "MarketDataService 应在 apply_specification 后被重新创建"
//     );
//
//     assert!(
//         !Arc::ptr_eq(&old_account, &new_account),
//         "AccountService 应在 apply_specification 后被重新创建"
//     );
// }
//
// #[test]
// fn test_conclude_host_params_spot() {
//     let mut spec = ExchangeSpecification::builder().build();
//     spec.exchange_specific_parameters.insert(
//         "EXCHANGE_TYPE".into(),
//         ExchangeParam::ExchangeType(ExchangeType::Spot),
//     );
//
//     BinanceExchange::conclude_host_params(&mut spec);
//
//     assert!(
//         spec.ssl_uri.unwrap().contains("api.binance.com")
//             || spec.ssl_uri.unwrap().contains("api1.binance.com"),
//         "Spot → 应设置为 SPOT URL"
//     );
// }
//
// #[tokio::test]
// async fn test_timestamp_provider_exists() {
//     let exchange = BinanceExchange::default()
//         .await
//         .unwrap();
//
//     assert!(Arc::strong_count(&exchange.timestamp_provider) >= 1);
// }
//
// #[tokio::test]
// async fn test_exchange_health_proxy() {
//     let exchange = BinanceExchange::default()
//         .await
//         .unwrap();
//
//     let market = exchange.market_data_service().unwrap();
//     let health = market.exchange_health().await;
//
//     // 默认实现 → Offline
//     assert!(matches!(health, xchange_core::dto::ExchangeHealth::Offline));
// }
