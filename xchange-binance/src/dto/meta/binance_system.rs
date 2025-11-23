use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceSystemStatus {
    /// 0: normal, 1: system maintenance
    pub status: String,

    /// 描述信息：normal / system maintenance
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceTime {
    /// 服务器时间，Unix 毫秒时间戳
    #[serde(rename = "serverTime")]
    pub server_time: i64,
}

impl BinanceTime {
    /// 等价 Java: new Date(serverTime)
    pub fn server_time_datetime(&self) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(self.server_time)
            .single()
            .expect("invalid timestamp")
    }
}
