pub mod currency;

use std::collections::BTreeSet;

/// 货币属性结构体
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyAttributes {
    /// 包含常用代码 + 可选别名
    pub codes: BTreeSet<&'static str>,
    /// ISO 货币代码，如果没有则可能是 X 开头的提议代码
    pub iso_code: &'static str,
    /// 常用代码（primary code）
    pub common_code: &'static str,
    /// 名称
    pub name: &'static str,
    /// Unicode 符号（如 $、€）
    pub unicode: &'static str,
}

impl CurrencyAttributes {
    /// 构造函数
    pub fn new(
        common_code: &'static str,
        name: Option<&'static str>,
        unicode: Option<&'static str>,
        alternative_codes: &[&'static str],
    ) -> Self {
        // 1. 构建 codes 集合（去重且有序）
        let mut codes: BTreeSet<&'static str> = alternative_codes.iter().copied().collect();
        codes.insert(common_code);

        // 2. 尝试找到 ISO 代码：优先非 X 开头，否则取 X 开头提议
        let mut iso_code: Option<&'static str> = None;
        let mut possible_x_code: Option<&'static str> = None;

        for &code in &codes {
            if iso_code.is_none() && !code.starts_with('X') {
                // 这里 Java 用 Currency.getInstance，我们暂时直接用 code 本身
                iso_code = Some(code);
            }
            if code.starts_with('X') {
                possible_x_code = Some(code);
            }
        }

        let iso_code = iso_code.or(possible_x_code).unwrap_or(common_code);

        // 3. name 和 unicode 的默认处理
        let name = name.unwrap_or(common_code);
        let unicode = unicode.unwrap_or(common_code);

        Self {
            codes,
            iso_code,
            common_code,
            name,
            unicode,
        }
    }
}
