use crate::agent::Agent;
use crate::conclusion::{Document, Person, Relationship};
use crate::gedcomx::attribution::verify_attribution_opt;
use crate::gedcomx::Attribution;
use crate::ser::{SerError, SerializeXml};
use crate::source::SourceDescription;
use deserx::DeserializeXml;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use std::io;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
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
        self.add_person(p);
        self
    }

    pub fn add_person(&mut self, p: Person) {
        self.persons.push(p);
    }

    pub fn attribution(mut self, attribution: Attribution) -> Self {
        self.set_attribution(attribution);
        self
    }

    pub fn set_attribution(&mut self, attribution: Attribution) {
        self.attribution = Some(attribution);
    }

    pub fn relationship(mut self, relationship: Relationship) -> Self {
        self.add_relationship(relationship);
        self
    }

    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.push(relationship);
    }

    pub fn source_description(mut self, source_description: SourceDescription) -> Self {
        self.add_source_description(source_description);
        self
    }

    pub fn add_source_description(&mut self, source_description: SourceDescription) {
        self.source_descriptions.push(source_description);
    }

    pub fn agent(mut self, p: Agent) -> Self {
        self.add_agent(p);
        self
    }

    pub fn add_agent(&mut self, p: Agent) {
        self.agents.push(p);
    }

    pub fn document(mut self, p: Document) -> Self {
        self.add_document(p);
        self
    }

    pub fn add_document(&mut self, p: Document) {
        self.documents.push(p);
    }
}

impl GedcomX {
    pub fn persons(&self) -> &[Person] {
        self.persons.as_slice()
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

impl DeserializeXml for GedcomX {
    fn deserialize_xml<R: io::BufRead>(
        deserializer: &mut quick_xml::Reader<R>,
    ) -> Result<Self, quick_xml::Error> {
        let mut buf = Vec::new();
        loop {
            match deserializer.read_event_into(&mut buf)? {
                Event::Start(e) => {
                    log::trace!("read start={:?}", e);
                    match e.name().as_ref() {
                        b"gedcomx" => break,
                        e => {
                            return Err(quick_xml::Error::UnexpectedToken(format!(
                                "got tag: '{}', expected: 'gedcomx'",
                                String::from_utf8_lossy(e)
                            )))
                        }
                    }
                }
                e => log::trace!("skipping {:?}", e),
            };
        }
        log::trace!("buf = {:?}", String::from_utf8_lossy(&buf));
        let mut gedcomx = Self::new();

        loop {
            match deserializer.read_event_into(&mut buf)? {
                Event::Start(e) => {
                    log::trace!("read start={:?}", e);
                    match e.name().as_ref() {
                        b"agent" => {
                            let agent = Agent::deserialize_xml_with_start(deserializer, &e)?;
                            gedcomx.add_agent(agent);
                        }
                        b"attribution" => {
                            let attribution = Attribution::deserialize_xml(deserializer)?;
                            gedcomx.set_attribution(attribution);
                        }
                        b"document" => {
                            let document = Document::deserialize_xml_with_start(deserializer, &e)?;
                            gedcomx.add_document(document);
                        }
                        b"person" => {
                            let person = Person::deserialize_xml_with_start(deserializer, &e)?;
                            gedcomx.add_person(person);
                        }
                        b"relationship" => {
                            let relationship = Relationship::deserialize_xml_with_start(deserializer, &e)?;
                            gedcomx.add_relationship(relationship);
                        }
                        b"sourceDescription" => {
                            let source_description = SourceDescription::deserialize_xml_with_start(deserializer, &e)?;
                            gedcomx.add_source_description(source_description);
                        }
                        e => {
                            return Err(quick_xml::Error::UnexpectedToken(format!(
                                "got tag: '{}', expected one of: ['attribution', 'person', 'relationship', 'sourceDescription', 'agent', 'document']'",
                                String::from_utf8_lossy(e)
                            )))
                        }
                    }
                }
                Event::End(e) => {
                    log::trace!("read End={:?}", e);
                    match e.name().as_ref() {
                        b"gedcomx" => {
                            log::debug!("found '</gedcomx>' returning ..");
                            break;
                        }
                        _tag => log::trace!("skipping '{:?}' ...", e),
                    }
                }
                e => log::trace!("skipping {:?}", e),
            };
        }
        log::trace!("buf = {:?}", String::from_utf8_lossy(&buf));
        log::trace!("{:#?}", gedcomx);
        Ok(gedcomx)
    }
}

pub fn verify_gedcomx(a: &GedcomX, b: &GedcomX) -> Result<(), String> {
    verify_attribution_opt(a.attribution.as_ref(), b.attribution.as_ref())?;
    assert_eq!(a.agents, b.agents);
    assert_eq!(a.documents, b.documents);
    for (a_person, b_person) in a.persons.iter().zip(b.persons.iter()) {
        assert_eq!(a_person, b_person);
    }
    assert_eq!(a.relationships, b.relationships);
    assert_eq!(a.source_descriptions, b.source_descriptions);
    // verify_agents(&a.agents, &b.agents)?;
    // verify_relationships(&a.relationships, &b.relationships)?;
    // verify_source_descriptions(&a.source_descriptions, &b.source_descriptions)?;
    // verify_persons(&a.persons, &b.persons)?;
    // verify_documents(&a.documents, &b.documents)?;
    Ok(())
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

    // #[test]
    // fn serialize_empty_as_json() -> Result<(), Box<dyn std::error::Error>> {
    //     let gedcomx = GedcomX::new();

    //     let result = serde_json::to_string(&gedcomx)?;
    //     let expected = r#"{"attribution":null,"persons":[],"relationships":[],"sourceDescriptions":[],"agents":[],"documents":[]}"#;
    //     assert_eq!(result, expected);
    //     Ok(())
    // }
}
