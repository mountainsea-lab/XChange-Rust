// ----------------- 辅助函数 -----------------

use std::sync::Arc;
use xchange_binance::binance_exchange::BinanceExchange;
use xchange_binance::dto::BinanceError;
use xchange_binance::dto::meta::binance_system::{BinanceSystemStatus, BinanceTime};
use xchange_binance::dto::meta::exchange_info::BinanceExchangeInfo;
use xchange_binance::service::market_data_service::BinanceMarketDataService;
use xchange_core::dto::meta::ExchangeHealth;
use xchange_core::exchange::Exchange;
use xchange_core::service::marketdata::market_data_service::MarketDataService;
use xchange_core::utils::service_arc;

/// 异步创建 BinanceExchange
async fn default_exchange() -> Result<Arc<BinanceExchange>, BinanceError> {
    BinanceExchange::default().await
}

// ----------------- 测试 -----------------
#[tokio::test]
async fn test_market_data_service_arc() {
    let exchange = default_exchange().await.expect("default() should succeed");

    let market_service: Arc<dyn MarketDataService + Send + Sync> =
        exchange.market_data_service().unwrap();

    // 获取 Arc<T>
    let market_service_arc: Arc<BinanceMarketDataService> = service_arc(&market_service);

    // ping 方法
    let ping_result = market_service_arc.ping().await;
    assert!(ping_result.is_ok(), "Ping should succeed");

    // exchange_health 方法
    let health = market_service_arc.exchange_health().await;
    assert!(matches!(
        health,
        ExchangeHealth::Online | ExchangeHealth::Offline
    ));
}

#[tokio::test]
async fn test_binance_time() {
    let exchange = default_exchange().await.expect("default() should succeed");
    let service: Arc<dyn MarketDataService + Send + Sync> = exchange.market_data_service().unwrap();

    let market: Arc<BinanceMarketDataService> = service_arc(&service);

    let result = market.binance_time().await;
    assert!(result.is_ok(), "binance_time() should succeed");

    let time: BinanceTime = result.unwrap();
    println!("BinanceTime = {:?}", time);
}

#[tokio::test]
async fn test_system_status() {
    let exchange = default_exchange().await.expect("default() should succeed");
    let service: Arc<dyn MarketDataService + Send + Sync> = exchange.market_data_service().unwrap();

    let market: Arc<BinanceMarketDataService> = service_arc(&service);

    let result = market.system_status().await;
    assert!(result.is_ok(), "system_status() should succeed");

    let status: BinanceSystemStatus = result.unwrap();
    println!("BinanceSystemStatus = {:?}", status);

    // 一般会是 normal 或则 maintenance，测试只验证结构有效
    assert!(
        status.status == 0 || status.status == 1,
        "system_status should return a valid status"
    );
}

#[tokio::test]
async fn test_exchange_info() {
    let exchange = default_exchange().await.expect("default() should succeed");
    let service: Arc<dyn MarketDataService + Send + Sync> = exchange.market_data_service().unwrap();

    let market: Arc<BinanceMarketDataService> = service_arc(&service);

    let result = market.exchange_info().await;
    assert!(result.is_ok(), "exchange_info() should succeed");

    let info: BinanceExchangeInfo = result.unwrap();
    println!("BinanceExchangeInfo = {:?}", info);

    assert!(
        !info.symbols.is_empty(),
        "exchange_info symbols should not be empty"
    );
}
