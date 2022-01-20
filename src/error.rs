use good_lp::ResolutionError;
use crate::error::Error::{BookDeserializationError, FmtError, IoError, TermError};

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    FmtError(std::fmt::Error),
    TermError(term::Error),
    UnknownItem(String),
    UnknownBuilding(String),
    InvalidBuilding(String),
    InvalidRecipeIndex(usize),
    ResolutionError(ResolutionError),
    BookDeserializationError(serde_json::Error),
}

pub type Result<T> = std::result::Result<T,Error>;


impl From<term::Error> for Error {
    fn from(e: term::Error) -> Self {
        TermError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        IoError(e)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        FmtError(e)
    }
}

impl From<ResolutionError> for Error {
    fn from(error: ResolutionError) -> Self {
        Error::ResolutionError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        BookDeserializationError(error)
    }
}