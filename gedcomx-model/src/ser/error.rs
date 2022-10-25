use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum SerError {
    IoError(io::Error),
    XmlError(quick_xml::Error),
}

impl fmt::Display for SerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(_) => write!(f, "I/O error"),
            Self::XmlError(_) => write!(f, "XML error"),
        }
    }
}

impl Error for SerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IoError(err) => Some(err),
            Self::XmlError(err) => Some(err),
        }
    }
}

impl From<quick_xml::Error> for SerError {
    fn from(err: quick_xml::Error) -> Self {
        Self::XmlError(err)
    }
}
