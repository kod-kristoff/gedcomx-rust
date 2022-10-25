use crate::types::NamePartType;

/// A name conclusion
#[derive(Debug)]
pub struct Name {}

impl Name {
    pub fn new() -> Self {
        Self {}
    }
}

// Builder lite
impl Name {
    pub fn name_form(self, name_form: Option<NameForm>) -> Self {
        self
    }
}

impl Name {
    pub fn get_part(&self, part: NamePartType) -> Option<&str> {
        None
    }
}

/// A name form conclusion
#[derive(Debug)]
pub struct NameForm {
    full_text: String,
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

        #[test]
        fn null_form() {
            let name = Name::new().name_form(None);
            assert!(name.get_part(NamePartType::Given).is_none());
            assert!(name.get_part(NamePartType::Surname).is_none());
        }

        #[test]
        fn form_no_parts() {
            let name_form = NameForm::new()
                .full_text("John Fitzgerald Kennedy".into())
                .lang("en".into());
            let name = Name::new().name_form(Some(name_form));
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
