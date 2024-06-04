use std::path::PathBuf;

use thiserror::Error;
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum MyError {
    #[error(r#"{0}. path: "{1}""#)]
    OpenZipError(ZipError, PathBuf),
}
