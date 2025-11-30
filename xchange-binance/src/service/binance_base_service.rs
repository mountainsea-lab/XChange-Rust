use crate::binance_exchange::{
    BinanceExchange, EXCHANGE_TYPE_KEY, FUTURES_URL, INVERSE_FUTURES_URL,
};
use crate::client::BinanceClient;
use crate::dto::BinanceError;
use crate::service::{BinanceEd25519Digest, BinanceHmacDigest};
use std::sync::Arc;
use xchange_core::exchange::ExchangeType;
use xchange_core::rescu::params_digest::ParamsDigest;

pub struct BinanceBaseService {
    pub api_key: Option<String>,

    pub client: Arc<BinanceClient>,

    pub digest: Option<Arc<dyn ParamsDigest + Send + Sync>>,

    pub exchange: Arc<BinanceExchange>,
}

impl BinanceBaseService {
    pub fn new(exchange: Arc<BinanceExchange>) -> Result<Self, BinanceError> {
        // ---------------------
        // 1) 读取 spec（只读锁即可）
        // ---------------------
        let spec_read = exchange.base.spec.read();
        let api_key = spec_read.api_key.clone();
        let secret_key = spec_read.secret_key.clone();
        let exchange_type = spec_read
            .exchange_specific_parameters
            .get(EXCHANGE_TYPE_KEY)
            .cloned();
        let base_url = spec_read
            .ssl_uri
            .clone()
            .or(spec_read.plain_text_uri.clone())
            .expect("no base URL provided");
        let use_sandbox = spec_read.use_sandbox; // 这里提前 clone

        drop(spec_read); // 提前释放锁

        // ---------------------
        // 2) 构建 Client（public or authed）
        // ---------------------
        let mut client = if let Some(ref key) = api_key {
            BinanceClient::new_authenticated(&base_url, key)?
        } else {
            BinanceClient::new_public(&base_url)?
        };

        // ---------------------
        // 3) 根据 exchange_type attach futures / inverse futures
        // ---------------------
        if let Some(ref key) = api_key {
            match exchange_type {
                Some(ExchangeType::Futures) | Some(ExchangeType::PortfolioMargin) => {
                    attach_futures(&mut client, key, FUTURES_URL)?;
                }
                Some(ExchangeType::Inverse) => {
                    attach_futures(&mut client, key, INVERSE_FUTURES_URL)?;
                }
                _ => {}
            }
        }

        let client = Arc::new(client);

        // ---------------------
        // 4) 构建 digest（ED25519 / HMAC）
        // ---------------------
        let digest: Option<Arc<dyn ParamsDigest + Send + Sync>> =
            match (use_sandbox, secret_key.as_ref()) {
                (true, Some(sk)) => Some(Arc::new(BinanceEd25519Digest::new(sk)?) as _),
                (_, Some(sk)) => Some(Arc::new(BinanceHmacDigest::new(sk)?) as _),
                _ => None,
            };

        Ok(Self {
            exchange,
            api_key,
            client,
            digest,
        })
    }
}

/// 提取的 futures attach 函数
fn attach_futures(
    client: &mut BinanceClient,
    api_key: &str,
    url: &str,
) -> Result<(), BinanceError> {
    let futures_cli = BinanceClient::new_authenticated(url, api_key)?;
    client.futures_authed = futures_cli.futures_authed;
    Ok(())
}
