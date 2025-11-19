/// Represents the health status of a wallet on the exchange.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletHealth {
    /// You can deposit and withdraw funds from the exchange
    Online,

    /// Deposits are disabled
    DepositsDisabled,

    /// Withdrawals are disabled
    WithdrawalsDisabled,

    /// You cannot deposit nor withdraw funds from the exchange
    Offline,

    /// The exchange does not inform us about the health of this wallet
    Unknown,
}
