use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fee {
    // 0.1% fee eq 0.001 positive
    #[serde(rename = "maker_fee")]
    maker_fee: Decimal,

    #[serde(rename = "taker_fee")]
    taker_fee: Decimal,
}

impl Fee {
    pub fn new(maker_fee: Decimal, taker_fee: Decimal) -> Self {
        Self {
            maker_fee,
            taker_fee,
        }
    }

    pub fn maker_fee(&self) -> Decimal {
        self.maker_fee
    }

    pub fn taker_fee(&self) -> Decimal {
        self.taker_fee
    }
}

impl fmt::Display for Fee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Fee [makerFee={}, takerFee={}]",
            self.maker_fee, self.taker_fee
        )
    }
}

impl Hash for Fee {
    fn hash<H: Hasher>(&self, state: &mut H) {
        //  makerFee.hashCode() + 31 * takerFee.hashCode()
        let maker_hash = {
            let mut s = std::collections::hash_map::DefaultHasher::new();
            self.maker_fee.hash(&mut s);
            s.finish()
        };
        let taker_hash = {
            let mut s = std::collections::hash_map::DefaultHasher::new();
            self.taker_fee.hash(&mut s);
            s.finish()
        };

        // a + 31 * b
        let combined = maker_hash.wrapping_add(31u64.wrapping_mul(taker_hash));
        combined.hash(state);
    }
}
