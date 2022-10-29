#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Uri(String);

impl Uri {
    pub fn new(uri: String) -> Self {
        Self(uri)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
