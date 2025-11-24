use crate::TimeUnit;
use std::sync::atomic::{AtomicU64, Ordering};

/// 基于当前时间且保证单调递增的 nonce 生成器
pub struct TimeNonce {
    nonce: AtomicU64,
    unit: TimeUnit,
}

impl TimeNonce {
    pub fn new(unit: TimeUnit) -> Self {
        Self {
            nonce: AtomicU64::new(0),
            unit,
        }
    }

    /// 返回一个单调递增的 nonce
    pub fn next(&self) -> u64 {
        let now = self.unit.now();

        self.nonce
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |prev| {
                Some(now.max(prev))
            })
            .expect("fetch_update failed")
    }
}
