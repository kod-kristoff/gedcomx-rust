pub trait DeserializeXml: Sized {
    fn deserialize_xml<R: std::io::BufRead>(
        _deserializer: &mut quick_xml::Reader<R>,
    ) -> Result<Self, quick_xml::Error> {
        todo!("return error")
    }
    fn deserialize_xml_with_start<'de, R: std::io::BufRead>(
        _deserializer: &mut quick_xml::Reader<R>,
        _start: &quick_xml::events::BytesStart<'de>,
    ) -> Result<Self, quick_xml::Error> {
        todo!("return error")
    }
}
