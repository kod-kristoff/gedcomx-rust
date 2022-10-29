use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

use crate::{ser::SerializeXml, types::NamePartType};

/// A name form conclusion
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NameForm {
    full_text: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    lang: String,
}

impl NameForm {
    pub fn new() -> Self {
        Self {
            full_text: String::new(),
            lang: String::new(),
        }
    }
}

// Builder lite
impl NameForm {
    pub fn full_text(mut self, full_text: String) -> Self {
        self.full_text = full_text;
        self
    }

    pub fn lang(mut self, lang: String) -> Self {
        self.lang = lang;
        self
    }
}

impl SerializeXml for NameForm {
    fn tag(&self) -> &str {
        "nameForm"
    }
    fn serialize_xml<W: std::io::Write>(
        &self,
        ser: &mut quick_xml::Writer<W>,
    ) -> Result<(), crate::ser::SerError> {
        let elem = BytesStart::new(self.tag());
        ser.write_event(Event::Start(elem))?;

        if !self.full_text.is_empty() {
            ser.write_event(Event::Start(BytesStart::new("fullText")))?;
            ser.write_event(Event::Text(BytesText::new(&self.full_text)))?;
            ser.write_event(Event::End(BytesEnd::new("fullText")))?;
        }
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
