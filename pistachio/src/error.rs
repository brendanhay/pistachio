use std::{
    error,
    fmt,
    io,
};

use crate::{
    lexer::Token,
    parser::ParseError,
    render::RenderError,
};

// XXX: Tidy this up

/// Error type used that can be emitted during template parsing.
#[derive(Debug)]
pub enum Error {
    /// An IO error was encountered - happens when parsing a file
    Io(io::Error),
    Lexer(Box<str>),
    Parser(Box<str>),
    Render(usize, Box<str>),
    ParsingFailed(ParseError<Box<str>, Box<Error>>),
    InvalidPartial(Box<str>),
    LoadingDisabled,
    NotFound(Box<str>),
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<RenderError<io::Error>> for Error {
    fn from(err: RenderError<io::Error>) -> Self {
        match err {
            RenderError::WriteError(io) => Error::Io(io),
            RenderError::MissingVariable(start, key) => Error::Render(start, key),
        }
    }
}

impl From<ParseError<Token<'_>>> for Error {
    fn from(err: ParseError<Token<'_>>) -> Self {
        let err = err.map_token(|token| Box::from(token.to_string()));
        let err = err.map_error(|error| Box::from(error));

        Error::ParsingFailed(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Io(err) => err.fmt(f),
            Lexer(err) => err.fmt(f),
            Parser(err) => err.fmt(f),
            Render(start, key) => write!(f, "Missing variable `{}` at position {}", key, start),
            ParsingFailed(err) => err.fmt(f),
            LoadingDisabled => write!(f, "Partials are not allowed in the current context"),
            InvalidPartial(path) => path.fmt(f),
            NotFound(path) => path.fmt(f),
        }
    }
}
