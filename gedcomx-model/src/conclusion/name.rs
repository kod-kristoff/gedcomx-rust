use quick_xml::events::{BytesEnd, BytesStart, Event};

use crate::{conclusion::NameForm, ser::SerializeXml, types::NamePartType};

/// A name conclusion
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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
        self.name_forms.push(name_form);
        self
    }
}

impl Name {
    pub fn get_part(&self, part: NamePartType) -> Option<&str> {
        None
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
