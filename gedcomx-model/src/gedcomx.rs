use crate::conclusion::Person;
use crate::ser::{SerError, SerializeXml};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use std::io;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "gedcomx")]
pub struct GedcomX {
    persons: Vec<Person>,
}

impl GedcomX {
    pub fn new() -> Self {
        Self {
            persons: Vec::new(),
        }
    }
}

impl GedcomX {
    pub fn person(mut self, p: Person) -> Self {
        self.persons.push(p);
        self
    }
}

impl SerializeXml for GedcomX {
    fn tag(&self) -> &str {
        "gedcomx"
    }

    fn serialize_xml<W: io::Write>(&self, ser: &mut quick_xml::Writer<W>) -> Result<(), SerError> {
        let mut root = BytesStart::new(self.tag());
        root.push_attribute(("xmlns", "http://gedcomx.org/v1/"));
        ser.write_event(Event::Start(root))?;

        for person in &self.persons {
            person.serialize_xml(ser)?;
        }
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::error::Error;

    // fn gedcomx_record() -> GedcomX {
    //     GedcomX::new().person(Person::new())
    // }

    // #[test]
    // fn serialize_gedcomx_record_as_xml() -> Result<(), Box<dyn Error>> {
    //     let gedcomx = gedcomx_record();

    //     let mut buffer = Vec::new();
    //     let mut ser = quick_xml::se::Serializer::new(&mut buffer);
    //     // let result = quick_xml::se::to_string(&gedcomx)?;
    //     gedcomx.serialize(&mut ser)?;
    //     assert_eq!(String::from_utf8(buffer)?, "<gedcomx/>");
    //     Ok(())
    // }

    #[test]
    fn serialize_empty_as_xml() -> Result<(), Box<dyn std::error::Error>> {
        let gedcomx = GedcomX::new();

        let result = quick_xml::se::to_string(&gedcomx)?;
        println!("gedcomx={}", result);
        assert_eq!(result, "<gedcomx/>");
        Ok(())
    }

    #[test]
    fn serialize_empty_as_json() -> Result<(), Box<dyn std::error::Error>> {
        let gedcomx = GedcomX::new();

        let result = serde_json::to_string(&gedcomx)?;
        let expected = r#"{"persons":[]}"#;
        assert_eq!(result, expected);
        Ok(())
    }
}
