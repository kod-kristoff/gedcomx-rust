use crate::ser::{xml, SerError, SerializeXml};

use deserx::DeserializeXml;
use quick_xml::events::{BytesEnd, BytesStart, Event};

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
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

impl DeserializeXml for SourceCitation {
    fn deserialize_xml_with_start<'de, R: std::io::BufRead>(
        deserializer: &mut quick_xml::Reader<R>,
        _start: &quick_xml::events::BytesStart<'de>,
    ) -> Result<Self, quick_xml::Error> {
        let mut buf = Vec::new();
        let mut source_citation = Self::new();
        // let attr = start.try_get_attribute("id")?;
        // let id: String = if let Some(id) = attr {
        //     id.unescape_value()?.into()
        //     // source_citation.set_contributor(ResourceReference::with_resource(
        //     //     resource.unescape_value()?.into(),
        //     // ));
        // } else {
        //     todo!("handle no 'id'")
        // };
        // source_citation.set_id(id);
        // let attr = start.try_get_attribute("resourceType")?;
        // let resource_type = if let Some(resource_type) = attr {
        //     Uri::new(resource_type.unescape_value()?.into())
        // } else {
        //     todo!()
        // };
        // source_citation.set_resource_type(resource_type);
        loop {
            match deserializer.read_event_into(&mut buf)? {
                Event::Empty(e) => {
                    log::debug!("read Empty={:?}", e);
                    match e.name().as_ref() {
                        b"analysis" => {
                            let attr = e.try_get_attribute("resource")?;
                            if let Some(_value) = attr {
                                // source_citation.set_analysis(DocumentReference::new(
                                //     value.unescape_value()?.into(),
                                // ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"evidence" => {
                            let attr = e.try_get_attribute("resource")?;
                            if let Some(_value) = attr {
                                // source_citation
                                //     .subject
                                //     .add_evidence(EvidenceReference::with_resource(
                                //         value.unescape_value()?.into(),
                                //     ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"gender" => {
                            let attr = e.try_get_attribute("type")?;
                            if let Some(_value) = attr {
                                // source_citation.set_gender(Gender::from_qname_uri(
                                //     value.unescape_value()?.as_ref(),
                                // ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"source" => {
                            let attr = e.try_get_attribute("description")?;
                            if let Some(_source) = attr {
                                // source_citation.add_source(SourceReference::new(
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
                        b"value" => {
                            log::trace!("found 'value'");
                            if let Event::Text(e_text) = deserializer.read_event_into(&mut buf)? {
                                source_citation.set_value(e_text.unescape()?.into());
                            }
                        }
                        b"name" => {
                            log::trace!("found 'name'");
                            // let name = Name::deserialize_xml_with_start(deserializer, &e)?;
                            // source_citation.add_name(name);
                        }
                        _tag => todo!("handle {:?}", e),
                    }
                }
                Event::End(e) => match e.name().as_ref() {
                    b"citation" => {
                        log::trace!("found end of 'source_citation' returning ...");
                        break;
                    }
                    _tag => log::trace!("skipping '{:?}' ...", e),
                },
                e => {
                    log::trace!("got: {:?} skipping ...", e);
                }
            }
        }
        log::debug!("source_citation = {:?}", source_citation);
        Ok(source_citation)
    }
}
