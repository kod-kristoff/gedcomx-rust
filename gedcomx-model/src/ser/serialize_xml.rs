use std::io;

use quick_xml::events::{BytesDecl, Event};

use crate::ser::{SerError, XmlSerializer};
use crate::GedcomX;

pub trait SerializeXml {
    fn tag(&self) -> &str;
    // fn children(&self) -> impl Iterator<Item = &dyn SerializeXml>;
    fn serialize_xml<W: io::Write>(&self, ser: &mut quick_xml::Writer<W>) -> Result<(), SerError>;
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
