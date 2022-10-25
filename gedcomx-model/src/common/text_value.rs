#[derive(Debug, Clone)]
pub struct TextValue {
    value: String,
    lang: String,
}

impl From<String> for TextValue {
    fn from(value: String) -> Self {
        Self {
            value,
            lang: String::new(),
        }
    }
}
