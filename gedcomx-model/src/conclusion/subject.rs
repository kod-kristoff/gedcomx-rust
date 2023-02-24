use crate::{
    common::{EvidenceReference, ResourceReference},
    conclusion::Conclusion,
    ser::{SerError, SerializeXml},
    source::SourceReference,
};

use serde::Deserializer;
use std::{borrow::Cow, io};

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::Deserialize;

    let s: Cow<String> = Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(
            s.as_str(),
            &["true", "false"],
        )),
    }
}
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Subject {
    #[serde(default, skip_serializing_if = "is_default")]
    extracted: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    evidence: Vec<EvidenceReference>,
    #[serde(flatten)]
    conclusion: Conclusion,
    // gender: Option<Gender>,
    // names: Vec<Name>,
    // facts: Vec<Fact>,
}

impl Subject {
    pub fn new() -> Self {
        Self {
            conclusion: Conclusion::default(),
            // id: String::new(),
            extracted: false,
            evidence: Vec::new(),
            // gender: None,
            // names: Vec::new(),
            // facts: Vec::new(),
        }
    }
}

impl Default for Subject {
    fn default() -> Self {
        Self::new()
    }
}

// Builder lite
impl Subject {
    pub fn extracted(mut self, yes: bool) -> Self {
        self.extracted = yes;
        self
    }

    // pub fn id<S: Into<String>>(mut self, id: S) -> Self {
    //     self.set_id(id.into());
    //     self
    // }
    pub fn evidence<S: Into<EvidenceReference>>(mut self, evidence: S) -> Self {
        self.add_evidence(evidence.into());
        self
    }
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

impl Subject {
    pub fn set_extracted(&mut self, yes: bool) {
        self.extracted = yes;
    }

    // pub fn set_id(&mut self, id: String) {
    //     self.id = id;
    // }
    pub fn set_analysis(&mut self, analysis: ResourceReference) {
        self.conclusion.set_analysis(analysis);
    }
    pub fn add_source(&mut self, source: SourceReference) {
        self.conclusion.add_source(source);
    }
    pub fn add_evidence(&mut self, evidence: EvidenceReference) {
        self.evidence.push(evidence);
    }
    pub fn extracted_as_str(&self) -> &'static str {
        bool_as_str(self.extracted)
    }
    pub fn is_extracted(&self) -> bool {
        self.extracted
    }
}
pub fn bool_as_str(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}
impl SerializeXml for Subject {
    fn tag(&self) -> &str {
        "subject"
    }

    fn serialize_xml<W: io::Write>(&self, ser: &mut quick_xml::Writer<W>) -> Result<(), SerError> {
        self.conclusion.serialize_xml(ser)?;

        for evidence in &self.evidence {
            evidence.serialize_xml(ser)?;
        }
        Ok(())
    }
}
