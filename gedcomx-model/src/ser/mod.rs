mod error;
mod serialize_xml;
pub mod xml;

pub use crate::ser::error::SerError;
pub use crate::ser::serialize_xml::{serialize_to_xml, SerializeXml};

pub use quick_xml::Writer as XmlSerializer;
