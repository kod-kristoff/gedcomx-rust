use quick_xml::events::{BytesStart, Event};

use crate::ser::SerializeXml;

/// Enumeration of standard name types.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NameType {
    /// Name given at birth.
    BirthName,

    /// Name used at the time of death.
    DeathName,

    /// Name accepted at marriage.
    MarriedName,

    /// "Also known as" name.
    AlsoKnownAs,

    /// Nickname.
    Nickname,

    /// Name given at adoption.
    AdoptiveName,

    /// A formal name, usually given to distinguish it from a name more commonly used.
    FormalName,

    /// A name given at a religious rite or ceremony.
    ReligiousName,
    // OTHER(String),
    Other,
}

const QNAME_MAP: [(NameType, &str); 8] = [
    (NameType::BirthName, "http://gedcomx.org/BirthName"),
    (NameType::DeathName, "http://gedcomx.org/DeathName"),
    (NameType::MarriedName, "http://gedcomx.org/MarriedName"),
    (NameType::AlsoKnownAs, "http://gedcomx.org/AlsoKnownAs"),
    (NameType::Nickname, "http://gedcomx.org/Nickname"),
    (NameType::AdoptiveName, "http://gedcomx.org/AdoptiveName"),
    (NameType::FormalName, "http://gedcomx.org/FormalName"),
    (NameType::ReligiousName, "http://gedcomx.org/ReligiousName"),
];
impl NameType {
    pub fn as_qname_uri(&self) -> &str {
        for (relationship, qname) in QNAME_MAP {
            if self == &relationship {
                return qname;
            }
        }
        "OTHER"
    }
    pub fn from_qname_uri(qname_uri: &str) -> Self {
        for (relationship, qname) in QNAME_MAP {
            if qname == qname_uri {
                return relationship;
            }
        }
        Self::Other
    }
}

impl serde::Serialize for NameType {
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

impl<'de> serde::Deserialize<'de> for NameType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(NameTypeVisitor)
    }
}

struct NameTypeVisitor;

impl<'de> serde::de::Visitor<'de> for NameTypeVisitor {
    type Value = NameType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an uri")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NameType::from_qname_uri(v))
    }
}
impl SerializeXml for NameType {
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
