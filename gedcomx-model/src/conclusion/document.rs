use deserx::DeserializeXml;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

use crate::{
    common::{IriRef, ResourceReference},
    conclusion::NameForm,
    ser::{xml, SerializeXml},
    types::NamePartType,
};

/// An abstract document that contains derived (conclusionary) text -- for example, a transcription or researcher analysis.

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Document {
    text: String,
    id: IriRef,
}

impl Document {
    pub fn new(id: IriRef) -> Self {
        Self {
            id,
            text: String::new(),
        }
    }
}

// Builder lite
impl Document {
    // pub fn id<S: Into<String>>(mut self, id: S) -> Self {
    //     self.set_id(id.into());
    //     self
    // }
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.set_text(text.into());
        self
    }
}

impl Document {
    // pub fn set_id(&mut self, id: String) {
    //     self.id = id;
    // }
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

impl DeserializeXml for Document {
    fn deserialize_xml_with_start<'de, R: std::io::BufRead>(
        deserializer: &mut quick_xml::Reader<R>,
        start: &quick_xml::events::BytesStart<'de>,
    ) -> Result<Self, quick_xml::Error> {
        let mut buf = Vec::new();

        let id = if let Some(id) = start.try_get_attribute("id")? {
            IriRef::parse(id.unescape_value()?.into()).expect("iri")
        } else {
            todo!("handle no id")
        };
        let mut document = Self::new(id);
        loop {
            match deserializer.read_event_into(&mut buf)? {
                Event::End(e) => {
                    log::trace!("got End={:?}", e);
                    match e.name().as_ref() {
                        b"document" => {
                            log::trace!("found end of 'document', returning ...");
                            break;
                        }
                        _tag => log::trace!("found End={:?}, skipping ...", e),
                    }
                }
                Event::Start(e) => {
                    log::trace!("got Start={:?}", e);
                    match e.name().as_ref() {
                        b"text" => {
                            log::trace!("found 'text', reading ...");
                            if let Event::Text(text) = deserializer.read_event_into(&mut buf)? {
                                document.set_text(text.unescape()?.into());
                            }
                        }
                        _tag => todo!("handle Start={:?}", e),
                    }
                }
                Event::Text(e) => log::trace!("got Text={:?}, skipping ...", e),
                e => {
                    log::trace!("got {:?}", e);
                    todo!("handle {:?}", e)
                }
            }
        }
        log::debug!("document={:?}", document);
        Ok(document)
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DocumentReference(ResourceReference);

impl DocumentReference {
    pub fn new(reference: IriRef) -> Self {
        Self(ResourceReference::new(reference))
    }
}

impl From<&Document> for DocumentReference {
    fn from(doc: &Document) -> Self {
        DocumentReference::new(doc.id.clone())
    }
}

impl From<DocumentReference> for ResourceReference {
    fn from(doc_ref: DocumentReference) -> Self {
        doc_ref.0
    }
}
