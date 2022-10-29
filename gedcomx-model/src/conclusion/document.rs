use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

use crate::{
    common::ResourceReference,
    conclusion::NameForm,
    ser::{xml, SerializeXml},
    types::NamePartType,
};

/// An abstract document that contains derived (conclusionary) text -- for example, a transcription or researcher analysis.

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Document {
    text: String,
    id: String,
}

impl Document {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            text: String::new(),
        }
    }
}

// Builder lite
impl Document {
    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.set_id(id.into());
        self
    }
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.set_text(text.into());
        self
    }
}

impl Document {
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

// impl From<&str> for Document {
//     fn from(s: &str) -> Self {
//         Self::new().name_form(NameForm::new().full_text(s.into()))
//     }
// }

impl SerializeXml for Document {
    fn tag(&self) -> &str {
        "document"
    }
    fn serialize_xml<W: std::io::Write>(
        &self,
        ser: &mut quick_xml::Writer<W>,
    ) -> Result<(), crate::ser::SerError> {
        let mut elem = BytesStart::new(self.tag());
        elem.push_attribute(("id", self.id.as_str()));
        ser.write_event(Event::Start(elem))?;

        ser.write_event(Event::Start(BytesStart::new("text")))?;
        ser.write_event(Event::Text(BytesText::from_escaped(self.text.as_str())))?;
        ser.write_event(Event::End(BytesEnd::new("text")))?;
        // xml::write_elem_w_text(ser, "text", self.text.as_str())?;

        // for name_form in &self.name_forms {
        //     name_form.serialize_xml(ser)?;
        // }
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DocumentReference(ResourceReference);

impl From<&Document> for DocumentReference {
    fn from(doc: &Document) -> Self {
        Self(ResourceReference::with_resource(format!("#{}", doc.id)))
    }
}

impl From<DocumentReference> for ResourceReference {
    fn from(doc_ref: DocumentReference) -> Self {
        doc_ref.0
    }
}
