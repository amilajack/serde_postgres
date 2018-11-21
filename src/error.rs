use std::{fmt, error};

use serde::{de, ser};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
    TrailingValues,
    UnknownField,
    InvalidType,
    UnsupportedType,
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(error::Error::description(self))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Message(ref msg) => msg,
            Error::TrailingValues => "Unexpected columns",
            Error::UnknownField => "Unknown field",
            Error::InvalidType => "Invalid type",
            Error::UnsupportedType => "Type unsupported",
        }
    }
}
