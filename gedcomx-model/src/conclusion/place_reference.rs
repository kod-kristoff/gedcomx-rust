use quick_xml::events::{BytesEnd, BytesStart, Event};

use crate::ser::{xml, SerializeXml};

/// A name conclusion
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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
