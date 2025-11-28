// use crate::client::binance::BinanceAuthedClient;
// use crate::client::binance_futures::BinanceFuturesAuthedClient;
// use crate::client::BinanceClient;
// use crate::dto::meta::binance_system::BinanceSystemStatus;
// use crate::dto::{BinanceError, ExchangeType};
// use std::sync::Arc;
//
// pub struct BinanceBaseService {
//     pub exchange: Arc<BinanceExchange>,
//     pub client: BinanceClient,
//     pub api_key: String,
//     pub signature_creator: BinanceHmacDigest,
// }
//
// impl BinanceBaseService {
//     pub fn new(exchange: Arc<BinanceExchange>) -> Self {
//         let exchange_type = exchange.get_exchange_type().unwrap_or(ExchangeType::Spot);
//
//         let api_key = exchange.get_api_key().to_string();
//         let signature_creator = BinanceHmacDigest::new(exchange.get_secret_key().to_string());
//
//         let mut client = BinanceClient {
//             auth: Some(BinanceAuthedClient::new(exchange.clone())),
//             futures: None,
//             inverse_futures: None,
//         };
//
//         match exchange_type {
//             ExchangeType::Spot => {}
//             ExchangeType::Futures => {
//                 client.futures = Some(BinanceFuturesAuthenticatedClient::new(exchange.clone()));
//             }
//             ExchangeType::Inverse => {
//                 client.inverse_futures =
//                     Some(BinanceFuturesAuthenticatedClient::new(exchange.clone()));
//             }
//         }
//
//         Self {
//             exchange,
//             client,
//             api_key,
//             signature_creator,
//         }
//     }
//
//     pub fn get_recv_window(&self) -> Result<Option<u64>, BinanceError> {
//         match self.exchange.get_exchange_specific_parameter("recvWindow") {
//             None => Ok(None),
//             Some(param) => match param {
//                 ExchangeParam::Number(n) => {
//                     if *n < 0 || *n > 60000 {
//                         Err(BinanceError::InvalidParameter(
//                             "recvWindow out of range [0, 60000]".into(),
//                         ))
//                     } else {
//                         Ok(Some(*n as u64))
//                     }
//                 }
//                 ExchangeParam::String(s) => s
//                     .parse::<u64>()
//                     .map(Some)
//                     .map_err(|_| BinanceError::InvalidParameter("recvWindow parse failed".into())),
//             },
//         }
//     }
//
//     pub fn get_timestamp_factory(&self) -> SynchronizedValueFactory {
//         self.exchange.get_timestamp_factory()
//     }
//
//     pub fn get_system_status(&self) -> Result<BinanceSystemStatus, BinanceError> {
//         if let Some(auth) = &self.client.auth {
//             resilient_call(|| auth.system_status())
//         } else {
//             Err(BinanceError::ClientNotInitialized)
//         }
//     }
// }
