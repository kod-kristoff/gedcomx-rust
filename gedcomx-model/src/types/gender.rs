use crate::ser::SerializeXml;
use quick_xml::events::{BytesStart, Event};

/// Enumeration of known gender types.
#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Gender {
    /// Male.
    Male,
    /// Female.
    #[serde(rename = "http://gedcomx.org/Female")]
    Female,
    /// Intersex.
    Intersex,
    /// Unknown. Note that this should be used strictly as "unknown" and not to
    /// indicate a type that is not set or not understood.
    Unknown,
}

impl Gender {
    fn as_qname_uri(&self) -> &str {
        match self {
            Self::Female => "http://gedcomx.org/Female",
            Self::Male => "http://gedcomx.org/Male",
            Self::Intersex => "http://gedcomx.org/Intersex",
            Self::Unknown => "http://gedcomx.org/Unknown",
        }
    }
}

impl serde::Serialize for Gender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("type", self.as_qname_uri())?;
        map.end()
    }
}
impl SerializeXml for Gender {
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
