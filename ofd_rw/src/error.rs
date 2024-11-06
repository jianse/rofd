use thiserror::Error;
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum Error {
    #[error(r#"{0}. path: "{1}""#)]
    OpenZipError(ZipError, String),

    #[error("ofd entry not found")]
    OfdEntryNotFound,

    #[error("no such document")]
    NoSuchDocument,

    #[error("no such template")]
    NoSuchTemplate,

    #[error("error parse xml: {0}")]
    MiniDomError(#[from] minidom::Error),

    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),

    #[error("{0}")]
    XmlDeError(#[from] xdom::de::XmlDeError),

    #[error("{0}")]
    ZipError(#[from] ZipError),
}

pub type Result<T> = std::result::Result<T, Error>;
