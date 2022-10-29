use std::io;

use crate::ser::SerError;
use crate::GedcomX;
use quick_xml::events::{BytesDecl, Event};
pub use quick_xml::Writer as XmlSerializer;

pub trait SerializeXml {
    fn tag(&self) -> &str;
    // fn children(&self) -> impl Iterator<Item = &dyn SerializeXml>;
    fn serialize_xml<W: io::Write>(&self, ser: &mut XmlSerializer<W>) -> Result<(), SerError>;
}

impl<T> SerializeXml for Option<T>
where
    T: SerializeXml,
{
    fn tag(&self) -> &str {
        match self {
            None => "",
            Some(t) => t.tag(),
        }
    }
    fn serialize_xml<W: io::Write>(&self, ser: &mut XmlSerializer<W>) -> Result<(), SerError> {
        match self {
            None => Ok(()),
            Some(t) => t.serialize_xml(ser),
        }
    }
}
pub fn serialize_to_xml<W: io::Write>(
    gx: &GedcomX,
    writer: &mut XmlSerializer<W>,
) -> Result<(), SerError> {
    // let mut writer = quick_xml::Writer::new_with_indent(writer, b' ', 2);
    // writer.create_element(gx.tag()).write_empty()?;
    writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )))?;
    gx.serialize_xml(writer)?;
    Ok(())
}
