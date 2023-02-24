use deserx::DeserializeXml;
use quick_xml::events::{BytesEnd, BytesStart, Event};

use crate::{
    conclusion::Date,
    ser::{xml, SerializeXml},
    types::FactType,
};

use super::PlaceReference;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fact {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    value: String,
    r#type: FactType,
    date: Option<Date>,
    place: Option<PlaceReference>,
}

impl Fact {
    pub fn new(fact_type: FactType) -> Self {
        Self {
            r#type: fact_type,
            date: None,
            place: None,
            value: String::new(),
        }
    }
}

impl Fact {
    pub fn date(mut self, date: Date) -> Self {
        self.set_date(date);
        self
    }

    pub fn place(mut self, place: PlaceReference) -> Self {
        self.set_place(place);
        self
    }

    pub fn value<S: Into<String>>(mut self, s: S) -> Self {
        self.set_value(s.into());
        self
    }
}

impl Fact {
    pub fn set_date(&mut self, date: Date) {
        self.date = Some(date);
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    pub fn set_place(&mut self, place: PlaceReference) {
        self.place = Some(place);
    }

    pub fn r#type(&self) -> FactType {
        self.r#type
    }
}

impl SerializeXml for Fact {
    fn tag(&self) -> &str {
        "fact"
    }

    fn serialize_xml<W: std::io::Write>(
        &self,
        ser: &mut quick_xml::Writer<W>,
    ) -> Result<(), crate::ser::SerError> {
        let mut elem = BytesStart::new(self.tag());
        elem.push_attribute(("type", self.r#type.as_qname_uri()));
        ser.write_event(Event::Start(elem))?;
        self.date.serialize_xml(ser)?;
        self.place.serialize_xml(ser)?;
        xml::write_elem_w_text_if_not_empty(ser, "value", &self.value)?;
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}

impl DeserializeXml for Fact {
    fn deserialize_xml_with_start<'de, R: std::io::BufRead>(
        deserializer: &mut quick_xml::Reader<R>,
        start: &quick_xml::events::BytesStart<'de>,
    ) -> Result<Self, quick_xml::Error> {
        let mut buf = Vec::new();
        let attr = start.try_get_attribute("type")?;
        let fact_type: FactType = if let Some(fact_type) = attr {
            FactType::from_qname_uri(fact_type.unescape_value()?.as_ref())
        } else {
            todo!("handle no 'fact_type'")
        };
        let mut fact = Self::new(fact_type);
        // let attr = start.try_get_attribute("extracted")?;
        // let extracted = if let Some(extracted) = attr {
        //     match extracted.unescape_value()?.as_ref() {
        //         "true" | "1" => true,
        //         _ => false,
        //     }
        // } else {
        //     false
        // };
        // fact.set_extracted(extracted);
        loop {
            match deserializer.read_event_into(&mut buf)? {
                Event::Empty(e) => {
                    log::debug!("read Empty={:?}", e);
                    match e.name().as_ref() {
                        b"gender" => {
                            let attr = e.try_get_attribute("type")?;
                            if let Some(_value) = attr {
                                // fact.set_gender(Gender::from_qfact_uri(
                                //     value.unescape_value()?.as_ref(),
                                // ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"source" => {
                            let attr = e.try_get_attribute("description")?;
                            if let Some(_source) = attr {
                                // fact.add_source(SourceReference::new(
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
                        b"date" => {
                            log::trace!("found 'date'");
                            let date = Date::deserialize_xml_with_start(deserializer, &e)?;
                            fact.set_date(date);
                        }
                        b"place" => {
                            log::trace!("found 'place'");
                            let place =
                                PlaceReference::deserialize_xml_with_start(deserializer, &e)?;
                            fact.set_place(place);
                        }
                        b"value" => {
                            log::trace!("found 'value'");
                            if let Event::Text(value_text) =
                                deserializer.read_event_into(&mut buf)?
                            {
                                fact.set_value(value_text.unescape()?.into());
                            }
                        }
                        _tag => todo!("handle {:?}", e),
                    }
                }
                Event::End(e) => match e.name().as_ref() {
                    b"fact" => {
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
        log::debug!("fact = {:?}", fact);
        Ok(fact)
    }
}
