use crate::currency::currency::Currency;
use crate::service::account::params::RequestDepositAddressParams;

/// Default implementation of `RequestDepositAddressParams`.
#[derive(Debug, Clone)]
pub struct DefaultRequestDepositAddressParams {
    /// Currency for the deposit address
    pub currency: Currency,

    /// Optional network (chain) for the deposit
    pub network: Option<String>,

    /// Whether to request a new address (default: false)
    pub new_address: bool,

    /// Extra arguments specific to the exchange
    pub extra_arguments: Vec<String>,
}

impl DefaultRequestDepositAddressParams {
    /// Convenience method to create with currency and extra arguments
    pub fn create(currency: Currency, args: &[impl AsRef<str>]) -> Self {
        Self {
            currency,
            network: None,
            new_address: false,
            extra_arguments: args.iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }

    /// Builder pattern
    pub fn builder() -> DefaultRequestDepositAddressParamsBuilder {
        DefaultRequestDepositAddressParamsBuilder::default()
    }
}

/// Builder for `DefaultRequestDepositAddressParams`
#[derive(Debug, Default)]
pub struct DefaultRequestDepositAddressParamsBuilder {
    currency: Option<Currency>,
    network: Option<String>,
    new_address: bool,
    extra_arguments: Vec<String>,
}

impl DefaultRequestDepositAddressParamsBuilder {
    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn network(mut self, network: impl Into<String>) -> Self {
        self.network = Some(network.into());
        self
    }

    pub fn new_address(mut self, flag: bool) -> Self {
        self.new_address = flag;
        self
    }

    pub fn extra_arguments(mut self, args: &[impl AsRef<str>]) -> Self {
        self.extra_arguments = args.iter().map(|s| s.as_ref().to_string()).collect();
        self
    }

    pub fn build(self) -> DefaultRequestDepositAddressParams {
        DefaultRequestDepositAddressParams {
            currency: self.currency.expect("currency must be set"),
            network: self.network,
            new_address: self.new_address,
            extra_arguments: self.extra_arguments,
        }
    }
}

impl RequestDepositAddressParams for DefaultRequestDepositAddressParams {
    fn currency(&self) -> &Currency {
        &self.currency
    }

    fn network(&self) -> Option<&str> {
        self.network.as_deref()
    }

    fn is_new_address(&self) -> bool {
        self.new_address
    }

    fn extra_arguments(&self) -> &[String] {
        &self.extra_arguments
    }
}
