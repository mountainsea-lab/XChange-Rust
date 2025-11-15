use serde::{Deserialize, Serialize};
use std::fmt;

pub mod address_with_tag;
pub mod balance;
pub mod fee;
pub mod wallet;

/// wallet enum
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WalletFeature {
    /// The wallet has the ability to deposit external funds and withdraw funds allocated on it
    Funding,
    /// You can trade funds allocated to this wallet
    Trading,
    /// You can do margin trading with funds allocated to this wallet
    MarginTrading,
    /// You can fund other margin traders with funds allocated to this wallet to earn an interest
    MarginFunding,
    /// Wallet for futures platform
    FuturesTrading,
}

impl fmt::Display for WalletFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            WalletFeature::Funding => "FUNDING",
            WalletFeature::Trading => "TRADING",
            WalletFeature::MarginTrading => "MARGIN_TRADING",
            WalletFeature::MarginFunding => "MARGIN_FUNDING",
            WalletFeature::FuturesTrading => "FUTURES_TRADING",
        };
        write!(f, "{}", s)
    }
}
