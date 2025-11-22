use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

// -----------------------------
// OAuth Trait
// -----------------------------
pub trait OAuthSigner: Send + Sync {
    fn sign(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
}

// -----------------------------
// Proxy Config
// -----------------------------
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

impl ProxyConfig {
    pub fn new(host: impl Into<String>, port: u16, kind: ProxyKind) -> Self {
        Self {
            host: host.into(),
            port,
            kind,
        }
    }
}

// -----------------------------
// ConnectionType
// -----------------------------
#[derive(Clone)]
pub enum ConnectionType {
    Default,
    // Future: RawTcp, WebSocket, Tls, UnixSocket, etc.
}

// -----------------------------
// JsonConfig
// -----------------------------
#[derive(Clone)]
pub struct JsonConfig;

// -----------------------------
// ClientConfig
// -----------------------------
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

// -----------------------------
// Builder
// -----------------------------
pub struct ClientConfigBuilder {
    inner: ClientConfig,
}

impl ClientConfig {
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder {
            inner: ClientConfig::default(),
        }
    }
}

impl ClientConfigBuilder {
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.inner.timeout = timeout;
        self
    }

    pub fn read_timeout(mut self, timeout: Duration) -> Self {
        self.inner.read_timeout = timeout;
        self
    }

    pub fn proxy(mut self, proxy: ProxyConfig) -> Self {
        self.inner.proxy = Some(proxy);
        self
    }

    pub fn ignore_http_error_codes(mut self, val: bool) -> Self {
        self.inner.ignore_http_error_codes = val;
        self
    }

    pub fn wrap_unexpected_exceptions(mut self, val: bool) -> Self {
        self.inner.wrap_unexpected_exceptions = val;
        self
    }

    pub fn json_config(mut self, config: JsonConfig) -> Self {
        self.inner.json_config = config;
        self
    }

    pub fn oauth(mut self, signer: Arc<dyn OAuthSigner>) -> Self {
        self.inner.oauth = Some(signer);
        self
    }

    pub fn connection_type(mut self, conn: ConnectionType) -> Self {
        self.inner.connection_type = conn;
        self
    }

    /// Add a default param: key -> value (serde_json::Value)
    pub fn default_param(mut self, key: impl Into<String>, val: impl Into<Value>) -> Self {
        self.inner.default_params.insert(key.into(), val.into());
        self
    }

    /// Add multiple default params at once
    pub fn default_params(
        mut self,
        data: impl IntoIterator<Item = (impl Into<String>, impl Into<Value>)>,
    ) -> Self {
        for (k, v) in data {
            self.inner.default_params.insert(k.into(), v.into());
        }
        self
    }

    pub fn build(self) -> ClientConfig {
        self.inner
    }

    pub fn finish(self) -> ClientConfig {
        self.build()
    }
}
