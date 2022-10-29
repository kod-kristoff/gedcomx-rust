use crate::{common::QnameUri, ser::SerializeXml};
use quick_xml::events::{BytesStart, Event};

/// Enumeration of standard relationship types.
#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub enum RelationshipType {
    AncestorDescendant,
    Couple,
    EnslavedBy,
    Godparent,
    ParentChild,
}

impl RelationshipType {
    pub fn as_qname_uri(&self) -> &str {
        match self {
            Self::AncestorDescendant => "http://gedcomx.org/AncestorDescendant",
            Self::Couple => "http://gedcomx.org/Couple",
            Self::EnslavedBy => "http://gedcomx.org/EnslavedBy",
            Self::Godparent => "http://gedcomx.org/Godparent",
            Self::ParentChild => "http://gedcomx.org/ParentChild",
        }
    }
}

impl serde::Serialize for RelationshipType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // use serde::ser::SerializeMap;
        // let mut map = serializer.serialize_map(Some(1))?;
        // map.serialize_entry("type", self.as_qname_uri())?;
        // map.end()
        serializer.serialize_str(self.as_qname_uri())
    }
}

impl SerializeXml for RelationshipType {
    fn tag(&self) -> &str {
        "gender"
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
