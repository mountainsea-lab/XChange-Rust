pub mod currency;

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// 货币属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyAttributes {
    /// 所有货币代码(包括通用代码和替代代码)
    pub codes: BTreeSet<String>,

    /// ISO 货币代码(如果存在)
    pub iso_code: Option<String>,

    /// 通用代码(主要使用的代码)
    pub common_code: String,

    /// 货币名称
    pub name: String,

    /// Unicode 符号
    pub unicode: String,
}

impl CurrencyAttributes {
    /// 创建新的货币属性
    pub fn new(
        common_code: impl Into<String>,
        name: Option<String>,
        unicode: Option<String>,
        alternative_codes: &[String],
    ) -> Self {
        let common_code = common_code.into();

        // 构建代码集合
        let mut codes = BTreeSet::new();
        codes.insert(common_code.clone());
        for code in alternative_codes {
            codes.insert(code.clone());
        }

        // 尝试识别 ISO 代码和加密货币代码
        let mut iso_code = None;
        let mut possible_iso_proposal_crypto_code = None;

        for code in &codes {
            // 尝试识别标准 ISO 货币代码
            // 注: Rust 标准库没有 java.util.Currency 等价物
            // 可以使用 iso_currency crate 或自定义逻辑

            // 识别以 X 开头的加密货币代码
            if code.starts_with('X') {
                possible_iso_proposal_crypto_code = Some(code.clone());
            }
        }

        // 如果没有找到标准 ISO 代码,使用加密货币代码
        if iso_code.is_none() {
            iso_code = possible_iso_proposal_crypto_code;
        }

        // 确定名称
        let final_name = name.unwrap_or_else(|| common_code.clone());

        // 确定 Unicode 符号
        let final_unicode = unicode.unwrap_or_else(|| common_code.clone());

        Self {
            codes,
            iso_code,
            common_code,
            name: final_name,
            unicode: final_unicode,
        }
    }

    /// 检查是否包含指定代码
    pub fn contains_code(&self, code: &str) -> bool {
        self.codes.contains(code)
    }
}

impl std::hash::Hash for CurrencyAttributes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.common_code.hash(state);
    }
}

impl Eq for CurrencyAttributes {}

// 基于 common_code 的相等性比较
impl PartialEq for CurrencyAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.common_code == other.common_code
    }
}
