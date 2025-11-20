use serde::{Deserialize, Serialize};

pub mod currency_metadata;
pub mod exchange_metadata;
pub mod fee_tier;
pub mod instrument_metadata;
pub mod rate_limit;

/// Represents the health status of a wallet on the exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExchangeHealth {
    /// Exchange is fully operational
    Online,

    /// Exchange is offline
    Offline,

    /// Can only cancel orders but cannot place new orders
    CancelOnly,
}
