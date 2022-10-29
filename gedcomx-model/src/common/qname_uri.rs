pub trait QnameUri {
    fn as_qname_uri(&self) -> &str;
}

// impl<T: QnameUri> serde::Serialize for T {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         use serde::ser::SerializeMap;
//         let mut map = serializer.serialize_map(Some(1))?;
//         map.serialize_entry("type", self.as_qname_uri())?;
//         map.end()
//     }
// }
