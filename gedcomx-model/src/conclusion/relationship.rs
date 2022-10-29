use crate::{
    common::{EvidenceReference, ResourceReference},
    conclusion::{Fact, Name},
    ser::{xml, SerError, SerializeXml},
    types::{Gender, RelationshipType},
};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use std::io;

use super::{DocumentReference, Subject};

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Relationship {
    id: Option<String>,
    r#type: RelationshipType,
    #[serde(flatten)]
    subject: Subject,
    person1: Option<ResourceReference>,
    person2: Option<ResourceReference>,
    source: Option<ResourceReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    facts: Vec<Fact>,
}

impl Relationship {
    pub fn new(r#type: RelationshipType) -> Self {
        Self {
            id: None,
            r#type,
            subject: Subject::default(),
            person1: None,
            person2: None,
            source: None,
            facts: Vec::new(),
        }
    }
}

// Builder lite
impl Relationship {
    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.set_id(id.into());
        self
    }
    pub fn extracted(mut self, yes: bool) -> Self {
        self.subject = self.subject.extracted(yes);
        self
    }

    pub fn source<S: Into<ResourceReference>>(mut self, source: S) -> Self {
        self.source = Some(source.into());
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
    pub fn person1<R: Into<ResourceReference>>(mut self, person1: R) -> Self {
        self.set_person1(person1.into());
        self
    }

    pub fn person2<R: Into<ResourceReference>>(mut self, person2: R) -> Self {
        self.set_person2(person2.into());
        self
    }

    pub fn fact(mut self, fact: Fact) -> Self {
        self.facts.push(fact);
        self
    }
}

impl Relationship {
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
    pub fn set_person1(&mut self, person1: ResourceReference) {
        self.person1 = Some(person1);
    }
    pub fn set_person2(&mut self, person2: ResourceReference) {
        self.person2 = Some(person2);
    }
    pub fn is_extracted(&self) -> bool {
        self.subject.is_extracted()
    }
    pub fn set_analysis(&mut self, analysis: DocumentReference) {
        self.subject.set_analysis(analysis.into());
    }
}

// impl From<&Relationship> for EvidenceReference {
//     fn from(p: &Relationship) -> Self {
//         EvidenceReference::with_resource(format!("#{}", p.id))
//     }
// }
impl SerializeXml for Relationship {
    fn tag(&self) -> &str {
        "relationship"
    }

    fn serialize_xml<W: io::Write>(&self, ser: &mut quick_xml::Writer<W>) -> Result<(), SerError> {
        let mut root = BytesStart::new(self.tag());
        if self.is_extracted() {
            root.push_attribute(("extracted", self.subject.extracted_as_str()));
        }
        root.push_attribute(("type", self.r#type.as_qname_uri()));
        ser.write_event(Event::Start(root))?;

        if let Some(person1) = self.person1.as_ref() {
            xml::write_elem_w_attribute(ser, "person1", ("resource", person1.resource()))?;
        }
        if let Some(person2) = self.person2.as_ref() {
            xml::write_elem_w_attribute(ser, "person2", ("resource", person2.resource()))?;
        }
        self.subject.serialize_xml(ser)?;

        if let Some(source) = self.source.as_ref() {
            let mut source_elem = BytesStart::new("source");
            source_elem.push_attribute(("description", source.resource()));
            ser.write_event(Event::Empty(source_elem))?;
        }

        for fact in &self.facts {
            fact.serialize_xml(ser)?;
        }
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}
