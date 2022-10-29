use quick_xml::events::{BytesEnd, BytesStart, Event};

use crate::{
    conclusion::Date,
    ser::{xml, SerializeXml},
    types::FactType,
};

use super::PlaceReference;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
        self.date = Some(date);
        self
    }

    pub fn place(mut self, place: PlaceReference) -> Self {
        self.place = Some(place);
        self
    }

    pub fn value<S: Into<String>>(mut self, s: S) -> Self {
        self.set_value(s.into());
        self
    }
}

impl Fact {
    pub fn set_value(&mut self, value: String) {
        self.value = value;
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
