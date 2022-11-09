use deserx::DeserializeXml;
use quick_xml::events::{BytesEnd, BytesStart, Event};

use crate::ser::{xml, SerializeXml};

/// A place conclusion
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaceReference {
    original: String,
}

impl PlaceReference {
    pub fn new() -> Self {
        Self {
            original: String::new(),
        }
    }
}

// Builder lite
impl PlaceReference {
    pub fn original<S: Into<String>>(mut self, original: S) -> Self {
        self.set_original(original.into());
        self
    }
}

impl PlaceReference {
    pub fn set_original(&mut self, original: String) {
        self.original = original;
    }
}

impl SerializeXml for PlaceReference {
    fn tag(&self) -> &str {
        "place"
    }
    fn serialize_xml<W: std::io::Write>(
        &self,
        ser: &mut quick_xml::Writer<W>,
    ) -> Result<(), crate::ser::SerError> {
        let elem = BytesStart::new(self.tag());
        ser.write_event(Event::Start(elem))?;

        if !self.original.is_empty() {
            xml::write_elem_w_text(ser, "original", &self.original)?;
        }
        // for name_form in &self.name_forms {
        //     name_form.serialize_xml(ser)?;
        // }
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}
impl DeserializeXml for PlaceReference {
    fn deserialize_xml_with_start<'de, R: std::io::BufRead>(
        deserializer: &mut quick_xml::Reader<R>,
        start: &quick_xml::events::BytesStart<'de>,
    ) -> Result<Self, quick_xml::Error> {
        let mut buf = Vec::new();
        let mut place = Self::new();
        // let attr = start.try_get_attribute("extracted")?;
        // let extracted = if let Some(extracted) = attr {
        //     match extracted.unescape_value()?.as_ref() {
        //         "true" | "1" => true,
        //         _ => false,
        //     }
        // } else {
        //     false
        // };
        // place.set_extracted(extracted);
        loop {
            match deserializer.read_event_into(&mut buf)? {
                Event::Empty(e) => {
                    log::debug!("read Empty={:?}", e);
                    match e.name().as_ref() {
                        b"gender" => {
                            let attr = e.try_get_attribute("type")?;
                            if let Some(value) = attr {
                                // place.set_gender(Gender::from_qplace_uri(
                                //     value.unescape_value()?.as_ref(),
                                // ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"source" => {
                            let attr = e.try_get_attribute("description")?;
                            if let Some(source) = attr {
                                // place.add_source(SourceReference::new(
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
                        b"original" => {
                            log::trace!("found 'original'");
                            if let Event::Text(e_text) = deserializer.read_event_into(&mut buf)? {
                                place.set_original(e_text.unescape()?.into());
                            }
                        }
                        _tag => todo!("handle {:?}", e),
                    }
                }
                Event::End(e) => match e.name().as_ref() {
                    b"place" => {
                        log::trace!("found end of 'place' returning ...");
                        break;
                    }
                    _tag => log::trace!("skipping '{:?}' ...", e),
                },
                e => {
                    log::trace!("got: {:?} skipping ...", e);
                }
            }
        }
        log::debug!("place = {:?}", place);
        Ok(place)
    }
}
