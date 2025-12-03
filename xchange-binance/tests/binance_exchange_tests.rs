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
            .as_any()
            .is::<BinanceMarketDataService>()
    );

    assert!(
        exchange
            .base
            .account_service
            .read()
            .as_ref()
            .unwrap()
            .as_any()
            .is::<BinanceAccountService>()
    );
}
