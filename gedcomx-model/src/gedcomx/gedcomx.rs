use crate::agent::Agent;
use crate::conclusion::{Document, Person, Relationship};
use crate::gedcomx::Attribution;
use crate::ser::{SerError, SerializeXml};
use crate::source::SourceDescription;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use std::io;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "gedcomx", rename_all = "camelCase")]
pub struct GedcomX {
    attribution: Option<Attribution>,
    persons: Vec<Person>,
    relationships: Vec<Relationship>,
    source_descriptions: Vec<SourceDescription>,
    agents: Vec<Agent>,
    documents: Vec<Document>,
}

impl GedcomX {
    pub fn new() -> Self {
        Self {
            attribution: None,
            persons: Vec::new(),
            relationships: Vec::new(),
            source_descriptions: Vec::new(),
            agents: Vec::new(),
            documents: Vec::new(),
        }
    }
}

impl GedcomX {
    pub fn person(mut self, p: Person) -> Self {
        self.persons.push(p);
        self
    }

    pub fn attribution(mut self, attribution: Attribution) -> Self {
        self.attribution = Some(attribution);
        self
    }

    pub fn relationship(mut self, relationship: Relationship) -> Self {
        self.relationships.push(relationship);
        self
    }

    pub fn source_description(mut self, source_description: SourceDescription) -> Self {
        self.source_descriptions.push(source_description);
        self
    }
    pub fn agent(mut self, p: Agent) -> Self {
        self.agents.push(p);
        self
    }
    pub fn document(mut self, p: Document) -> Self {
        self.documents.push(p);
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

        if let Some(attribution) = &self.attribution {
            attribution.serialize_xml(ser)?;
        }
        for person in &self.persons {
            person.serialize_xml(ser)?;
        }
        for relationship in &self.relationships {
            relationship.serialize_xml(ser)?;
        }
        for source_description in &self.source_descriptions {
            source_description.serialize_xml(ser)?;
        }
        for agent in &self.agents {
            agent.serialize_xml(ser)?;
        }
        for document in &self.documents {
            document.serialize_xml(ser)?;
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
        let expected = r#"{"attribution":null,"persons":[],"relationships":[],"sourceDescriptions":[],"agents":[],"documents":[]}"#;
        assert_eq!(result, expected);
        Ok(())
    }
}
