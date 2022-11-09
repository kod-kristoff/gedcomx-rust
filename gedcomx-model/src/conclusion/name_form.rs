use deserx::DeserializeXml;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

use crate::{ser::SerializeXml, types::NamePartType};

/// A name form conclusion
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
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

impl DeserializeXml for NameForm {
    fn deserialize_xml_with_start<'de, R: std::io::BufRead>(
        deserializer: &mut quick_xml::Reader<R>,
        start: &quick_xml::events::BytesStart<'de>,
    ) -> Result<Self, quick_xml::Error> {
        let mut buf = Vec::new();
        // let attr = start.try_get_attribute("id")?;
        // let id: String = if let Some(id) = attr {
        //     id.unescape_value()?.into()
        // } else {
        //     todo!("handle no 'id'")
        // };
        let mut name_form = Self::new();
        // let attr = start.try_get_attribute("extracted")?;
        // let extracted = if let Some(extracted) = attr {
        //     match extracted.unescape_value()?.as_ref() {
        //         "true" | "1" => true,
        //         _ => false,
        //     }
        // } else {
        //     false
        // };
        // name_form.set_extracted(extracted);
        loop {
            match deserializer.read_event_into(&mut buf)? {
                Event::Empty(e) => {
                    log::debug!("read Empty={:?}", e);
                    match e.name().as_ref() {
                        b"gender" => {
                            let attr = e.try_get_attribute("type")?;
                            if let Some(value) = attr {
                                // name_form.set_gender(Gender::from_qname_form_uri(
                                //     value.unescape_value()?.as_ref(),
                                // ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"source" => {
                            let attr = e.try_get_attribute("description")?;
                            if let Some(source) = attr {
                                // name_form.add_source(SourceReference::new(
                                //     Uri::new(source.unescape_value()?.to_string()),
                                //     String::new(),
                                // ));
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
                        b"fullText" => {
                            log::trace!("found 'fullText'");
                            let mut text_buf = Vec::new();
                            if let Event::Text(e_text) =
                                deserializer.read_event_into(&mut text_buf)?
                            {
                                name_form.full_text = e_text.unescape()?.into();
                            } else {
                                todo!("handle not Text")
                            }
                        }
                        _tag => todo!("handle {:?}", e),
                    }
                }
                Event::End(e) => match e.name().as_ref() {
                    b"nameForm" => {
                        log::trace!("found end of 'nameForm' returning ...");
                        break;
                    }
                    _tag => log::trace!("skipping '{:?}' ...", e),
                },
                e => {
                    log::trace!("got: {:?} skipping ...", e);
                }
            }
        }
        log::debug!("name_form = {:?}", name_form);
        Ok(name_form)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
}
