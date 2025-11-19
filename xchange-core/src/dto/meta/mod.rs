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

/// Represents the operational health status of an exchange.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeHealth {
    /// Exchange is fully operational
    Online,

    /// Exchange is offline
    Offline,

    /// Can only cancel orders but cannot place new orders
    CancelOnly,
}
