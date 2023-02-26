mod evidence_reference;
mod qname_uri;
mod reference;
mod resource_reference;
mod text_value;
mod uri;

pub use evidence_reference::EvidenceReference;
pub use qname_uri::QnameUri;
pub use reference::Reference;
pub use resource_reference::ResourceReference;
pub use text_value::TextValue;
pub use uri::Uri;

pub type Date = chrono::NaiveDate;
pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type Iri = oxiri::Iri<String>;
pub type IriRef = oxiri::IriRef<String>;
pub type IriParseError = oxiri::IriParseError;
