use std::fmt::{Display, Formatter};
use std::str::Utf8Error;
use clap::ErrorKind;
use good_lp::ResolutionError;
use serde::de::StdError;
use crate::error::Error::{BookDeserialization, Fmt, Io, Term};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    TargetParsingFailed(String),
    Fmt(std::fmt::Error),
    Term(term::Error),
    FilterParsingFailed(String),
    UnknownItem(String),
    UnknownBuilding(String),
    InvalidBuilding(String),
    InvalidRecipeIndex(usize),
    ResolutionFailed(ResolutionError),
    BookDeserialization(serde_json::Error),
    Clap(ErrorKind),
    Utf8(Utf8Error),
}

pub type Result<T> = std::result::Result<T,Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Io(e) => format!("I/O error : {}", e),
            Error::TargetParsingFailed(e) => format!("fail to parse target '{}'",e),
            Fmt(e) => e.to_string(),
            Term(e) => e.to_string(),
            Error::FilterParsingFailed(e) => e.clone(),
            Error::UnknownItem(item) => format!("Unknown item '{}'",item),
            Error::UnknownBuilding(building) => format!("Unknown building '{}'",building),
            Error::InvalidBuilding(building) => format!("Invalid building '{}'",building),
            Error::InvalidRecipeIndex(e) => format!("Invalid recipe index '{}'",e),
            Error::ResolutionFailed(e) => format!("Could not find a solution : {}", e),
            BookDeserialization(e) => format!("Book deserialization failed : {}", e),
            Error::Clap(e) => format!("{:?}", e),
            Error::Utf8(e) => format!("{}", e)
        };

        f.write_str(&message)
    }
}


impl StdError for Error {


}

impl From<term::Error> for Error {
    fn from(e: term::Error) -> Self {
        Term(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Io(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::Utf8(e)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Fmt(e)
    }
}

impl From<ResolutionError> for Error {
    fn from(error: ResolutionError) -> Self {
        Error::ResolutionFailed(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        BookDeserialization(error)
    }
}