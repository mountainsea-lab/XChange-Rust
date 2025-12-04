// ----------------- 辅助函数 -----------------

use std::sync::Arc;
use xchange_binance::binance_exchange::{BinanceExchange, EXCHANGE_TYPE_KEY};
use xchange_binance::dto::BinanceError;
use xchange_binance::dto::marketdata::KlineInterval;
use xchange_binance::dto::meta::binance_system::{BinanceSystemStatus, BinanceTime};
use xchange_binance::dto::meta::exchange_info::BinanceExchangeInfo;
use xchange_binance::service::market_data_service::BinanceMarketDataService;
use xchange_core::currency::currency_pair::CurrencyPair;
use xchange_core::dto::meta::ExchangeHealth;
use xchange_core::exchange::{Exchange, ExchangeType};
use xchange_core::exchange_specification::ExchangeParam;
use xchange_core::service::marketdata::market_data_service::MarketDataService;
use xchange_core::utils::service_arc;

/// 异步创建 BinanceExchange
async fn default_exchange() -> Result<Arc<BinanceExchange>, BinanceError> {
    BinanceExchange::default().await
}

async fn new_exchange_futures() -> Result<Arc<BinanceExchange>, BinanceError> {
    let mut exchange = BinanceExchange::new().await?;
    let mut spec = BinanceExchange::default_exchange_specification();
    spec.api_key = None;
    spec.secret_key = None;
    spec.use_sandbox = true;
    spec.exchange_specific_parameters.insert(
        EXCHANGE_TYPE_KEY.into(),
        ExchangeParam::ExchangeType(ExchangeType::Futures),
    );
    spec.exchange_specific_parameters.insert(
        "Portfolio_Margin_Enabled".into(),
        ExchangeParam::Boolean(true),
    );
    let _ = exchange.apply_specification(spec);

    Ok(exchange)
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

#[tokio::test]
async fn test_futures_exchange_info() {
    let exchange = new_exchange_futures()
        .await
        .expect("default() should succeed");
    let service: Arc<dyn MarketDataService + Send + Sync> = exchange.market_data_service().unwrap();

    let market: Arc<BinanceMarketDataService> = service_arc(&service);

    let result = market.future_exchange_info().await;
    assert!(result.is_ok(), "exchange_info() should succeed");

    let info: BinanceExchangeInfo = result.unwrap();
    println!("BinanceExchangeInfo = {:?}", info);

    assert!(
        !info.symbols.is_empty(),
        "exchange_info symbols should not be empty"
    );
}

#[tokio::test]
async fn test_klines_default_limit() {
    // 1. 初始化 exchange
    let exchange = default_exchange()
        .await
        .expect("default_exchange() should succeed");

    // 2. 获取服务
    let service: Arc<dyn MarketDataService + Send + Sync> = exchange.market_data_service().unwrap();

    let market: Arc<BinanceMarketDataService> = service_arc(&service);

    let cp = CurrencyPair::from_symbols("BTC", "USDT");

    // 3. API 调用
    let result = market.klines_default_limit(cp, KlineInterval::M1).await;

    // 4. 成功性检查
    assert!(
        result.is_ok(),
        "klines_default_limit() should succeed, err = {:?}",
        result.err()
    );

    let klines = result.unwrap();

    // 5. 不应为空
    assert!(
        !klines.is_empty(),
        "klines_default_limit() should return > 0 klines"
    );

    // --- 以下为根据你提供的 BinanceKline 完整字段增强校验 ---

    use rust_decimal::Decimal;

    for k in &klines {
        // 基础字段
        assert!(k.open_time > 0, "open_time must be > 0");
        assert!(
            k.close_time >= k.open_time,
            "close_time should >= open_time"
        );

        // 价格字段必须为 Decimal >= 0
        assert!(k.open >= Decimal::ZERO, "open must >= 0");
        assert!(k.high >= Decimal::ZERO, "high must >= 0");
        assert!(k.low >= Decimal::ZERO, "low must >= 0");
        assert!(k.close >= Decimal::ZERO, "close must >= 0");

        // high >= low
        assert!(k.high >= k.low, "high >= low must hold");

        // volume、quote_asset_volume >= 0
        assert!(k.volume >= Decimal::ZERO, "volume must be >= 0");
        assert!(
            k.quote_asset_volume >= Decimal::ZERO,
            "quote_asset_volume must be >= 0"
        );

        // taker buy volumes
        assert!(
            k.taker_buy_base_asset_volume >= Decimal::ZERO,
            "taker_buy_base_asset_volume must >= 0"
        );
        assert!(
            k.taker_buy_quote_asset_volume >= Decimal::ZERO,
            "taker_buy_quote_asset_volume must >= 0"
        );

        // trades 数量
        assert!(
            k.number_of_trades >= 0,
            "number_of_trades must be non-negative"
        );

        // instrument 与 interval 校验
        assert_eq!(
            k.interval,
            KlineInterval::M1,
            "interval must match request interval"
        );
    }

    // 6. 校验 interval 的连续性（M1 ≈ 60 秒）
    for i in 1..klines.len() {
        let prev = klines[i - 1].open_time;
        let curr = klines[i].open_time;
        let diff = curr - prev;

        // Binance 通常为 60000 ms
        assert!(
            (diff - 60_000).abs() <= 2000,
            "1m kline interval should be close to 60s (60000 ms), diff = {}",
            diff
        );
    }

    println!("klines_default_limit returned {} klines", klines.len());
    println!("First = {:?}", klines.first());
    println!("Last  = {:?}", klines.last());
}

#[tokio::test]
async fn test_klines_default_futures_limit() {
    // 1. 初始化 exchange
    let exchange = new_exchange_futures()
        .await
        .expect("default_exchange() should succeed");

    // 2. 获取服务
    let service: Arc<dyn MarketDataService + Send + Sync> = exchange.market_data_service().unwrap();

    let market: Arc<BinanceMarketDataService> = service_arc(&service);

    let cp = CurrencyPair::from_symbols("BTC", "USDT");

    // 3. API 调用
    let result = market
        .future_klines_default_limit(cp, KlineInterval::M1)
        .await;

    // 4. 成功性检查
    assert!(
        result.is_ok(),
        "future_klines_default_limit() should succeed, err = {:?}",
        result.err()
    );

    let klines = result.unwrap();

    // 5. 不应为空
    assert!(
        !klines.is_empty(),
        "klines_default_limit() should return > 0 klines"
    );

    // --- 以下为根据你提供的 BinanceKline 完整字段增强校验 ---

    use rust_decimal::Decimal;

    for k in &klines {
        // 基础字段
        assert!(k.open_time > 0, "open_time must be > 0");
        assert!(
            k.close_time >= k.open_time,
            "close_time should >= open_time"
        );

        // 价格字段必须为 Decimal >= 0
        assert!(k.open >= Decimal::ZERO, "open must >= 0");
        assert!(k.high >= Decimal::ZERO, "high must >= 0");
        assert!(k.low >= Decimal::ZERO, "low must >= 0");
        assert!(k.close >= Decimal::ZERO, "close must >= 0");

        // high >= low
        assert!(k.high >= k.low, "high >= low must hold");

        // volume、quote_asset_volume >= 0
        assert!(k.volume >= Decimal::ZERO, "volume must be >= 0");
        assert!(
            k.quote_asset_volume >= Decimal::ZERO,
            "quote_asset_volume must be >= 0"
        );

        // taker buy volumes
        assert!(
            k.taker_buy_base_asset_volume >= Decimal::ZERO,
            "taker_buy_base_asset_volume must >= 0"
        );
        assert!(
            k.taker_buy_quote_asset_volume >= Decimal::ZERO,
            "taker_buy_quote_asset_volume must >= 0"
        );

        // trades 数量
        assert!(
            k.number_of_trades >= 0,
            "number_of_trades must be non-negative"
        );

        // instrument 与 interval 校验
        assert_eq!(
            k.interval,
            KlineInterval::M1,
            "interval must match request interval"
        );
    }

    // 6. 校验 interval 的连续性（M1 ≈ 60 秒）
    for i in 1..klines.len() {
        let prev = klines[i - 1].open_time;
        let curr = klines[i].open_time;
        let diff = curr - prev;

        // Binance 通常为 60000 ms
        assert!(
            (diff - 60_000).abs() <= 2000,
            "1m kline interval should be close to 60s (60000 ms), diff = {}",
            diff
        );
    }

    println!("klines_default_limit returned {} klines", klines.len());
    println!("First = {:?}", klines.first());
    println!("Last  = {:?}", klines.last());
}
