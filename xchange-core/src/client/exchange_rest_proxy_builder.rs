use crate::BuildError;
use crate::client::client_config::{ClientConfig, ProxyConfig, ProxyKind};
use crate::client::{ClientConfigCustomizer, Interceptor, RestProxyFactory};
use crate::exchange_specification::ExchangeSpecification;
use std::any::Any;
use std::sync::Arc;
use std::time::Duration;

pub struct ExchangeRestProxyBuilder {
    exchange_spec: ExchangeSpecification,
    base_url: String,
    client_config: Option<ClientConfig>,
    customizers: Vec<Arc<dyn ClientConfigCustomizer>>,
    interceptors: Vec<Arc<dyn Interceptor>>,
    proxy_factory: Arc<dyn RestProxyFactory>,
}

impl ExchangeRestProxyBuilder {
    /// 构造函数：从 ExchangeSpecification 获取 base_url
    pub fn for_interface(spec: ExchangeSpecification) -> Result<Self, BuildError> {
        let base_url = spec
            .ssl_uri
            .clone()
            .or_else(|| spec.plain_text_uri.clone())
            .ok_or(BuildError::MissingField("base_url missing".into()))?;

        Ok(Self {
            exchange_spec: spec,
            base_url,
            client_config: None,
            customizers: vec![],
            interceptors: vec![],
            proxy_factory: Arc::new(crate::client::DefaultProxyFactory),
        })
    }

    /// 链式设置 ClientConfig
    pub fn client_config(mut self, cfg: ClientConfig) -> Self {
        self.client_config = Some(cfg);
        self
    }

    /// 增加 ClientConfigCustomizer
    pub fn customize(mut self, c: Arc<dyn ClientConfigCustomizer>) -> Self {
        self.customizers.push(c);
        self
    }

    /// 设置 base_url
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// 添加单个 interceptor
    pub fn interceptor(mut self, ic: Arc<dyn Interceptor>) -> Self {
        self.interceptors.push(ic);
        self
    }

    /// 添加 interceptor 列表
    pub fn interceptors<I>(mut self, list: I) -> Self
    where
        I: IntoIterator<Item = Arc<dyn Interceptor>>,
    {
        self.interceptors.extend(list);
        self
    }

    /// 设置 ProxyFactory
    pub fn proxy_factory(mut self, pf: Arc<dyn RestProxyFactory>) -> Self {
        self.proxy_factory = pf;
        self
    }

    /// 异步构建，返回具体交易所客户端，由交易所模块定义 trait 或 struct
    pub async fn build(self) -> Box<dyn Any + Send + Sync> {
        let mut cfg = self
            .client_config
            .unwrap_or_else(|| Self::create_client_config(&self.exchange_spec));

        // 应用 customizer
        for c in &self.customizers {
            c.customize(&mut cfg);
        }

        self.proxy_factory
            .create_proxy(self.base_url, cfg, self.interceptors)
            .await
    }

    /// Java 的 createClientConfig 转译
    fn create_client_config(spec: &ExchangeSpecification) -> ClientConfig {
        let mut cfg = ClientConfig::default();

        cfg.timeout = Duration::from_millis(spec.http_conn_timeout);

        cfg.read_timeout = Duration::from_millis(spec.http_read_timeout);

        if let Some(host) = &spec.proxy_host {
            cfg.proxy = Some(ProxyConfig {
                host: host.clone(),
                port: spec.proxy_port.unwrap_or(80),
                kind: ProxyKind::Http,
            });
        }

        cfg
    }
}
