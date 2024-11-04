use thiserror::Error;
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum MyError {
    #[error(r#"{0}. path: "{1}""#)]
    OpenZipError(ZipError, String),

    #[error("error parse xml: {0}")]
    MiniDomError(#[from] minidom::Error),

    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),

    #[error("{0}")]
    XmlDeError(#[from] xdom::de::XmlDeError),
}
