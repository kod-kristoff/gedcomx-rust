use crate::ser::{SerError, SerializeXml};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use std::io;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Person {
    id: String,
    extracted: bool,
}

impl Person {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            extracted: false,
        }
    }
}

// Builder lite
impl Person {
    pub fn extracted(mut self, yes: bool) -> Self {
        self.extracted = yes;
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = id;
        self
    }
}

pub fn bool_as_str(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}
impl SerializeXml for Person {
    fn tag(&self) -> &str {
        "person"
    }

    fn serialize_xml<W: io::Write>(&self, ser: &mut quick_xml::Writer<W>) -> Result<(), SerError> {
        let mut root = BytesStart::new(self.tag());
        root.push_attribute(("extracted", bool_as_str(self.extracted)));
        root.push_attribute(("id", self.id.as_str()));
        ser.write_event(Event::Start(root))?;

        // for person in self.persons {
        //     person.serialize_xml(ser)?;
        // }
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}
