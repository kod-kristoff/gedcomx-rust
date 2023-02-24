use deserx::DeserializeXml;
use quick_xml::events::{BytesEnd, BytesStart, Event};

use crate::{conclusion::NameForm, ser::SerializeXml, types::NamePartType};

/// A name conclusion
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    name_forms: Vec<NameForm>,
}

impl Name {
    pub fn new() -> Self {
        Self {
            name_forms: Vec::new(),
        }
    }
}

// Builder lite
impl Name {
    pub fn name_form(mut self, name_form: NameForm) -> Self {
        self.add_name_form(name_form);
        self
    }
}

impl Name {
    pub fn add_name_form(&mut self, name_form: NameForm) {
        self.name_forms.push(name_form);
    }
    pub fn get_part(&self, _part: NamePartType) -> Option<&str> {
        None
    }

    pub fn name_forms(&self) -> &[NameForm] {
        self.name_forms.as_slice()
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self::new().name_form(NameForm::new().full_text(s.into()))
    }
}

impl SerializeXml for Name {
    fn tag(&self) -> &str {
        "name"
    }
    fn serialize_xml<W: std::io::Write>(
        &self,
        ser: &mut quick_xml::Writer<W>,
    ) -> Result<(), crate::ser::SerError> {
        let elem = BytesStart::new(self.tag());
        ser.write_event(Event::Start(elem))?;

        for name_form in &self.name_forms {
            name_form.serialize_xml(ser)?;
        }
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}
impl DeserializeXml for Name {
    fn deserialize_xml_with_start<'de, R: std::io::BufRead>(
        deserializer: &mut quick_xml::Reader<R>,
        _start: &quick_xml::events::BytesStart<'de>,
    ) -> Result<Self, quick_xml::Error> {
        let mut buf = Vec::new();
        // let attr = start.try_get_attribute("id")?;
        // let id: String = if let Some(id) = attr {
        //     id.unescape_value()?.into()
        // } else {
        //     todo!("handle no 'id'")
        // };
        let mut name = Self::new();
        // let attr = start.try_get_attribute("extracted")?;
        // let extracted = if let Some(extracted) = attr {
        //     match extracted.unescape_value()?.as_ref() {
        //         "true" | "1" => true,
        //         _ => false,
        //     }
        // } else {
        //     false
        // };
        // name.set_extracted(extracted);
        loop {
            match deserializer.read_event_into(&mut buf)? {
                Event::Empty(e) => {
                    log::debug!("read Empty={:?}", e);
                    match e.name().as_ref() {
                        b"gender" => {
                            let attr = e.try_get_attribute("type")?;
                            if let Some(_value) = attr {
                                // name.set_gender(Gender::from_qname_uri(
                                //     value.unescape_value()?.as_ref(),
                                // ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"source" => {
                            let attr = e.try_get_attribute("description")?;
                            if let Some(_source) = attr {
                                // name.add_source(SourceReference::new(
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
                        b"nameForm" => {
                            log::trace!("found 'name'");
                            let name_form = NameForm::deserialize_xml_with_start(deserializer, &e)?;
                            name.add_name_form(name_form);
                        }
                        _tag => todo!("handle {:?}", e),
                    }
                }
                Event::End(e) => match e.name().as_ref() {
                    b"name" => {
                        log::trace!("found end of 'name' returning ...");
                        break;
                    }
                    _tag => log::trace!("skipping '{:?}' ...", e),
                },
                e => {
                    log::trace!("got: {:?} skipping ...", e);
                }
            }
        }
        log::debug!("name = {:?}", name);
        Ok(name)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    mod get_part {
        use super::*;
        #[test]
        fn no_forms() {
            let name = Name::new();
            assert!(name.get_part(NamePartType::Given).is_none());
            assert!(name.get_part(NamePartType::Surname).is_none());
        }

        // #[test]
        // fn null_form() {
        //     let name = Name::new().name_form(None);
        //     assert!(name.get_part(NamePartType::Given).is_none());
        //     assert!(name.get_part(NamePartType::Surname).is_none());
        // }

        #[test]
        fn form_no_parts() {
            let name_form = NameForm::new()
                .full_text("John Fitzgerald Kennedy".into())
                .lang("en".into());
            let name = Name::new().name_form(name_form);
            assert!(name.get_part(NamePartType::Given).is_none());
            assert!(name.get_part(NamePartType::Surname).is_none());
        }
        // NameForm nameForm = new NameForm("John Fitzgerald Kennedy")
        //   .lang("en")
        //   .part(NamePartType.Given, "John")
        //   .part(NamePartType.Given, "Fitzgerald")
        //   .part(NamePartType.Surname, "Kennedy");
        // Name name = new Name().nameForm(nameForm);
        // assertEquals("John", name.getPart(NamePartType.Given));
        // assertEquals("Kennedy", name.getPart(NamePartType.Surname));

        // assertNull(nameNoParts.getPart(NamePartType.Given));
        // assertNull(nameNoParts.getPart(NamePartType.Surname));

        // NameForm nameFormNullParts = new NameForm("John Fitzgerald Kennedy")
        //   .lang("en")
        //   .part(NamePartType.Given, null)
        //   .part(NamePartType.Surname, null);
        // Name nameNullParts = new Name().nameForm(nameFormNullParts);
        // assertNull(nameNullParts.getPart(NamePartType.Given));
        // assertNull(nameNullParts.getPart(NamePartType.Surname));
    }
}
