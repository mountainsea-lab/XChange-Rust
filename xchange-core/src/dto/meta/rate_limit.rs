use crate::TimeUnit;
use serde::{Deserialize, Deserializer, Serialize};

/// Describe a call rate limit as a number of calls per some time span.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    /// Number of calls allowed in the given time span.
    #[serde(default = "default_calls")]
    pub calls: u32,

    /// Time span value.
    #[serde(default = "default_time_span")]
    pub time_span: u64,

    /// Time unit of the time span.
    #[serde(
        default = "default_time_unit",
        deserialize_with = "deserialize_time_unit"
    )]
    pub time_unit: TimeUnit,
}

// ---- defaults ----

fn default_calls() -> u32 {
    1
}

fn default_time_span() -> u64 {
    1
}

fn default_time_unit() -> TimeUnit {
    TimeUnit::Seconds
}

// ---- constructors ----

impl RateLimit {
    ///  no-args constructor
    pub fn new() -> Self {
        Self::default()
    }

    /// full constructor
    pub fn with(calls: u32, time_span: u64, time_unit: TimeUnit) -> Self {
        Self {
            calls,
            time_span,
            time_unit,
        }
    }

    ///  getPollDelayMillis()
    pub fn poll_delay_millis(&self) -> u64 {
        self.time_unit.to_millis(self.time_span) / self.calls as u64
    }
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            calls: default_calls(),
            time_span: default_time_span(),
            time_unit: default_time_unit(),
        }
    }
}

impl TimeUnit {
    pub fn to_millis(self, value: u64) -> u64 {
        match self {
            TimeUnit::Nanoseconds => value / 1_000_000,
            TimeUnit::Microseconds => value / 1_000,
            TimeUnit::Milliseconds => value,
            TimeUnit::Seconds => value * 1_000,
            TimeUnit::Minutes => value * 60_000,
            TimeUnit::Hours => value * 3_600_000,
            TimeUnit::Days => value * 86_400_000,
        }
    }
}

/// Custom deserializer replicating Java TimeUnitDeserializer:
/// - Case-insensitive ("seconds", "SECONDS", "Seconds")
/// - Auto convert to SCREAMING_SNAKE_CASE
pub fn deserialize_time_unit<'de, D>(deserializer: D) -> Result<TimeUnit, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let upper = s.trim().to_uppercase();
    serde_json::from_str(&format!("\"{}\"", upper)).map_err(serde::de::Error::custom)
}
