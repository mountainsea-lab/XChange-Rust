use crate::client::{RateLimiter, RetryConfig};
use crate::rescu::HttpError;
use crate::rescu::params_digest::ParamsDigest;
use hmac::Mac;
use reqwest::{Client, Method, Proxy};
use serde::de::DeserializeOwned;
use std::future::Future;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

        let client = builder.build()?;

        Ok(Self {
            http: client,
            retry,
            rate_limiter: Arc::new(Mutex::new(limiter)),
        })
    }

    pub async fn acquire_limiter(&self) -> Result<(), HttpError> {
        let mut guard = self.rate_limiter.lock().map_err(|_| {
            HttpError::Io(io::Error::new(
                io::ErrorKind::Other,
                "rate limiter poisoned",
            ))
        })?;

        guard.acquire().await;
        Ok(())
    }

    pub async fn execute<F, Fut, T>(self: Arc<Self>, f: F) -> Result<T, HttpError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, HttpError>> + Send,
        T: Send + 'static,
    {
        for attempt in 0..self.retry.max_attempts {
            self.acquire_limiter().await?;
            let fut = f();
            match fut.await {
                Ok(resp) => return Ok(resp),
                Err(err) => {
                    if attempt + 1 == self.retry.max_attempts {
                        return Err(err);
                    }
                    let delay = self.retry.delay_for_attempt(attempt);
                    tokio::time::sleep(delay).await;
                }
            }
        }

        Err(HttpError::Io(io::Error::new(
            io::ErrorKind::Other,
            "max retry attempts reached",
        )))
    }
}

#[derive(Clone)]
pub struct RequestBuilder {
    client: Arc<ResilientHttpClient>,
    pub method: Method,
    path: String,
    pub query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    pub body: Option<String>,
    signed: Option<(String, Arc<dyn ParamsDigest>)>, // (api_key, digest)
}

impl RequestBuilder {
    pub fn new(client: Arc<ResilientHttpClient>, method: Method, path: impl Into<String>) -> Self {
        Self {
            client,
            method,
            path: path.into(),
            query: vec![],
            headers: vec![],
            body: None,
            signed: None,
        }
    }

    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.push((key.into(), value.into()));
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    pub fn body(mut self, b: impl Into<String>) -> Self {
        self.body = Some(b.into());
        self
    }

    /// 链式签名方法
    pub fn signed(mut self, api_key: impl Into<String>, digest: Arc<dyn ParamsDigest>) -> Self {
        self.signed = Some((api_key.into(), digest));
        self
    }

    fn build_query_with_signature(&self) -> Result<Vec<(String, String)>, HttpError> {
        let mut q = self.query.clone();

        // timestamp
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(HttpError::InvalidTimestamp)?
            .as_millis()
            .to_string();
        q.push(("timestamp".to_string(), ts));

        if let Some((_api_key, digest)) = &self.signed {
            // 让 digest 获取所有参数（method + body + query）
            let signature = digest.digest_request(self)?;
            q.push(("signature".to_string(), signature));
        }

        Ok(q)
    }

    pub async fn send<T: DeserializeOwned + Send + 'static>(
        self,
        base_url: String,
    ) -> Result<T, HttpError> {
        let client = self.client.clone();
        let method = self.method.clone();
        let headers = self.headers.clone();
        let path = self.path.clone();
        let query = self.build_query_with_signature()?;
        let url = format!("{}{}", base_url, path);

        let api_key_header = self.signed.as_ref().map(|(key, _)| key.clone());

        client
            .clone()
            .execute(move || {
                let client = client.clone();
                let url = url.clone();
                let method = method.clone();
                let query = query.clone();
                let headers = headers.clone();
                let api_key_header = api_key_header.clone();

                async move {
                    let mut req = client.http.request(method, &url);

                    for (k, v) in query.iter() {
                        req = req.query(&[(k.as_str(), v.as_str())]);
                    }

                    for (k, v) in headers.iter() {
                        req = req.header(k, v);
                    }

                    if let Some(key) = api_key_header {
                        req = req.header("X-MBX-APIKEY", key);
                    }

                    let resp = req.send().await.map_err(HttpError::Reqwest)?;
                    if resp.status().is_success() {
                        resp.json::<T>().await.map_err(HttpError::Reqwest)
                    } else {
                        Err(HttpError::Io(io::Error::new(
                            io::ErrorKind::Other,
                            format!("HTTP status {}", resp.status()),
                        )))
                    }
                }
            })
            .await
    }
}
