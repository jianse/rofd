use std::{path::PathBuf, str::FromStr};

use thiserror::Error;
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum MyError {
    #[error(r#"{0}. path: "{1}""#)]
    OpenZipError(ZipError, PathBuf),

    #[error("parse error")]
    ParseError,

    #[error("unknow path command {0}.")]
    UnknownPathCommnad(String),

    #[error("invalid")]
    Invalid,
}
