use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use xchange_core::currency::currency::Currency;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceCurrencyInfo {
    #[serde(rename = "coin")]
    pub currency: Currency, // 假设 Currency 已定义

    #[serde(rename = "depositAllEnable")]
    pub deposit_enabled: Option<bool>,

    #[serde(rename = "withdrawAllEnable")]
    pub withdraw_enabled: Option<bool>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "free")]
    pub free: Option<Decimal>,

    #[serde(rename = "locked")]
    pub locked: Option<Decimal>,

    #[serde(rename = "freeze")]
    pub freeze: Option<Decimal>,

    #[serde(rename = "withdrawing")]
    pub withdrawing: Option<Decimal>,

    #[serde(rename = "ipoing")]
    pub ipoing: Option<Decimal>,

    #[serde(rename = "ipoable")]
    pub ipoable: Option<Decimal>,

    #[serde(rename = "storage")]
    pub storage: Option<Decimal>,

    #[serde(rename = "isLegalMoney")]
    pub is_legal_money: Option<bool>,

    #[serde(rename = "trading")]
    pub trading: Option<bool>,

    #[serde(rename = "networkList")]
    pub networks: Option<Vec<Network>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Network {
    #[serde(rename = "network")]
    pub id: Option<String>,

    #[serde(rename = "coin")]
    pub currency: Option<Currency>,

    #[serde(rename = "withdrawIntegerMultiple")]
    pub withdraw_integer_multiple: Option<Decimal>,

    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,

    #[serde(rename = "depositEnable")]
    pub deposit_enabled: Option<bool>,

    #[serde(rename = "withdrawEnable")]
    pub withdraw_enabled: Option<bool>,

    #[serde(rename = "depositDesc")]
    pub deposit_desc: Option<String>,

    #[serde(rename = "withdrawDesc")]
    pub withdraw_desc: Option<String>,

    #[serde(rename = "specialTips")]
    pub special_tips: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "resetAddressStatus")]
    pub reset_address_status: Option<bool>,

    #[serde(rename = "addressRegex")]
    pub address_regex: Option<String>,

    #[serde(rename = "memoRegex")]
    pub memo_regex: Option<String>,

    #[serde(rename = "withdrawFee")]
    pub withdraw_fee: Option<Decimal>,

    #[serde(rename = "withdrawMin")]
    pub withdraw_min: Option<Decimal>,

    #[serde(rename = "withdrawMax")]
    pub withdraw_max: Option<Decimal>,

    #[serde(rename = "minConfirm")]
    pub min_confirm: Option<u32>,

    #[serde(rename = "unLockConfirm")]
    pub unlock_confirm: Option<u32>,

    #[serde(rename = "sameAddress")]
    pub same_address: Option<bool>,

    #[serde(rename = "estimatedArrivalTime")]
    pub estimated_arrival_time: Option<u32>,

    #[serde(rename = "busy")]
    pub busy: Option<bool>,

    #[serde(rename = "contractAddressUrl")]
    pub contract_address_url: Option<String>,

    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
}
