use crate::{
    common::{EvidenceReference, ResourceReference},
    conclusion::{Fact, Name},
    ser::{SerError, SerializeXml},
    source::SourceReference,
    types::Gender,
};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use std::io;

use super::{DocumentReference, Subject};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Person {
    #[serde(flatten)]
    subject: Subject,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    names: Vec<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gender: Option<Gender>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    facts: Vec<Fact>,
    id: String,
}

impl Person {
    pub fn new(id: String) -> Self {
        Self {
            id,
            subject: Subject::default(),
            gender: None,
            names: Vec::new(),
            facts: Vec::new(),
        }
    }
    pub fn with_id<S: Into<String>>(id: S) -> Self {
        Self::new(id.into())
    }
}

// Builder lite
impl Person {
    pub fn extracted(mut self, yes: bool) -> Self {
        self.subject = self.subject.extracted(yes);
        self
    }

    pub fn source<S: Into<SourceReference>>(mut self, source: S) -> Self {
        self.add_source(source.into());
        self
    }
    pub fn analysis<S: Into<DocumentReference>>(mut self, analysis: S) -> Self {
        self.set_analysis(analysis.into());
        self
    }
    pub fn evidence<S: Into<EvidenceReference>>(mut self, evidence: S) -> Self {
        self.subject.add_evidence(evidence.into());
        self
    }
    pub fn gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }

    pub fn name<N: Into<Name>>(mut self, name: N) -> Self {
        self.add_name(name.into());
        self
    }

    pub fn fact(mut self, fact: Fact) -> Self {
        self.facts.push(fact);
        self
    }
}

impl Person {
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
    pub fn add_source(&mut self, source: SourceReference) {
        self.subject.add_source(source);
    }
    pub fn add_name(&mut self, name: Name) {
        self.names.push(name);
    }
    pub fn is_extracted(&self) -> bool {
        self.subject.is_extracted()
    }
    pub fn set_analysis(&mut self, analysis: DocumentReference) {
        self.subject.set_analysis(analysis.into());
    }
}

impl From<&Person> for EvidenceReference {
    fn from(p: &Person) -> Self {
        EvidenceReference::with_resource(format!("#{}", p.id))
    }
}
impl From<&Person> for ResourceReference {
    fn from(p: &Person) -> Self {
        ResourceReference::with_resource(format!("#{}", p.id))
    }
}
impl SerializeXml for Person {
    fn tag(&self) -> &str {
        "person"
    }

    fn serialize_xml<W: io::Write>(&self, ser: &mut quick_xml::Writer<W>) -> Result<(), SerError> {
        let mut root = BytesStart::new(self.tag());
        if self.is_extracted() {
            root.push_attribute(("extracted", self.subject.extracted_as_str()));
        }
        root.push_attribute(("id", self.id.as_str()));
        ser.write_event(Event::Start(root))?;

        self.subject.serialize_xml(ser)?;

        if let Some(gender) = self.gender.as_ref() {
            gender.serialize_xml(ser)?;
        }
        for name in &self.names {
            name.serialize_xml(ser)?;
        }
        for fact in &self.facts {
            fact.serialize_xml(ser)?;
        }
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}
