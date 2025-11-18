use crate::dto::marketdata::trade::Trade;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeSortType {
    SortByTimestamp,
    SortByID,
}

impl TradeSortType {
    pub fn apply(&self, trades: &mut Vec<Trade>) {
        match self {
            TradeSortType::SortByTimestamp => {
                trades.sort_by(Self::cmp_trade_timestamp);
            }
            TradeSortType::SortByID => {
                trades.sort_by(Self::cmp_trade_id_bigint);
            }
        }
    }

    fn cmp_trade_timestamp(t1: &Trade, t2: &Trade) -> Ordering {
        t1.timestamp.cmp(&t2.timestamp)
    }

    fn cmp_trade_id_bigint(t1: &Trade, t2: &Trade) -> Ordering {
        const ALLOWED_RADIXES: &[u32] = &[10, 16];

        for &radix in ALLOWED_RADIXES {
            // 尝试解析 BigUint —— 和 Java 的 BigInteger 一样
            let id1 = BigUint::parse_bytes(t1.id.as_bytes(), radix);
            let id2 = BigUint::parse_bytes(t2.id.as_bytes(), radix);

            if let (Some(v1), Some(v2)) = (id1, id2) {
                return v1.cmp(&v2);
            }
        }

        // fallback：字符串比较 —— Java 也是这样
        t1.id.cmp(&t2.id)
    }
}

/// DTO representing a collection of trades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trades {
    pub trades: Vec<Trade>, // A vector to hold the collection of trades
    pub last_id: i64,       // Assuming last_id is a 64-bit integer (long in Java)
    pub next_page_cursor: Option<String>, // A string to hold the next page cursor
    pub trade_sort_type: TradeSortType, // The sorting type for trades
}

impl Trades {
    pub fn new(
        trades: Vec<Trade>,
        last_id: i64,
        trade_sort_type: TradeSortType,
        next_page_cursor: Option<String>,
    ) -> Self {
        let mut trades = trades;

        trade_sort_type.apply(&mut trades);

        Trades {
            trades,
            last_id,
            trade_sort_type,
            next_page_cursor,
        }
    }
}
