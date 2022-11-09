use crate::common::{DateTime, ResourceReference};
use crate::ser::{SerError, SerializeXml, XmlSerializer};
use chrono::Utc;
use deserx::DeserializeXml;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use serde_with::TimestampMilliSeconds;
use std::io;

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Attribution {
    contributor: ResourceReference,
    #[serde_as(as = "TimestampMilliSeconds")]
    modified: DateTime,
}

impl Attribution {
    pub fn new() -> Self {
        Self {
            contributor: ResourceReference::new(),
            modified: Utc::now(),
        }
    }
}

impl Attribution {
    pub fn contributor<C: Into<ResourceReference>>(mut self, contributor: C) -> Self {
        self.set_contributor(contributor.into());
        self
    }

    pub fn set_contributor(&mut self, contributor: ResourceReference) {
        self.contributor = contributor;
    }

    pub fn modified(mut self, modified: DateTime) -> Self {
        self.set_modified(modified);
        self
    }

    pub fn set_modified(&mut self, modified: DateTime) {
        self.modified = modified;
    }
}
pub fn verify_attribution_opt(
    a: Option<&Attribution>,
    b: Option<&Attribution>,
) -> Result<(), String> {
    if let Some(a_inner) = a {
        if let Some(b_inner) = b {
            if a_inner.contributor != b_inner.contributor {
                return Err(format!(
                    "a.contributor != b.contributor, '{:?}' != '{:?}'",
                    a_inner.contributor, b_inner.contributor,
                ));
            }
            if a_inner.modified != b_inner.modified {
                return Err(format!(
                    "a.modified != b.modified, '{:?}' != '{:?}'",
                    a_inner.modified, b_inner.modified,
                ));
            }
        } else {
            return Err(format!("a != b, '{:?}' != 'None'", a));
        }
    } else {
        if b.is_some() {
            return Err(format!("a != b, 'None' != '{:?}'", b));
        }
    }
    Ok(())
}
impl SerializeXml for Attribution {
    fn tag(&self) -> &str {
        "attribution"
    }
    fn serialize_xml<W: io::Write>(&self, ser: &mut XmlSerializer<W>) -> Result<(), SerError> {
        let elem = BytesStart::new(self.tag());
        ser.write_event(Event::Start(elem))?;

        let mut contributor = BytesStart::new("contributor");
        contributor.push_attribute(("resource", self.contributor.resource()));
        ser.write_event(Event::Empty(contributor))?;

        ser.write_event(Event::Start(BytesStart::new("modified")))?;
        ser.write_event(Event::Text(BytesText::new(&self.modified.to_rfc3339())))?;
        ser.write_event(Event::End(BytesEnd::new("modified")))?;
        ser.write_event(Event::End(BytesEnd::new(self.tag())))?;
        Ok(())
    }
}

impl DeserializeXml for Attribution {
    fn deserialize_xml<R: std::io::BufRead>(
        deserializer: &mut quick_xml::Reader<R>,
    ) -> Result<Self, quick_xml::Error> {
        let mut buf = Vec::new();
        let mut attribution = Self::new();
        loop {
            match deserializer.read_event_into(&mut buf)? {
                Event::Empty(e) => {
                    log::debug!("read Empty={:?}", e);
                    match e.name().as_ref() {
                        b"contributor" => {
                            let attr = e.try_get_attribute("resource")?;
                            if let Some(resource) = attr {
                                attribution.set_contributor(ResourceReference::with_resource(
                                    resource.unescape_value()?.into(),
                                ));
                            }
                        }
                        tag => todo!("handle {:?}", tag),
                    }
                }
                Event::Start(e) => {
                    log::debug!("read Start={:?}", e);
                    match e.name().as_ref() {
                        b"modified" => {
                            log::trace!("found 'modified'");
                            let mut date_buf = Vec::new();
                            let event = deserializer.read_event_into(&mut date_buf)?;
                            if let Event::Text(date_text) = event {
                                log::debug!("date_text = {:?}", date_text);
                                let modified = date_text
                                    .unescape()?
                                    .parse::<DateTime>()
                                    .expect("attribution: failed parse date");
                                attribution.set_modified(modified);
                            }
                        }
                        tag => todo!("handle {:?}", tag),
                    }
                }
                Event::End(e) => match e.name().as_ref() {
                    b"attribution" => {
                        log::trace!("found end of 'attribution' returning ...");
                        break;
                    }
                    _tag => log::trace!("skipping '{:?}' ...", e),
                },
                e => {
                    log::trace!("got: {:?} skipping ...", e);
                }
            }
        }
        log::debug!("attribution = {:?}", attribution);
        Ok(attribution)
    }
}
