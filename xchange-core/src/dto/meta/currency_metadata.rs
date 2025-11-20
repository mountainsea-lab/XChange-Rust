use crate::dto::meta::WalletHealth;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Metadata for a specific currency on the exchange.
///
/// Includes fee information, scale, minimum withdrawal,
/// and wallet health status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyMetaData {
    /// Number of decimal places supported.
    #[serde(rename = "scale")]
    pub scale: Option<u32>,

    /// Withdrawal fee.
    #[serde(rename = "withdrawal_fee")]
    pub withdrawal_fee: Option<Decimal>,

    /// Minimum withdrawal amount.
    #[serde(rename = "min_withdrawal_amount")]
    pub min_withdrawal_amount: Option<Decimal>,

    /// Wallet health status.
    #[serde(rename = "wallet_health")]
    pub wallet_health: WalletHealth,
}

impl CurrencyMetaData {
    /// Constructor equivalent to Java's:
    /// `new CurrencyMetaData(scale, withdrawalFee)`
    pub fn new(scale: Option<u32>, withdrawal_fee: Option<Decimal>) -> Self {
        Self::new_full(scale, withdrawal_fee, None, WalletHealth::Unknown)
    }

    /// Constructor equivalent to Java's:
    /// `new CurrencyMetaData(scale, withdrawalFee, minWithdrawalAmount)`
    pub fn new_with_min(
        scale: Option<u32>,
        withdrawal_fee: Option<Decimal>,
        min_withdrawal_amount: Option<Decimal>,
    ) -> Self {
        Self::new_full(
            scale,
            withdrawal_fee,
            min_withdrawal_amount,
            WalletHealth::Unknown,
        )
    }

    /// Full constructor equivalent to Java's main constructor.
    pub fn new_full(
        scale: Option<u32>,
        withdrawal_fee: Option<Decimal>,
        min_withdrawal_amount: Option<Decimal>,
        wallet_health: WalletHealth,
    ) -> Self {
        Self {
            scale,
            withdrawal_fee,
            min_withdrawal_amount,
            wallet_health,
        }
    }
}

impl fmt::Display for CurrencyMetaData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CurrencyMetaData [scale={:?}, withdrawalFee={:?}, minWithdrawalAmount={:?}, walletHealth={:?}]",
            self.scale, self.withdrawal_fee, self.min_withdrawal_amount, self.wallet_health
        )
    }
}
