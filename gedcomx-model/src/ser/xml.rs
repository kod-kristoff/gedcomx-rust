use std::io;

use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Result,
};

pub fn write_elem_w_text<W: io::Write>(
    ser: &mut quick_xml::Writer<W>,
    tag: &str,
    text: &str,
) -> Result<()> {
    ser.write_event(Event::Start(BytesStart::new(tag)))?;
    ser.write_event(Event::Text(BytesText::new(text)))?;
    ser.write_event(Event::End(BytesEnd::new(tag)))
}

pub fn write_elem_w_attribute<W: io::Write>(
    ser: &mut quick_xml::Writer<W>,
    tag: &str,
    attribute: (&str, &str),
) -> Result<()> {
    let mut elem = BytesStart::new(tag);
    elem.push_attribute(attribute);
    ser.write_event(Event::Empty(elem))
    // ser.write_event(Event::Text(BytesText::new(text)))?;
    // ser.write_event(Event::End(BytesEnd::new(tag)))
}

pub fn write_elem_w_text_if_not_empty<W: io::Write>(
    ser: &mut quick_xml::Writer<W>,
    tag: &str,
    text: &str,
) -> Result<()> {
    if text.len() > 0 {
        println!("writing ...");
        write_elem_w_text(ser, tag, text)?;
    }
    Ok(())
}
