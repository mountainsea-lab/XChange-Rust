use crate::binance_exchange::{BinanceExchange, EXCHANGE_TYPE_KEY};
use crate::client::BinanceClient;
use crate::client::binance_spot::BinanceAuthed;
use crate::dto::BinanceError;
use crate::dto::meta::binance_system::BinanceSystemStatus;
use crate::service::{BinanceEd25519Digest, BinanceHmacDigest};
use std::sync::Arc;
use xchange_core::ValueFactory;
use xchange_core::client::{ResilientCall, boxed};
use xchange_core::exchange_specification::ExchangeParam;
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
            .and_then(|param| param.as_exchange_type());

        let base_url = spec_read
            .ssl_uri
            .clone()
            .or(spec_read.plain_text_uri.clone())
            .expect("no base URL provided");

        let use_ed25519 = matches!(
            spec_read.exchange_specific_parameters.get("ed25519"),
            Some(ExchangeParam::Boolean(true))
        );

        drop(spec_read); // 提前释放锁

        let client = Arc::new(BinanceClient::new_with_exchange_type(
            &base_url,
            api_key.as_deref(),
            exchange_type,
        )?);

        // ---------------------
        // 4) 构建 digest（ED25519 / HMAC）
        // ---------------------
        let digest: Option<Arc<dyn ParamsDigest + Send + Sync>> =
            match (use_ed25519, secret_key.as_ref()) {
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

    pub fn get_recv_window(&self) -> Result<Option<u64>, BinanceError> {
        let spec_read = self.exchange.base.spec.read();
        match spec_read.exchange_specific_parameters.get("recvWindow") {
            None => Ok(None),
            Some(ExchangeParam::Number(n)) => {
                if *n < 0 || *n > 60000 {
                    Err(BinanceError::InvalidParam(
                        "recvWindow must be in range [0, 60000]".into(),
                    ))
                } else {
                    Ok(Some(*n as u64))
                }
            }
            Some(ExchangeParam::String(s)) => {
                let n: u64 = s.parse().map_err(|e| {
                    BinanceError::InvalidParam(format!("recvWindow parse error: {}", e))
                })?;
                if n > 60000 {
                    Err(BinanceError::InvalidParam(
                        "recvWindow must be in range [0, 60000]".into(),
                    ))
                } else {
                    Ok(Some(n))
                }
            }
            _ => Err(BinanceError::InvalidParam(
                "recvWindow must be a number or string".into(),
            )),
        }
    }

    pub fn timestamp_factory(&self) -> Arc<dyn ValueFactory<u64> + Send + Sync> {
        self.exchange.timestamp_provider.clone()
    }

    pub async fn system_status(&self) -> Result<BinanceSystemStatus, BinanceError> {
        let spot_client = self.client.spot.clone();

        let mut resilient = ResilientCall::new(move || {
            let auth_client = spot_client.clone();
            async move { auth_client.system_status().await.map_err(boxed) }
        });

        resilient.call().await.map_err(|e| BinanceError::from(e))
    }
}
