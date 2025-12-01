pub mod auth_utils;
pub mod time_nonce;

use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use serde::de::{self, Deserializer};
use serde_json::Value;

/// 辅助函数：尝试 Display，否则 Debug
pub fn display_value<T: std::fmt::Debug + 'static>(value: &T) -> String {
    if let Some(vec) = (value as &dyn std::any::Any).downcast_ref::<Vec<String>>() {
        vec.join(", ")
    } else {
        format!("{:?}", value)
    }
}

/// 反序列化 Binance serverTime（数字或字符串毫秒）为 `DateTime<Utc>`
pub fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    // 先把 JSON 反序列化成 Value
    let value = Value::deserialize(deserializer)?;

    // 根据类型处理
    let ts_millis = match value {
        Value::Number(num) => num
            .as_i64()
            .ok_or_else(|| de::Error::custom("Invalid number for timestamp"))?,
        Value::String(s) => s
            .parse::<i64>()
            .map_err(|_| de::Error::custom("Invalid string for timestamp"))?,
        _ => return Err(de::Error::custom("Unexpected type for timestamp")),
    };

    // 转换为 DateTime<Utc>
    Utc.timestamp_millis_opt(ts_millis)
        .single()
        .ok_or_else(|| de::Error::custom(format!("Invalid timestamp: {}", ts_millis)))
}
