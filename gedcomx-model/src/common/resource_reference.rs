use crate::common::Reference;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ResourceReference(Reference);

impl ResourceReference {
    pub fn new() -> Self {
        Self(Reference::new())
    }
    pub fn with_resource(resource: String) -> Self {
        Self(Reference::with_resource(resource))
    }
}

impl ResourceReference {
    pub fn resource(&self) -> &str {
        self.0.resource()
    }
}
