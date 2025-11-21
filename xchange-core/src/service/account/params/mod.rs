use crate::currency::currency::Currency;

mod default_request_deposit_address_params;

/// Parameters for requesting a deposit address from an exchange.
pub trait RequestDepositAddressParams {
    fn currency(&self) -> &Currency;
    fn network(&self) -> Option<&str>;
    fn is_new_address(&self) -> bool;
    fn extra_arguments(&self) -> &[String];
}
