use good_lp::ResolutionError;
use crate::error::Error::BookDeserializationError;

#[derive(Debug)]
pub enum Error {
    UnknownItem(String),
    InvalidRecipeIndex(usize),
    ResolutionError(ResolutionError),
    BookDeserializationError(serde_json::Error),
}

pub type Result<T> = std::result::Result<T,Error>;


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