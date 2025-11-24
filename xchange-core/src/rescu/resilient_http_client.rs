use crate::client::{RateLimiter, RetryConfig};
use crate::rescu::HttpError;
use reqwest::{Client, Method, Proxy, Response};
use serde::de::DeserializeOwned;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

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

    // pub async fn execute<F, Fut, T>(&self, mut f: F) -> Result<T, HttpError>
    // where
    //     F: FnOnce(&Client) -> Fut,
    //     Fut: Future<Output = Result<T, HttpError>>,
    // {
    //     for attempt in 0..self.retry.max_attempts {
    //         // 限流
    //         self.acquire_limiter().await?;
    //
    //         // 调用用户传入函数
    //         match f(&self.http).await {
    //             Ok(resp) => return Ok(resp),
    //             Err(err) => {
    //                 if attempt + 1 == self.retry.max_attempts {
    //                     return Err(err);
    //                 }
    //                 let delay = self.retry.delay_for_attempt(attempt);
    //                 sleep(delay).await;
    //             }
    //         }
    //     }
    //
    //     Err(HttpError::Io(std::io::Error::new(
    //         io::ErrorKind::Other,
    //         "max retry attempts reached",
    //     )))
    // }
    pub async fn execute<F, Fut, T>(self: Arc<Self>, f: F) -> Result<T, HttpError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, HttpError>> + Send,
        T: Send + 'static,
    {
        for attempt in 0..self.retry.max_attempts {
            self.acquire_limiter().await?;
            let fut = f(); // 每次循环调用闭包
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

        Err(HttpError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "max retry attempts reached",
        )))
    }
}

#[derive(Clone)]
pub struct RequestBuilder {
    client: Arc<ResilientHttpClient>,
    method: Method,
    path: String,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
}

impl RequestBuilder {
    pub fn new(client: Arc<ResilientHttpClient>, method: Method, path: impl Into<String>) -> Self {
        Self {
            client,
            method,
            path: path.into(),
            query: vec![],
            headers: vec![],
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

    pub async fn send<T: DeserializeOwned + Send + 'static>(
        self,
        base_url: String,
    ) -> Result<T, HttpError> {
        let client_arc = self.client.clone();
        let method = self.method.clone();
        let query = self.query.clone();
        let headers = self.headers.clone();
        let path = self.path.clone();

        // 完整 URL 克隆成 String
        let url = format!("{}{}", base_url, path);

        client_arc
            .clone()
            .execute(move || {
                let client = client_arc.clone();
                let url = url.clone(); // async move 需要所有权
                let method = method.clone();
                let query = query.clone();
                let headers = headers.clone();

                async move {
                    let mut req = client.http.request(method, &url);
                    for (k, v) in query.iter() {
                        req = req.query(&[(k.as_str(), v.as_str())]);
                    }
                    for (k, v) in headers.iter() {
                        req = req.header(k, v);
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

// ======================= 使用示例 =======================
// let client = Arc::new(ResilientHttpClient::new(
//     RetryConfig { max_attempts: 3, initial_delay: Duration::from_millis(200), multiplier: 2 },
//     RateLimiter::new(5, Duration::from_secs(1)),
//     None
// )?);

// let resp: MyResponse = RequestBuilder::new(client.clone(), Method::GET, "/api/v3/ping")
//     .header("X-MBX-APIKEY", api_key)
//     .send("https://api.binance.com")
//     .await?;
