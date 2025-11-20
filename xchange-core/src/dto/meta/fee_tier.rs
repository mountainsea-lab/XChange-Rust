use crate::dto::account::fee::Fee;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct FeeTier {
    #[serde(rename = "begin_quantity")]
    pub begin_quantity: rust_decimal::Decimal,

    #[serde(rename = "fee")]
    pub fee: Fee,
}

impl FeeTier {
    pub fn new(begin_quantity: rust_decimal::Decimal, fee: Fee) -> Self {
        Self {
            begin_quantity,
            fee,
        }
    }
}

// ---------- 自定义排序，等价于 Java 的 Comparable ----------

impl PartialEq for FeeTier {
    fn eq(&self, other: &Self) -> bool {
        self.begin_quantity == other.begin_quantity
    }
}

impl Eq for FeeTier {}

impl PartialOrd for FeeTier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.begin_quantity.cmp(&other.begin_quantity))
    }
}

impl Ord for FeeTier {
    fn cmp(&self, other: &Self) -> Ordering {
        self.begin_quantity.cmp(&other.begin_quantity)
    }
}

// ---------- toString() 等价 ----------

impl fmt::Display for FeeTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FeeTier [beginQuantity={}, fee={:?}]",
            self.begin_quantity, self.fee
        )
    }
}
