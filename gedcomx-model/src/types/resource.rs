use crate::{common::Uri, ser::SerializeXml};
use quick_xml::events::{BytesStart, Event};

/// Enumeration of high-level genealogical resource types.
#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum ResourceType {
    /// A historical record.
    Record,

    /// A collection.
    Collection,

    /// A digital artifact, such as a digital image or video.
    DigitalArtifact,

    /// A physical artifact.
    PhysicalArtifact,

    /// A person.
    Person,
}

impl ResourceType {
    fn as_qname_uri(&self) -> &str {
        match self {
            Self::Record => "http://gedcomx.org/Record",
            Self::Collection => "http://gedcomx.org/Collection",
            Self::DigitalArtifact => "http://gedcomx.org/DigitalArtifact",
            Self::PhysicalArtifact => "http://gedcomx.org/PhysicalArtifact",
            Self::Person => "http://gedcomx.org/Person",
        }
    }
}

impl From<ResourceType> for Uri {
    fn from(r: ResourceType) -> Self {
        Uri::new(r.as_qname_uri().into())
    }
}
impl SerializeXml for ResourceType {
    fn tag(&self) -> &str {
        "resoureType"
    }

    fn serialize_xml<W: std::io::Write>(
        &self,
        ser: &mut quick_xml::Writer<W>,
    ) -> Result<(), crate::ser::SerError> {
        let mut elem = BytesStart::new(self.tag());
        elem.push_attribute(("type", self.as_qname_uri()));
        ser.write_event(Event::Empty(elem))?;
        Ok(())
    }
}
