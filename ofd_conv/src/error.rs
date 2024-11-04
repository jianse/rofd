use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("unknown path command {0}. At {1}")]
    UnknownPathCommand(String, usize),

    #[error("parse error")]
    ParseError,

    #[error("invalid")]
    Invalid,
}
