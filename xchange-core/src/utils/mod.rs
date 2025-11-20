/// 辅助函数：尝试 Display，否则 Debug
pub fn display_value<T: std::fmt::Debug + 'static>(value: &T) -> String {
    if let Some(vec) = (value as &dyn std::any::Any).downcast_ref::<Vec<String>>() {
        vec.join(", ")
    } else {
        format!("{:?}", value)
    }
}
