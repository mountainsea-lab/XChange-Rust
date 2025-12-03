pub mod binance_kline;

use serde::Deserializer;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum KlineInterval {
    M1,
    M3,
    M5,
    M15,
    M30,

    H1,
    H2,
    H4,
    H6,
    H8,
    H12,

    D1,
    D3,

    W1,

    Mo1,
}

impl KlineInterval {
    pub fn code(&self) -> &'static str {
        match self {
            Self::M1 => "1m",
            Self::M3 => "3m",
            Self::M5 => "5m",
            Self::M15 => "15m",
            Self::M30 => "30m",

            Self::H1 => "1h",
            Self::H2 => "2h",
            Self::H4 => "4h",
            Self::H6 => "6h",
            Self::H8 => "8h",
            Self::H12 => "12h",

            Self::D1 => "1d",
            Self::D3 => "3d",

            Self::W1 => "1w",

            Self::Mo1 => "1M",
        }
    }

    pub fn millis(&self) -> u64 {
        match self {
            Self::M1 => 60_000,
            Self::M3 => 180_000,
            Self::M5 => 300_000,
            Self::M15 => 900_000,
            Self::M30 => 1_800_000,

            Self::H1 => 3_600_000,
            Self::H2 => 7_200_000,
            Self::H4 => 14_400_000,
            Self::H6 => 21_600_000,
            Self::H8 => 28_800_000,
            Self::H12 => 43_200_000,

            Self::D1 => 86_400_000,
            Self::D3 => 259_200_000,

            Self::W1 => 604_800_000,

            Self::Mo1 => 2_592_000_000,
        }
    }

    pub fn from_secs(period_secs: u64) -> Option<Self> {
        let ms = period_secs * 1000;
        match ms {
            60_000 => Some(Self::M1),
            180_000 => Some(Self::M3),
            300_000 => Some(Self::M5),
            900_000 => Some(Self::M15),
            1_800_000 => Some(Self::M30),

            3_600_000 => Some(Self::H1),
            7_200_000 => Some(Self::H2),
            14_400_000 => Some(Self::H4),
            21_600_000 => Some(Self::H6),
            28_800_000 => Some(Self::H8),
            43_200_000 => Some(Self::H12),

            86_400_000 => Some(Self::D1),
            259_200_000 => Some(Self::D3),

            604_800_000 => Some(Self::W1),

            2_592_000_000 => Some(Self::Mo1),

            _ => None,
        }
    }

    pub const fn all() -> &'static [Self] {
        &[
            Self::M1,
            Self::M3,
            Self::M5,
            Self::M15,
            Self::M30,
            Self::H1,
            Self::H2,
            Self::H4,
            Self::H6,
            Self::H8,
            Self::H12,
            Self::D1,
            Self::D3,
            Self::W1,
            Self::Mo1,
        ]
    }
}

impl<'de> Deserialize<'de> for KlineInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::try_from(s.as_str()).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<&str> for KlineInterval {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "1m" => Ok(Self::M1),
            "3m" => Ok(Self::M3),
            "5m" => Ok(Self::M5),
            "15m" => Ok(Self::M15),
            "30m" => Ok(Self::M30),

            "1h" => Ok(Self::H1),
            "2h" => Ok(Self::H2),
            "4h" => Ok(Self::H4),
            "6h" => Ok(Self::H6),
            "8h" => Ok(Self::H8),
            "12h" => Ok(Self::H12),

            "1d" => Ok(Self::D1),
            "3d" => Ok(Self::D3),

            "1w" => Ok(Self::W1),

            "1M" => Ok(Self::Mo1),

            _ => Err("invalid KlineInterval"),
        }
    }
}

impl fmt::Display for KlineInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}
