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
    /// An IO error occurred while parsing or rendering.
    Io(io::Error),

    /// An LR parser error occurred.
    ParsingFailed(Box<str>),

    /// Loading templates at runtime is disabled.
    LoadingDisabled,

    /// An attempt to include a partial or parent failed.
    InvalidPartial(Box<str>),

    /// Tried to serialize a map key that was not a string.
    KeyMustBeAString,

    /// Tried to serialize a number bigger than the maximum allowable value for its type.
    NumberOutOfRange,

    /// A variable wasn't found on the stack and raising errors is enabled.
    MissingVariable((usize, usize), Box<str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => fmt::Display::fmt(err, f),
            Error::ParsingFailed(msg) => f.write_str(msg),
            Error::LoadingDisabled => {
                f.write_str("loading templates from the filesystem is disabled")
            },
            Error::InvalidPartial(msg) => write!(f, "partial path {} is invalid", msg),
            Error::KeyMustBeAString => f.write_str("key must be a string"),
            Error::NumberOutOfRange => f.write_str("number out of range"),
            Error::MissingVariable(span, var) => write!(
                f,
                "missing variable `{{{{{}}}}}` at position {:?}",
                var, span
            ),
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
