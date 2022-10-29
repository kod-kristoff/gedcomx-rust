use crate::{
    common::Uri,
    ser::{xml, SerializeXml},
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SourceReference {
    description: Uri,
    #[serde(default, skip_serializing)]
    description_id: String,
}

impl SourceReference {
    pub fn new(description: Uri, description_id: String) -> Self {
        Self {
            description,
            description_id,
        }
    }
}

impl SerializeXml for SourceReference {
    fn tag(&self) -> &str {
        "source"
    }

    fn serialize_xml<W: std::io::Write>(
        &self,
        ser: &mut quick_xml::Writer<W>,
    ) -> Result<(), crate::ser::SerError> {
        xml::write_elem_w_attribute(ser, self.tag(), ("description", self.description.as_str()))?;
        Ok(())
    }
}
