use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Empty DerFragment is not allowed")]
    ConvertError,
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    DerError(#[from] der::Error),

    #[error("Unsupported signature type")]
    UnSupportedSignClass,
}
