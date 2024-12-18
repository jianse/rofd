use sm2::pkcs8;
use sm2::pkcs8::spki;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Empty DerFragment is not allowed")]
    ConvertError,
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    DerError(#[from] der::Error),

    #[error(transparent)]
    SpkiError(#[from] spki::Error),

    #[error(transparent)]
    Pkcs8Error(#[from] pkcs8::Error),

    #[error("Unsupported signature type")]
    UnSupportedSignClass,
}
