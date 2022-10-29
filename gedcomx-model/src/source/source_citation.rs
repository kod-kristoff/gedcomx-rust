use crate::ser::{xml, SerError, SerializeXml};

use quick_xml::events::{BytesEnd, BytesStart, Event};

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SourceCitation {
    value: String,
    lang: Option<String>,
}

impl SourceCitation {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            lang: None,
        }
    }
}

impl SourceCitation {
    pub fn value<S: Into<String>>(mut self, value: S) -> Self {
        self.set_value(value.into());
        self
    }
}

impl SourceCitation {
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

impl<S: Into<String>> From<S> for SourceCitation {
    fn from(value: S) -> Self {
        Self {
            value: value.into(),
            lang: None,
        }
    }
}
impl SerializeXml for SourceCitation {
    fn tag(&self) -> &str {
        "citation"
    }

    fn serialize_xml<W: std::io::Write>(
        &self,
        ser: &mut quick_xml::Writer<W>,
    ) -> Result<(), SerError> {
        let elem = BytesStart::new(self.tag());
        ser.write_event(Event::Start(elem))?;

        xml::write_elem_w_text(ser, "value", self.value.as_str())?;
        // for citation in &self.citations {
        //     citation.serialize_xml(ser)?;
        // }
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}
