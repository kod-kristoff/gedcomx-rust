use crate::{
    common::{EvidenceReference, IriRef, ResourceReference, Uri},
    conclusion::{Fact, Name},
    ser::{SerError, SerializeXml},
    source::SourceReference,
    types::Gender,
    Result,
};
use deserx::DeserializeXml;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use std::io;

use super::{DocumentReference, Subject};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Person {
    #[serde(flatten)]
    subject: Subject,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    names: Vec<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gender: Option<Gender>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    facts: Vec<Fact>,
    id: IriRef,
}

impl Person {
    pub fn new(id: IriRef) -> Self {
        Self {
            id,
            subject: Subject::default(),
            gender: None,
            names: Vec::new(),
            facts: Vec::new(),
        }
    }
    pub fn with_id<S: Into<String>>(id: S) -> Result<Self> {
        Ok(Self::new(IriRef::parse(id.into())?))
    }
}

// Builder lite
impl Person {
    pub fn extracted(mut self, yes: bool) -> Self {
        self.set_extracted(yes);
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
        self.set_gender(gender);
        self
    }

    pub fn name<N: Into<Name>>(mut self, name: N) -> Self {
        self.add_name(name.into());
        self
    }

    pub fn fact(mut self, fact: Fact) -> Self {
        self.add_fact(fact);
        self
    }
}

impl Person {
    // pub fn set_id(&mut self, id: String) {
    //     self.id = id;
    // }
    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = Some(gender);
    }

    pub fn set_extracted(&mut self, yes: bool) {
        self.subject.set_extracted(yes);
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
    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }

    pub fn facts(&self) -> &[Fact] {
        self.facts.as_slice()
    }

    pub fn names(&self) -> &[Name] {
        self.names.as_slice()
    }
    pub fn id(&self) -> &IriRef {
        &self.id
    }
}

impl From<&Person> for EvidenceReference {
    fn from(p: &Person) -> Self {
        EvidenceReference::new(p.id.clone())
    }
}
impl From<&Person> for ResourceReference {
    fn from(p: &Person) -> Self {
        ResourceReference::new(p.id.clone())
    }
}
impl SerializeXml for Person {
    fn tag(&self) -> &str {
        "person"
    }

    fn serialize_xml<W: io::Write>(
        &self,
        ser: &mut quick_xml::Writer<W>,
    ) -> std::result::Result<(), SerError> {
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

impl DeserializeXml for Person {
    fn deserialize_xml_with_start<'de, R: std::io::BufRead>(
        deserializer: &mut quick_xml::Reader<R>,
        start: &quick_xml::events::BytesStart<'de>,
    ) -> std::result::Result<Self, quick_xml::Error> {
        //     <R: std::io::BufRead>(
        //     deserializer: &mut quick_xml::Reader<R>,
        // ) -> Result<Self, quick_xml::Error> {
        let mut buf = Vec::new();
        let attr = start.try_get_attribute("id")?;
        let id: String = if let Some(id) = attr {
            id.unescape_value()?.into()
            // person.set_contributor(ResourceReference::with_resource(
            //     resource.unescape_value()?.into(),
            // ));
        } else {
            todo!("handle no 'id'")
        };
        let mut person = Self::new(IriRef::parse(id).expect("iri"));
        let attr = start.try_get_attribute("extracted")?;
        let extracted = if let Some(extracted) = attr {
            match extracted.unescape_value()?.as_ref() {
                "true" | "1" => true,
                _ => false,
            }
        } else {
            false
        };
        person.set_extracted(extracted);
        loop {
            match deserializer.read_event_into(&mut buf)? {
                Event::Empty(e) => {
                    log::debug!("read Empty={:?}", e);
                    match e.name().as_ref() {
                        b"analysis" => {
                            let attr = e.try_get_attribute("resource")?;
                            if let Some(value) = attr {
                                person.set_analysis(DocumentReference::new(
                                    IriRef::parse(value.unescape_value()?.into()).expect("iri"),
                                ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"evidence" => {
                            let attr = e.try_get_attribute("resource")?;
                            if let Some(value) = attr {
                                person.subject.add_evidence(EvidenceReference::new(
                                    IriRef::parse(value.unescape_value()?.into()).expect("iri"),
                                ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"gender" => {
                            let attr = e.try_get_attribute("type")?;
                            if let Some(value) = attr {
                                person.set_gender(Gender::from_qname_uri(
                                    value.unescape_value()?.as_ref(),
                                ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"source" => {
                            let attr = e.try_get_attribute("description")?;
                            if let Some(source) = attr {
                                person.add_source(SourceReference::new(
                                    IriRef::parse(source.unescape_value()?.to_string())
                                        .expect("iri"),
                                    String::new(),
                                ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        _tag => todo!("handle {:?}", e),
                    }
                }
                Event::Start(e) => {
                    log::debug!("read Start={:?}", e);
                    match e.name().as_ref() {
                        b"fact" => {
                            log::trace!("found 'fact'");
                            let fact = Fact::deserialize_xml_with_start(deserializer, &e)?;
                            person.add_fact(fact);
                        }
                        b"name" => {
                            log::trace!("found 'name'");
                            let name = Name::deserialize_xml_with_start(deserializer, &e)?;
                            person.add_name(name);
                        }
                        _tag => todo!("handle {:?}", e),
                    }
                }
                Event::End(e) => match e.name().as_ref() {
                    b"person" => {
                        log::trace!("found end of 'person' returning ...");
                        break;
                    }
                    _tag => log::trace!("skipping '{:?}' ...", e),
                },
                e => {
                    log::trace!("got: {:?} skipping ...", e);
                }
            }
        }
        log::debug!("person = {:?}", person);
        Ok(person)
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn deserialize_extracted_from_xml() -> Result<(), Box<dyn std::error::Error>> {
//         let xml = r##"
//     <person extracted="true" id="P-1">
//         <source description="#S-1"/>
//         <gender type="http://gedcomx.org/Female"/>
//         <name>
//             <nameForm>
//                 <fullText>Emma Bocock</fullText>
//             </nameForm>
//         </name>
//         <fact type="http://gedcomx.org/Birth">
//             <date>
//                 <original>23 June 1843</original>
//             </date>
//             <place>
//                 <original>Broadfield Bar, Abbeydale Road, Ecclesall-Bierlow, York, England, United Kingdom</original>
//             </place>
//         </fact>
//     </person>

//         "##;
//         let _person: Person = quick_xml::de::from_str(xml)?;
//         Ok(())
//     }
//     #[test]
//     fn deserialize_derived_from_xml() -> Result<(), Box<dyn std::error::Error>> {
//         let xml = r##"
//     <person id="C-1">
//         <analysis resource="#D-1"/>
//         <evidence resource="#P-1"/>
//     </person>
//         "##;
//         let _person: Person = quick_xml::de::from_str(xml)?;
//         Ok(())
//     }
// }
