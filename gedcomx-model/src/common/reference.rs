#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Reference {
    resource: String, // TODO: use Uri type
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
}

impl Reference {
    pub fn new() -> Self {
        Self {
            resource: String::new(),
            resource_id: None,
        }
    }
    pub fn with_resource(resource: String) -> Self {
        Self {
            resource,
            resource_id: None,
        }
    }
}

impl Reference {
    pub fn resource(&self) -> &str {
        self.resource.as_str()
    }
}
