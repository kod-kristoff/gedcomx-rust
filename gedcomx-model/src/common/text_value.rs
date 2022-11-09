#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TextValue {
    value: String,
    lang: Option<String>,
}

impl TextValue {
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    pub fn lang(&self) -> Option<&str> {
        self.lang.as_ref().map(|s| s.as_str())
    }
}

impl<S: Into<String>> From<S> for TextValue {
    fn from(value: S) -> Self {
        Self {
            value: value.into(),
            lang: None,
        }
    }
}
