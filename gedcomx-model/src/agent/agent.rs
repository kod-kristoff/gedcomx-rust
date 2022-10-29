use std::fmt;

use quick_xml::events::{BytesEnd, BytesStart, Event};

use crate::{
    common::{ResourceReference, TextValue},
    ser::{xml, SerError, SerializeXml},
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Agent {
    names: Vec<TextValue>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
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
