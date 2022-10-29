use crate::{
    common::{EvidenceReference, ResourceReference},
    conclusion::{Fact, Name},
    ser::{xml, SerError, SerializeXml},
    source::SourceReference,
    types::Gender,
};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use std::io;

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Conclusion {
    analysis: Option<ResourceReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    sources: Vec<SourceReference>,
    // gender: Option<Gender>,
    // names: Vec<Name>,
    // facts: Vec<Fact>,
}

impl Conclusion {
    pub fn new() -> Self {
        Self {
            analysis: None,
            sources: Vec::new(),
            // id: String::new(),
            // gender: None,
            // names: Vec::new(),
            // facts: Vec::new(),
        }
    }
}

impl Default for Conclusion {
    fn default() -> Self {
        Self::new()
    }
}

// Builder lite
impl Conclusion {
    // pub fn extracted(mut self, yes: bool) -> Self {
    //     self.extracted = yes;
    //     self
    // }

    // pub fn id<S: Into<String>>(mut self, id: S) -> Self {
    //     self.set_id(id.into());
    //     self
    // }
    // pub fn <S: Into<EvidenceReference>>(mut self, evidence: S) -> Self {
    //     self.add_evidence(evidence.into());
    //     self
    // }
    // pub fn gender(mut self, gender: Gender) -> Self {
    //     self.gender = Some(gender);
    //     self
    // }

    // pub fn name<N: Into<Name>>(mut self, name: N) -> Self {
    //     self.add_name(name.into());
    //     self
    // }

    // pub fn fact(mut self, fact: Fact) -> Self {
    //     self.facts.push(fact);
    //     self
    // }
}

impl Conclusion {
    // pub fn set_id(&mut self, id: String) {
    //     self.id = id;
    // }
    pub fn set_analysis(&mut self, analysis: ResourceReference) {
        self.analysis = Some(analysis);
    }
    pub fn add_source(&mut self, source: SourceReference) {
        self.sources.push(source);
    }
}
impl SerializeXml for Conclusion {
    fn tag(&self) -> &str {
        "conclusion"
    }

    fn serialize_xml<W: io::Write>(&self, ser: &mut quick_xml::Writer<W>) -> Result<(), SerError> {
        for source in &self.sources {
            source.serialize_xml(ser)?;
        }
        if let Some(analysis) = self.analysis.as_ref() {
            xml::write_elem_w_attribute(ser, "analysis", ("resource", analysis.resource()))?;
        }
        // for evidence in &self.e3xvidence {
        //     evidence.serialize_xml(ser)?;
        // }
        Ok(())
    }
}
