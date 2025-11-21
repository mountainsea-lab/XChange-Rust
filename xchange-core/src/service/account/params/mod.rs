mod default_request_deposit_address_params;

/// Parameters for requesting a deposit address from an exchange.
pub trait RequestDepositAddressParams {
    fn currency(&self) -> &str;
    fn network(&self) -> Option<&str>;
    fn is_new_address(&self) -> bool;
    fn extra_arguments(&self) -> &[String];
}
