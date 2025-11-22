use serde_json::Value;
use std::collections::HashMap;

/// Configuration for resilience behavior of an exchange (retry and rate limiting).
#[derive(Debug, Clone, Copy, Default)]
pub struct ResilienceSpecification {
    /// If true, retry functionality is enabled (if implemented for the exchange)
    pub retry_enabled: bool,

    /// If true, call rate limiting functionality is enabled (if implemented for the exchange)
    pub rate_limiter_enabled: bool,
}

impl ResilienceSpecification {
    /// Create a new default specification (both features disabled)
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable retry
    pub fn set_retry_enabled(&mut self, enabled: bool) {
        self.retry_enabled = enabled;
    }

    /// Check if retry is enabled
    pub fn is_retry_enabled(&self) -> bool {
        self.retry_enabled
    }

    /// Enable or disable rate limiter
    pub fn set_rate_limiter_enabled(&mut self, enabled: bool) {
        self.rate_limiter_enabled = enabled;
    }

    /// Check if rate limiter is enabled
    pub fn is_rate_limiter_enabled(&self) -> bool {
        self.rate_limiter_enabled
    }
}
/// ExchangeSpecification with builder
#[derive(Debug, Clone)]
pub struct ExchangeSpecification {
    pub exchange_name: Option<String>,
    pub exchange_description: Option<String>,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub secret_key: Option<String>,
    pub api_key: Option<String>,
    pub ssl_uri: Option<String>,
    pub plain_text_uri: Option<String>,
    pub override_websocket_api_uri: Option<String>,
    pub host: Option<String>,
    pub port: u16,
    pub proxy_host: Option<String>,
    pub proxy_port: Option<u16>,
    pub http_conn_timeout: u64,
    pub http_read_timeout: u64,
    pub resilience: ResilienceSpecification,
    pub meta_data_json_file_override: Option<String>,
    pub should_load_remote_meta_data: bool,
    pub exchange_specific_parameters: HashMap<String, Value>,
}

impl ExchangeSpecification {
    pub fn builder() -> ExchangeSpecificationBuilder {
        ExchangeSpecificationBuilder::default()
    }

    pub fn get_parameter(&self, key: &str) -> Option<&Value> {
        self.exchange_specific_parameters.get(key)
    }
}

/// Builder for ExchangeSpecification
#[derive(Debug)]
pub struct ExchangeSpecificationBuilder {
    exchange_name: Option<String>,
    exchange_description: Option<String>,
    user_name: Option<String>,
    password: Option<String>,
    secret_key: Option<String>,
    api_key: Option<String>,
    ssl_uri: Option<String>,
    plain_text_uri: Option<String>,
    override_websocket_api_uri: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    proxy_host: Option<String>,
    proxy_port: Option<u16>,
    http_conn_timeout: Option<u64>,
    http_read_timeout: Option<u64>,
    resilience: Option<ResilienceSpecification>,
    meta_data_json_file_override: Option<String>,
    should_load_remote_meta_data: Option<bool>,
    exchange_specific_parameters: HashMap<String, Value>,
}

impl Default for ExchangeSpecificationBuilder {
    fn default() -> Self {
        Self {
            exchange_name: None,
            exchange_description: None,
            user_name: None,
            password: None,
            secret_key: None,
            api_key: None,
            ssl_uri: None,
            plain_text_uri: None,
            override_websocket_api_uri: None,
            host: None,
            port: None,
            proxy_host: None,
            proxy_port: None,
            http_conn_timeout: None,
            http_read_timeout: None,
            resilience: None,
            meta_data_json_file_override: None,
            should_load_remote_meta_data: None,
            exchange_specific_parameters: HashMap::new(),
        }
    }
}

impl ExchangeSpecificationBuilder {
    pub fn exchange_name(mut self, name: impl Into<String>) -> Self {
        self.exchange_name = Some(name.into());
        self
    }

    pub fn exchange_description(mut self, desc: impl Into<String>) -> Self {
        self.exchange_description = Some(desc.into());
        self
    }

    pub fn user_name(mut self, name: impl Into<String>) -> Self {
        self.user_name = Some(name.into());
        self
    }

    pub fn password(mut self, pwd: impl Into<String>) -> Self {
        self.password = Some(pwd.into());
        self
    }

    pub fn secret_key(mut self, key: impl Into<String>) -> Self {
        self.secret_key = Some(key.into());
        self
    }

    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn ssl_uri(mut self, uri: impl Into<String>) -> Self {
        self.ssl_uri = Some(uri.into());
        self
    }

    pub fn plain_text_uri(mut self, uri: impl Into<String>) -> Self {
        self.plain_text_uri = Some(uri.into());
        self
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn proxy_host(mut self, host: impl Into<String>) -> Self {
        self.proxy_host = Some(host.into());
        self
    }

    pub fn proxy_port(mut self, port: u16) -> Self {
        self.proxy_port = Some(port);
        self
    }

    pub fn http_conn_timeout(mut self, timeout: u64) -> Self {
        self.http_conn_timeout = Some(timeout);
        self
    }

    pub fn http_read_timeout(mut self, timeout: u64) -> Self {
        self.http_read_timeout = Some(timeout);
        self
    }

    pub fn resilience(mut self, resilience: ResilienceSpecification) -> Self {
        self.resilience = Some(resilience);
        self
    }

    pub fn meta_data_json_file_override(mut self, path: impl Into<String>) -> Self {
        self.meta_data_json_file_override = Some(path.into());
        self
    }

    pub fn should_load_remote_meta_data(mut self, value: bool) -> Self {
        self.should_load_remote_meta_data = Some(value);
        self
    }

    pub fn exchange_specific_parameter(mut self, key: impl Into<String>, value: Value) -> Self {
        self.exchange_specific_parameters.insert(key.into(), value);
        self
    }

    pub fn build(self) -> ExchangeSpecification {
        ExchangeSpecification {
            exchange_name: self.exchange_name,
            exchange_description: self.exchange_description,
            user_name: self.user_name,
            password: self.password,
            secret_key: self.secret_key,
            api_key: self.api_key,
            ssl_uri: self.ssl_uri,
            plain_text_uri: self.plain_text_uri,
            override_websocket_api_uri: self.override_websocket_api_uri,
            host: self.host,
            port: self.port.unwrap_or(80),
            proxy_host: self.proxy_host,
            proxy_port: self.proxy_port,
            http_conn_timeout: self.http_conn_timeout.unwrap_or(0),
            http_read_timeout: self.http_read_timeout.unwrap_or(0),
            resilience: self.resilience.unwrap_or_default(),
            meta_data_json_file_override: self.meta_data_json_file_override,
            should_load_remote_meta_data: self.should_load_remote_meta_data.unwrap_or(true),
            exchange_specific_parameters: self.exchange_specific_parameters,
        }
    }
}
