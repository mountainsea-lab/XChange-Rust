use crate::instrument::Instrument;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OpenPosition {
    pub id: Option<String>,
    // The instrument
    pub instrument: Arc<dyn Instrument>,
    // Is this a long or a short position
    pub type_: PositionType,
    pub margin_mode: Option<MarginMode>,
    //  The size of the position
    pub size: Option<Decimal>,
    //  The average entry price for the position
    pub price: Option<Decimal>,
    // The estimated liquidation price
    pub liquidation_price: Option<Decimal>,
    // The unrealised pnl of the position
    pub un_realised_pnl: Option<Decimal>,
    // Timestamp of creation
    pub created_at: Option<DateTime<Utc>>,
    // Timestamp of update
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PositionType {
    Long,
    Short,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MarginMode {
    Cross,
    Isolated,
}

// 可选：实现 Display 对应 Java toString
impl std::fmt::Display for OpenPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OpenPosition {{ id: {:?}, instrument: {:?}, type: {:?}, margin_mode: {:?}, size: {:?}, price: {:?}, liquidation_price: {:?}, un_realised_pnl: {:?}, created_at: {:?}, updated_at: {:?} }}",
            self.id,
            self.instrument,
            self.type_,
            self.margin_mode,
            self.size,
            self.price,
            self.liquidation_price,
            self.un_realised_pnl,
            self.created_at,
            self.updated_at
        )
    }
}
// 使用 symbol 来判断两个 instrument 是否相等
impl PartialEq for OpenPosition {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.instrument.symbol() == other.instrument.symbol()
            && self.type_ == other.type_
            && self.margin_mode == other.margin_mode
    }
}

impl Eq for OpenPosition {}

// 使用 symbol 来生成哈希
impl Hash for OpenPosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.instrument.symbol().hash(state);
        self.type_.hash(state);
        self.margin_mode.hash(state);
    }
}
