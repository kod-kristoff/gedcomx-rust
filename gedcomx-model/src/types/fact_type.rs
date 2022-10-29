use core::fmt;

use crate::ser::SerializeXml;
use quick_xml::events::{BytesStart, Event};

/// Enumeration of standard fact types.
#[derive(Debug, Clone, Copy)] //, serde::Deserialize)] //, serde::Serialize)]
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
    pub fn from_qname_uri(qname_uri: &str) -> Self {
        match qname_uri {
            "http://gedcomx.org/Birth" => Self::Birth,
            "http://gedcomx.org/Occupation" => Self::Occupation,
            // Self::Intersex => "http://gedcomx.org/Intersex",
            // Self::Unknown => "http://gedcomx.org/Unknown",
            _ => todo!("handle qname_uri='{}'", qname_uri),
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

impl<'de> serde::Deserialize<'de> for FactType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(FactTypeVisitor)
    }
}

struct FactTypeVisitor;

impl<'de> serde::de::Visitor<'de> for FactTypeVisitor {
    type Value = FactType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a uri")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(FactType::from_qname_uri(value))
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
