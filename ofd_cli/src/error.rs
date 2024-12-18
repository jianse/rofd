use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    // #[error(transparent)]
    // MiniDomError(#[from] minidom::Error),

    // #[error(transparent)]
    // IOError(#[from] std::io::Error),

    // #[error(transparent)]
    // XmlDeError(#[from] xdom::de::XmlDeError),
    #[error("parse duration error. At {pos}, Reason: {reason}")]
    ParseDurationError { pos: usize, reason: String },
}
