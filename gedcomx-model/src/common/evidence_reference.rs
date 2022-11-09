use quick_xml::events::{BytesEnd, BytesStart, Event};

use crate::ser::SerializeXml;

use super::Reference;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EvidenceReference {
    #[serde(rename = "resource", flatten)]
    reference: Reference,
}

impl EvidenceReference {
    pub fn new() -> Self {
        Self {
            reference: Reference::new(),
        }
    }
    pub fn with_resource(resource: String) -> Self {
        Self {
            reference: Reference::with_resource(resource),
        }
    }
}

impl EvidenceReference {
    pub fn resource(&self) -> &str {
        self.reference.resource()
    }
}

impl SerializeXml for EvidenceReference {
    fn tag(&self) -> &str {
        "evidence"
    }

    fn serialize_xml<W: std::io::Write>(
        &self,
        ser: &mut quick_xml::Writer<W>,
    ) -> Result<(), crate::ser::SerError> {
        let mut elem = BytesStart::new(self.tag());
        elem.push_attribute(("resource", self.resource()));
        ser.write_event(Event::Empty(elem))?;
        // ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}
