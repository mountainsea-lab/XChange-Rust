use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

pub trait OAuthSigner: Send + Sync {
    fn sign(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
}

#[derive(Clone)]
pub enum ProxyKind {
    Http,
    Socks5,
}

#[derive(Clone)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
    pub kind: ProxyKind,
}

#[derive(Clone)]
pub enum ConnectionType {
    Default,
}

#[derive(Clone)]
pub struct JsonConfig;

#[derive(Clone)]
pub struct ClientConfig {
    /// key -> default value
    pub default_params: HashMap<String, Value>,

    pub timeout: Duration,
    pub read_timeout: Duration,

    pub proxy: Option<ProxyConfig>,

    pub ignore_http_error_codes: bool,
    pub wrap_unexpected_exceptions: bool,

    pub json_config: JsonConfig,

    pub oauth: Option<Arc<dyn OAuthSigner>>,

    pub connection_type: ConnectionType,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            default_params: HashMap::new(),
            timeout: Duration::from_secs(10),
            read_timeout: Duration::from_secs(30),
            proxy: None,
            ignore_http_error_codes: false,
            wrap_unexpected_exceptions: true,
            json_config: JsonConfig,
            oauth: None,
            connection_type: ConnectionType::Default,
        }
    }
}
