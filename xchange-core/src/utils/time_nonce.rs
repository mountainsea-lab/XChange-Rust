use crate::{TimeUnit, ValueFactory};
use std::sync::Arc;
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

    /// 获取单调递增的 nonce
    ///
    /// 保证：
    /// - 当前时间戳 >= 上一个返回值
    /// - 如果时间戳未增加，则使用上一个值 + 1
    /// - 避免 u64 溢出
    pub fn next(&self) -> u64 {
        loop {
            let prev = self.nonce.load(Ordering::SeqCst);
            let now = self.unit.now();
            let next = now.max(prev.saturating_add(1));
            // 尝试原子更新，如果失败，循环重试
            if self
                .nonce
                .compare_exchange(prev, next, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                return next;
            }
        }
    }
}

// 直接在 Arc<TimeNonce> 上实现 trait，方便共享
impl ValueFactory<u64> for Arc<TimeNonce> {
    fn create(&self) -> u64 {
        self.next()
    }
}
