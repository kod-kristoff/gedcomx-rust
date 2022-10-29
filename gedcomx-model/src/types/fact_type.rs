use crate::ser::SerializeXml;
use quick_xml::events::{BytesStart, Event};

/// Enumeration of standard fact types.
#[derive(Debug, Clone, Copy, serde::Deserialize)] //, serde::Serialize)]
pub enum FactType {
    /// A fact of a person's birth.
    Birth,
    /// A fact of a person's occupation or employment.
    Occupation,
}

impl FactType {
    pub fn as_qname_uri(&self) -> &str {
        match self {
            Self::Birth => "http://gedcomx.org/Birth",
            Self::Occupation => "http://gedcomx.org/Occupation",
            // Self::Intersex => "http://gedcomx.org/Intersex",
            // Self::Unknown => "http://gedcomx.org/Unknown",
        }
    }
}

impl serde::Serialize for FactType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_qname_uri())
    }
}
impl SerializeXml for FactType {
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
