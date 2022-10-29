use crate::common::{DateTime, ResourceReference};
use crate::ser::{SerError, SerializeXml, XmlSerializer};
use chrono::Utc;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use serde_with::TimestampMilliSeconds;
use std::io;

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Attribution {
    contributor: ResourceReference,
    #[serde_as(as = "TimestampMilliSeconds")]
    modified: DateTime,
}

impl Attribution {
    pub fn new() -> Self {
        Self {
            contributor: ResourceReference::new(),
            modified: Utc::now(),
        }
    }
}

impl Attribution {
    pub fn contributor<C: Into<ResourceReference>>(mut self, contributor: C) -> Self {
        self.contributor = contributor.into();
        self
    }

    pub fn modified(mut self, modified: DateTime) -> Self {
        self.modified = modified;
        self
    }
}

impl SerializeXml for Attribution {
    fn tag(&self) -> &str {
        "attribution"
    }
    fn serialize_xml<W: io::Write>(&self, ser: &mut XmlSerializer<W>) -> Result<(), SerError> {
        let elem = BytesStart::new(self.tag());
        ser.write_event(Event::Start(elem))?;

        let mut contributor = BytesStart::new("contributor");
        contributor.push_attribute(("resource", self.contributor.resource()));
        ser.write_event(Event::Empty(contributor))?;

        ser.write_event(Event::Start(BytesStart::new("modified")))?;
        ser.write_event(Event::Text(BytesText::new(&self.modified.to_rfc3339())))?;
        ser.write_event(Event::End(BytesEnd::new("modified")))?;
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}
