use retrofit_rs::RetrofitError;
use xchange_binance::client::BinanceClient;
use xchange_binance::client::binance_spot::BinanceAuthed;
use xchange_core::exchange::ExchangeType;

#[tokio::test]
async fn test_binance_ping() -> Result<(), RetrofitError> {
    let base_url = "https://api.binance.com";

    let client = BinanceClient::new_with_exchange_type(base_url, None, Some(ExchangeType::Spot))?;

    // 调用真实 Binance ping
    let resp = client.spot.ping().await?;

    // Binance 的 ping 返回 "{}"
    assert!(resp.is_object());
    assert_eq!(resp, serde_json::json!({}));

    Ok(())
}
#[tokio::test]
async fn test_binance_system_status() -> Result<(), RetrofitError> {
    let base_url = "https://api.binance.com";
    let client = BinanceClient::new_with_exchange_type(base_url, None, None)?;
    let status = client.spot.system_status().await?;

    println!("system status = {:?}", status);

    // 基本断言
    // 通常返回 status: 0 表示正常
    assert!(status.status == 0 || status.status == 1);

    Ok(())
}

#[tokio::test]
async fn test_binance_time() -> Result<(), RetrofitError> {
    let base_url = "https://api.binance.com";
    let client = BinanceClient::new_with_exchange_type(base_url, None, Some(ExchangeType::Spot))?;

    let binance_time = client.spot.time().await?;

    println!("server time = {:?}", binance_time);

    // serverTime 必是一个大于 0 的毫秒时间戳
    assert!(binance_time.server_time > 0);

    Ok(())
}

#[tokio::test]
async fn test_binance_exchange_info() -> Result<(), RetrofitError> {
    let base_url = "https://api.binance.com";
    let client = BinanceClient::new_with_exchange_type(base_url, None, Some(ExchangeType::Spot))?;

    let info = client.spot.exchange_info().await?;

    println!("exchange info symbols = {}", info.symbols.len());

    // 必须包含一些交易对
    assert!(!info.symbols.is_empty());

    Ok(())
}

// #[tokio::test]
// async fn test_binance_authenticated_client_creation() -> Result<(), RetrofitError> {
//     let base_url = "https://api.binance.com";
//     let api_key = "dummy-key";
//
//     let client = BinanceClient::new_authenticated(base_url, api_key)?;
//
//     assert!(client.auth.is_some());
//
//     assert_eq!(client.public.retrofit().base_url(), base_url);
//
//     assert_eq!(
//         client.auth.as_ref().unwrap().retrofit().base_url(),
//         base_url
//     );
//
//     Ok(())
// }
