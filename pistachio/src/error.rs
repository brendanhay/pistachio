use std::{
    fmt,
    io,
};

use crate::parser::{
    ParseError,
    Token,
};

#[derive(Debug)]
pub enum Error {
    /// Catchall for syntax error messages
    Message(Box<str>),

    /// An IO error occurred while parsing or rendering.
    Io(io::Error),

    /// An LR parser error occurred.
    ParsingFailed(Box<str>),

    /// Loading templates at runtime is disabled.
    LoadingDisabled,

    /// An attempt to include a partial or parent failed.
    InvalidPartial,

    /// Tried to serialize a map key that was not a string.
    KeyMustBeAString,

    /// Tried to serialize a number bigger than the maximum allowable value for its type.
    NumberOutOfRange,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => f.write_str(msg),
            Error::Io(err) => fmt::Display::fmt(err, f),
            Error::ParsingFailed(msg) => f.write_str(msg),
            Error::LoadingDisabled => todo!(),
            Error::InvalidPartial => todo!(),
            Error::KeyMustBeAString => f.write_str("key must be a string"),
            Error::NumberOutOfRange => f.write_str("number out of range"),
        }
    }
}

impl From<ParseError<Token<'_>>> for Error {
    fn from(err: ParseError<Token<'_>>) -> Self {
        match err {
            ParseError::User { error } => error,
            _ => Error::ParsingFailed(Box::from(err.to_string())),
        }
    }
}
