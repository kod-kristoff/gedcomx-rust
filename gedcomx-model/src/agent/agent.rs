use std::fmt;

use deserx::DeserializeXml;
use quick_xml::events::{BytesEnd, BytesStart, Event};

use crate::{
    common::{ResourceReference, TextValue},
    ser::{xml, SerError, SerializeXml},
};

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Agent {
    names: Vec<TextValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    emails: Vec<ResourceReference>,
    id: String,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            names: Vec::new(),
            emails: Vec::new(),
        }
    }
}

impl Agent {
    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.set_id(id.into());
        self
    }

    pub fn name<T: Into<TextValue>>(mut self, name: T) -> Self {
        self.add_name(name.into());
        self
    }

    pub fn email(mut self, email: impl fmt::Display) -> Self {
        self.add_email(ResourceReference::with_resource(format!(
            "mailto:{}",
            email
        )));
        self
    }
}
impl Agent {
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
    pub fn add_name(&mut self, name: TextValue) {
        self.names.push(name);
    }
    pub fn add_email(&mut self, email: ResourceReference) {
        self.emails.push(email);
    }
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
}

impl From<&Agent> for ResourceReference {
    fn from(agent: &Agent) -> Self {
        ResourceReference::with_resource(format!("#{}", &agent.id))
    }
}

pub fn verify_agents(aas: &[Agent], bs: &[Agent]) -> Result<(), String> {
    if aas.len() != bs.len() {
        return Err(format!("length mismatch: {} != {}", aas.len(), bs.len()));
    }
    todo!()
}
impl SerializeXml for Agent {
    fn tag(&self) -> &str {
        "agent"
    }

    fn serialize_xml<W: std::io::Write>(
        &self,
        ser: &mut quick_xml::Writer<W>,
    ) -> Result<(), SerError> {
        let mut elem = BytesStart::new(self.tag());
        elem.push_attribute(("id", self.id.as_str()));
        ser.write_event(Event::Start(elem))?;

        for email in &self.emails {
            xml::write_elem_w_attribute(ser, "email", ("resource", email.resource()))?;
        }
        for name in &self.names {
            xml::write_elem_w_text(ser, "name", name.value())?;
        }
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}

impl DeserializeXml for Agent {
    fn deserialize_xml_with_start<'de, R: std::io::BufRead>(
        deserializer: &mut quick_xml::Reader<R>,
        start: &quick_xml::events::BytesStart<'de>,
    ) -> Result<Self, quick_xml::Error> {
        let mut buf = Vec::new();
        let mut agent = Self::new();
        let attr = start.try_get_attribute("id")?;
        let id: String = if let Some(id) = attr {
            id.unescape_value()?.into()
            // agent.set_contributor(ResourceReference::with_resource(
            //     resource.unescape_value()?.into(),
            // ));
        } else {
            todo!("handle no 'id'")
        };
        agent.set_id(id);
        loop {
            match deserializer.read_event_into(&mut buf)? {
                Event::Empty(e) => {
                    log::debug!("read Empty={:?}", e);
                    match e.name().as_ref() {
                        b"analysis" => {
                            let attr = e.try_get_attribute("resource")?;
                            if let Some(value) = attr {
                                // agent.set_analysis(DocumentReference::new(
                                //     value.unescape_value()?.into(),
                                // ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"email" => {
                            let attr = e.try_get_attribute("resource")?;
                            if let Some(value) = attr {
                                agent.add_email(ResourceReference::with_resource(
                                    value.unescape_value()?.into(),
                                ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"gender" => {
                            let attr = e.try_get_attribute("type")?;
                            if let Some(value) = attr {
                                // agent.set_gender(Gender::from_qname_uri(
                                //     value.unescape_value()?.as_ref(),
                                // ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"source" => {
                            let attr = e.try_get_attribute("description")?;
                            if let Some(source) = attr {
                                // agent.add_source(SourceReference::new(
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
                        b"citation" => {
                            log::trace!("found 'fact'");
                            // let citation =
                            //     SourceCitation::deserialize_xml_with_start(deserializer, &e)?;
                            // agent.add_citation(citation);
                        }
                        b"name" => {
                            log::trace!("found 'name'");
                            if let Event::Text(name_text) =
                                deserializer.read_event_into(&mut buf)?
                            {
                                agent.add_name(name_text.unescape()?.as_ref().into());
                            }
                        }
                        b"title" => {
                            log::trace!("found 'title'");
                            if let Event::Text(e_title) = deserializer.read_event_into(&mut buf)? {
                                // agent.add_title(e_title.unescape()?.into());
                            }
                        }
                        _tag => todo!("handle {:?}", e),
                    }
                }
                Event::End(e) => match e.name().as_ref() {
                    b"agent" => {
                        log::trace!("found end of 'agent' returning ...");
                        break;
                    }
                    _tag => log::trace!("skipping '{:?}' ...", e),
                },
                e => {
                    log::trace!("got: {:?} skipping ...", e);
                }
            }
        }
        log::debug!("agent = {:?}", agent);
        Ok(agent)
    }
}
