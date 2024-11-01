use crate::dom::TryFromDomError;
use thiserror::Error;
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum MyError {
    #[error(r#"{0}. path: "{1}""#)]
    OpenZipError(ZipError, String),

    #[error("parse error")]
    ParseError,

    #[error("unknown path command {0}. At {1}")]
    UnknownPathCommand(String, usize),

    #[error("invalid")]
    Invalid,

    #[error(transparent)]
    MiniDomError(#[from] minidom::Error),

    #[error(transparent)]
    TryFromDomError(#[from] TryFromDomError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    XmlDeError(#[from] xdom::de::XmlDeError),
}
