use crate::client::{RateLimiter, RetryConfig};
use crate::rescu::HttpError;
use reqwest::{Client, Proxy};
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct ResilientHttpClient {
    pub http: Client,
    pub retry: RetryConfig,
    pub rate_limiter: Arc<Mutex<RateLimiter>>,
}

impl ResilientHttpClient {
    pub fn new(
        retry: RetryConfig,
        limiter: RateLimiter,
        proxy: Option<String>,
    ) -> Result<Self, HttpError> {
        let mut builder = Client::builder().timeout(Duration::from_secs(30));

        if let Some(px) = proxy {
            let proxy_obj = Proxy::all(&px).map_err(|_| HttpError::InvalidProxy(px.clone()))?;
            builder = builder.proxy(proxy_obj);
        }

        let client = builder.build()?; //转换为 HttpError::Reqwest

        Ok(Self {
            http: client,
            retry,
            rate_limiter: Arc::new(Mutex::new(limiter)),
        })
    }

    pub async fn acquire_limiter(&self) -> Result<(), HttpError> {
        let mut guard = match self.rate_limiter.lock() {
            Ok(g) => g,
            Err(_) => {
                return Err(HttpError::Io(io::Error::new(
                    io::ErrorKind::Other,
                    "rate limiter mutex poisoned",
                )));
            }
        };

        guard.acquire().await;
        Ok(())
    }
}
