// // ----------------- 辅助函数 -----------------
//
// use std::any::Any;
// use std::sync::Arc;
// use xchange_binance::binance_exchange::BinanceExchange;
// use xchange_binance::dto::BinanceError;
// use xchange_binance::service::account_service::BinanceAccountService;
// use xchange_binance::service::market_data_service::BinanceMarketDataService;
//
// /// 异步创建 BinanceExchange
// async fn default_exchange() -> Result<Arc<BinanceExchange>, BinanceError> {
//     BinanceExchange::default().await
// }
//
// /// 通用函数：从 Exchange 获取具体服务
// pub fn get_service<S: 'static>(
//     service: Arc<dyn Any + Send + Sync>,
// ) -> Result<Arc<S>, String> {
//     let concrete = service
//         .as_any()
//         .downcast_ref::<S>()
//         .ok_or_else(|| format!("Failed to downcast service to {}", std::any::type_name::<S>()))?;
//     Ok(Arc::new(concrete.clone()))
// }
//
// /// 针对 MarketDataService 的快捷函数（测试中可直接 unwrap）
// pub fn get_market_service(exchange: &Arc<BinanceExchange>) -> Arc<BinanceMarketDataService> {
//     let service = exchange
//         .market_data_service()
//         .expect("MarketDataService should be initialized");
//     get_service::<BinanceMarketDataService>(service).expect("Expected BinanceMarketDataService")
// }
//
// /// 针对 AccountService 的快捷函数
// pub fn get_account_service(exchange: &Arc<BinanceExchange>) -> Arc<BinanceAccountService> {
//     let service = exchange
//         .account_service()
//         .expect("AccountService should be initialized");
//     get_service::<BinanceAccountService>(service).expect("Expected BinanceAccountService")
// }
//
// // ----------------- 测试 -----------------
// #[tokio::test]
// async fn test_market_data_service_initialization() {
//     let exchange = default_exchange().await.expect("default() should succeed");
//     let market_service = get_market_service(&exchange).await;
//
//     // ping 方法
//     let ping_result = market_service.ping().await;
//     assert!(ping_result.is_ok(), "Ping should succeed");
//
//     // exchange_health 方法
//     let health = market_service.exchange_health().await;
//     assert!(matches!(health, ExchangeHealth::Online | ExchangeHealth::Offline));
// }
//
// #[tokio::test]
// async fn test_market_data_service_system_status() {
//     let exchange = create_exchange().await.expect("default() should succeed");
//     let market_service = get_market_service(&exchange).await;
//
//     let status = market_service.system_status().await;
//     match status {
//         Ok(status) => {
//             println!("System status: {}", status.status);
//             assert!(status.status >= 0);
//         }
//         Err(e) => panic!("system_status() failed: {:?}", e),
//     }
// }
