use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

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

/// Enum representing funding transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundingType {
    Withdrawal,
    Deposit,
    Airdrop,
    OtherInflow,
    OtherOutflow,
    InternalWalletTransfer,
    InternalSubAccountTransfer,
    InternalWithdrawal,
    InternalDeposit,
    RealisedLoss,
    RealisedProfit,
    Trade,
    OrderFee,
    WithdrawalFee,
}

impl FundingType {
    /// 是否是流入
    pub fn is_inflowing(&self) -> bool {
        matches!(
            self,
            FundingType::Deposit
                | FundingType::Airdrop
                | FundingType::OtherInflow
                | FundingType::InternalDeposit
                | FundingType::RealisedProfit
        )
    }

    /// 是否是流出
    pub fn is_outflowing(&self) -> bool {
        !self.is_inflowing()
    }

    /// 将字符串转为 FundingType
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "WITHDRAWAL" => Some(FundingType::Withdrawal),
            "DEPOSIT" => Some(FundingType::Deposit),
            "AIRDROP" => Some(FundingType::Airdrop),
            "OTHER_INFLOW" => Some(FundingType::OtherInflow),
            "OTHER_OUTFLOW" => Some(FundingType::OtherOutflow),
            "INTERNAL_WALLET_TRANSFER" => Some(FundingType::InternalWalletTransfer),
            "INTERNAL_SUB_ACCOUNT_TRANSFER" => Some(FundingType::InternalSubAccountTransfer),
            "INTERNAL_WITHDRAWAL" => Some(FundingType::InternalWithdrawal),
            "INTERNAL_DEPOSIT" => Some(FundingType::InternalDeposit),
            "REALISED_LOSS" => Some(FundingType::RealisedLoss),
            "REALISED_PROFIT" => Some(FundingType::RealisedProfit),
            "TRADE" => Some(FundingType::Trade),
            "ORDER_FEE" => Some(FundingType::OrderFee),
            "WITHDRAWAL_FEE" => Some(FundingType::WithdrawalFee),
            _ => None,
        }
    }
}

// 实现 FromStr trait，可以直接用 "str".parse::<FundingType>()
impl FromStr for FundingType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FundingType::from_string(s).ok_or(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    // The user has requested the withdrawal or deposit, or the exchange has detected an initiated
    // deposit, but the exchange still has to fully process the funding. The funds are not available
    // to the user. The funding request may possibly still be cancelled though.
    Processing,
    // The exchange has processed the transfer fully and successfully. The funding typically cannot
    // be cancelled any more. For withdrawals, the funds are gone from the exchange, though they may
    // have not reached their destination yet. For deposits, the funds are available to the user.
    Complete,
    // The transfer was cancelled either by the user or by the exchange.
    Cancelled,
    //  The transfer has failed for any reason other than user cancellation after it was initiated
    //     and before it was successfully processed. For withdrawals, the funds are available to the
    //     user again.
    Failed,
}

/// fromString map
static STATUS_MAP: Lazy<HashMap<&'static str, Status>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Processing 对应多种字符串
    for s in &[
        "WAIT CONFIRMATION",
        "EMAIL CONFIRMATION",
        "EMAIL SENT",
        "AWAITING APPROVAL",
        "VERIFYING",
        "PENDING_APPROVAL",
        "PENDING",
        "PROCESSING", // 枚举名本身
    ] {
        m.insert(*s, Status::Processing);
    }

    // Complete
    for s in &["COMPLETED", "COMPLETE"] {
        m.insert(*s, Status::Complete);
    }

    // Cancelled
    for s in &["REVOKED", "CANCEL", "REFUND", "CANCELLED"] {
        m.insert(*s, Status::Cancelled);
    }

    // Failed
    for s in &["FAILURE", "FAILED"] {
        m.insert(*s, Status::Failed);
    }

    m
});

impl Status {
    pub fn resolve_status(s: &str) -> Option<Status> {
        STATUS_MAP.get(&s.to_uppercase()[..]).copied()
    }
}
