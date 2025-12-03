use std::sync::Arc;
use tokio;
use xchange_binance::binance_exchange::BinanceExchange;
use xchange_binance::dto::BinanceError;
use xchange_binance::service::account_service::BinanceAccountService;
use xchange_binance::service::market_data_service::BinanceMarketDataService;
use xchange_core::exchange_specification::ExchangeParam;

async fn default_exchange() -> Result<Arc<BinanceExchange>, BinanceError> {
    BinanceExchange::default().await
}

async fn new_exchange() -> Result<Arc<BinanceExchange>, BinanceError> {
    let mut exchange = BinanceExchange::new().await?;
    let mut spec = BinanceExchange::default_exchange_specification();
    spec.api_key = None;
    spec.secret_key = None;
    spec.use_sandbox = true;
    spec.exchange_specific_parameters.insert(
        "Portfolio_Margin_Enabled".into(),
        ExchangeParam::Boolean(true),
    );
    let _ = exchange.apply_specification(spec);
    Ok(Arc::new(exchange))
}

#[tokio::test]
async fn test_default_initialization() {
    let exchange = default_exchange().await.expect("default() should succeed");

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

#[tokio::test]
async fn test_new_initialization() {
    let exchange = new_exchange().await.expect("default() should succeed");

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
