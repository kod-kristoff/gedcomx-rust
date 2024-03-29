use chrono::Utc;
use deserx::DeserializeXml;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use serde::{Deserialize, Deserializer, Serializer};

use super::{SourceCitation, SourceReference};
use crate::{
    common::{DateTime, IriRef, ResourceReference, TextValue, Uri},
    ser::{xml, SerError, SerializeXml},
    types::ResourceType,
};


pub fn ser_opt_date<S: Serializer>(v: &Option<DateTime>, s: S) -> Result<S::Ok, S::Error> {
    match v {
        // Some(d) => s.serialize_i64(DateTime::from_utc(d.and_hms(0, 0, 0), Utc).timestamp()),
        Some(d) => s.serialize_i64(d.timestamp_millis()),
        None => s.serialize_unit(),
    }
}

pub fn deserialize_optional_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(OptionalDateTimeVisitor)
}

struct OptionalDateTimeVisitor;

impl<'de> serde::de::Visitor<'de> for OptionalDateTimeVisitor {
    type Value = Option<DateTime>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("optional timestamp")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        use chrono::TimeZone;
        Ok(Some(Utc.timestamp_millis(i64::deserialize(deserializer)?)))
    }
}

// #[serdxce_as]
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceDescription {
    resource_type: Option<Uri>,
    citations: Vec<SourceCitation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    names: Vec<TextValue>,
    titles: Vec<TextValue>,
    #[serde(
        serialize_with = "ser_opt_date",
        deserialize_with = "deserialize_optional_datetime"
    )]
    // #[serde_as(as = "TimestampMilliSeconds<i64>")]
    created: Option<DateTime>,
    repository: Option<ResourceReference>,
    // #[serde(default)]
    id: IriRef,
}

impl SourceDescription {
    pub fn new(id: IriRef) -> Self {
        Self {
            id,
            names: Vec::new(),
            titles: Vec::new(),
            citations: Vec::new(),
            repository: None,
            created: None,
            resource_type: None,
        }
    }
}

impl SourceDescription {
    // pub fn id<S: Into<String>>(mut self, id: S) -> Self {
    //     self.set_id(id.into());
    //     self
    // }

    pub fn repository<S: Into<ResourceReference>>(mut self, repository: S) -> Self {
        self.set_repository(repository.into());
        self
    }

    pub fn created<S: Into<DateTime>>(mut self, created: S) -> Self {
        self.set_created(created.into());
        self
    }

    pub fn resource_type(mut self, resource_type: ResourceType) -> Self {
        self.set_resource_type(resource_type.into());
        self
    }

    pub fn citation<T: Into<SourceCitation>>(mut self, citation: T) -> Self {
        self.add_citation(citation.into());
        self
    }

    pub fn name<T: Into<TextValue>>(mut self, name: T) -> Self {
        self.add_name(name.into());
        self
    }

    pub fn title<T: Into<TextValue>>(mut self, title: T) -> Self {
        self.add_title(title.into());
        self
    }
}
impl SourceDescription {
    // pub fn set_id(&mut self, id: String) {
    //     self.id = id;
    // }
    pub fn set_repository(&mut self, repository: ResourceReference) {
        self.repository = Some(repository);
    }
    pub fn set_created(&mut self, created: DateTime) {
        self.created = Some(created);
    }
    pub fn set_resource_type(&mut self, resource_type: Uri) {
        self.resource_type = Some(resource_type);
    }
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
    pub fn add_title(&mut self, title: TextValue) {
        self.titles.push(title);
    }
    pub fn add_citation(&mut self, citation: SourceCitation) {
        self.citations.push(citation);
    }
    pub fn add_name(&mut self, name: TextValue) {
        self.names.push(name);
    }
}

impl From<&SourceDescription> for ResourceReference {
    fn from(source: &SourceDescription) -> Self {
        ResourceReference::new(source.id.clone())
    }
}

impl From<&SourceDescription> for SourceReference {
    fn from(source: &SourceDescription) -> Self {
        Self::new(source.id.clone(), source.id.clone().into_inner())
    }
}
impl SerializeXml for SourceDescription {
    fn tag(&self) -> &str {
        "sourceDescription"
    }

    fn serialize_xml<W: std::io::Write>(
        &self,
        ser: &mut quick_xml::Writer<W>,
    ) -> Result<(), SerError> {
        let mut elem = BytesStart::new(self.tag());
        if let Some(resource_type) = &self.resource_type {
            elem.push_attribute(("resourceType", resource_type.as_str()));
        }
        elem.push_attribute(("id", self.id.as_str()));
        ser.write_event(Event::Start(elem))?;

        for citation in &self.citations {
            citation.serialize_xml(ser)?;
        }
        for title in &self.titles {
            xml::write_elem_w_text(ser, "title", title.value())?;
        }
        if let Some(created) = &self.created {
            xml::write_elem_w_text(ser, "created", &created.to_rfc3339())?;
        }
        if let Some(repository) = &self.repository {
            xml::write_elem_w_attribute(ser, "repository", ("resource", &repository.resource()))?;
        }
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}

impl DeserializeXml for SourceDescription {
    fn deserialize_xml_with_start<'de, R: std::io::BufRead>(
        deserializer: &mut quick_xml::Reader<R>,
        start: &quick_xml::events::BytesStart<'de>,
    ) -> Result<Self, quick_xml::Error> {
        let mut buf = Vec::new();
        let attr = start.try_get_attribute("id")?;
        let id: String = if let Some(id) = attr {
            id.unescape_value()?.into()
            // source_description.set_contributor(ResourceReference::with_resource(
            //     resource.unescape_value()?.into(),
            // ));
        } else {
            todo!("handle no 'id'")
        };
        let mut source_description = Self::new(IriRef::parse(id).expect("parsing iri"));
        let attr = start.try_get_attribute("resourceType")?;
        let resource_type = if let Some(resource_type) = attr {
            Uri::new(resource_type.unescape_value()?.into())
        } else {
            todo!()
        };
        source_description.set_resource_type(resource_type);
        loop {
            match deserializer.read_event_into(&mut buf)? {
                Event::Empty(e) => {
                    log::debug!("read Empty={:?}", e);
                    match e.name().as_ref() {
                        b"analysis" => {
                            let attr = e.try_get_attribute("resource")?;
                            if let Some(_value) = attr {
                                // source_description.set_analysis(DocumentReference::new(
                                //     value.unescape_value()?.into(),
                                // ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"repository" => {
                            let attr = e.try_get_attribute("resource")?;
                            if let Some(value) = attr {
                                source_description.set_repository(ResourceReference::new(
                                    IriRef::parse(value.unescape_value()?.into())
                                        .expect("parsing iri"),
                                ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"gender" => {
                            let attr = e.try_get_attribute("type")?;
                            if let Some(_value) = attr {
                                // source_description.set_gender(Gender::from_qname_uri(
                                //     value.unescape_value()?.as_ref(),
                                // ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        b"source" => {
                            let attr = e.try_get_attribute("description")?;
                            if let Some(_source) = attr {
                                // source_description.add_source(SourceReference::new(
                                //     Uri::new(source.unescape_value()?.to_string()),
                                //     String::new(),
                                // ));
                            } else {
                                todo!("handle error")
                            }
                        }
                        _tag => todo!("handle {:?}", e),
                    }
                }
                Event::Start(e) => {
                    log::debug!("read Start={:?}", e);
                    match e.name().as_ref() {
                        b"citation" => {
                            log::trace!("found 'fact'");
                            let citation =
                                SourceCitation::deserialize_xml_with_start(deserializer, &e)?;
                            source_description.add_citation(citation);
                        }
                        b"created" => {
                            log::trace!("found 'created'");
                            if let Event::Text(e_created) =
                                deserializer.read_event_into(&mut buf)?
                            {
                                source_description.set_created(
                                    e_created.unescape()?.as_ref().parse().expect("datetime"),
                                );
                            }
                        }
                        b"title" => {
                            log::trace!("found 'title'");
                            if let Event::Text(e_title) = deserializer.read_event_into(&mut buf)? {
                                source_description.add_title(e_title.unescape()?.into());
                            }
                        }
                        _tag => todo!("handle {:?}", e),
                    }
                }
                Event::End(e) => match e.name().as_ref() {
                    b"sourceDescription" => {
                        log::trace!("found end of 'source_description' returning ...");
                        break;
                    }
                    _tag => log::trace!("skipping '{:?}' ...", e),
                },
                e => {
                    log::trace!("got: {:?} skipping ...", e);
                }
            }
        }
        log::debug!("source_description = {:?}", source_description);
        Ok(source_description)
    }
}
