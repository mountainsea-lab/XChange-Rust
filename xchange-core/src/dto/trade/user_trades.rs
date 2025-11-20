use crate::dto::marketdata::trade::Trade;
use crate::dto::marketdata::trades::{TradeSortType, Trades};
use crate::dto::trade::user_trade::UserTrade;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTrades {
    pub trades: Trades, // This is the composition, as Rust doesn't have inheritance
}

impl UserTrades {
    // Constructor with trades and sort type
    pub fn new(user_trades: Vec<UserTrade>, trade_sort_type: TradeSortType) -> Self {
        let trades = Trades::new(
            user_trades
                .into_iter()
                .map(|ut| Trade {
                    order_type: ut.trade.order_type,
                    original_amount: ut.trade.original_amount,
                    instrument: ut.trade.instrument.clone(),
                    id: ut.trade.id.clone(),
                    maker_order_id: ut.trade.maker_order_id.clone(),
                    timestamp: ut.trade.timestamp, // You can adapt based on the `UserTrade` fields
                    price: ut.trade.price,
                    taker_order_id: ut.trade.taker_order_id.clone(),
                })
                .collect(),
            0, // last_id: default 0
            trade_sort_type,
            None, // no cursor
        );
        UserTrades { trades }
    }

    // Constructor with trades, last_id, and sort type
    pub fn new_with_last_id(
        user_trades: Vec<UserTrade>,
        last_id: i64,
        trade_sort_type: TradeSortType,
    ) -> Self {
        let trades = Trades::new(
            user_trades
                .into_iter()
                .map(|ut| Trade {
                    order_type: ut.trade.order_type,
                    original_amount: ut.trade.original_amount,
                    instrument: ut.trade.instrument.clone(),
                    id: ut.trade.id.clone(),
                    maker_order_id: ut.trade.maker_order_id.clone(),
                    timestamp: ut.trade.timestamp, // You can adapt based on the `UserTrade` fields
                    price: ut.trade.price,
                    taker_order_id: ut.trade.taker_order_id.clone(),
                })
                .collect(),
            last_id, // last_id: default 0
            trade_sort_type,
            None, // no cursor
        );
        UserTrades { trades }
    }

    // Constructor with trades, last_id, sort type, and next page cursor
    pub fn new_with_cursor(
        user_trades: Vec<UserTrade>,
        last_id: i64,
        trade_sort_type: TradeSortType,
        next_page_cursor: Option<String>,
    ) -> Self {
        let trades = Trades::new(
            user_trades
                .into_iter()
                .map(|ut| Trade {
                    order_type: ut.trade.order_type,
                    original_amount: ut.trade.original_amount,
                    instrument: ut.trade.instrument.clone(),
                    id: ut.trade.id.clone(),
                    maker_order_id: ut.trade.maker_order_id.clone(),
                    timestamp: ut.trade.timestamp, // You can adapt based on the `UserTrade` fields
                    price: ut.trade.price,
                    taker_order_id: ut.trade.taker_order_id.clone(),
                })
                .collect(),
            last_id,
            trade_sort_type,
            next_page_cursor,
        );
        UserTrades { trades }
    }
}

// Display implementation for UserTrades
impl fmt::Display for UserTrades {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserTrades [trades_count={}]", self.trades.trades.len())
    }
}
