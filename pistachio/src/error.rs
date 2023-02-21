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

// pub(crate) enum ErrorCode {
//     /// Catchall for syntax error messages
//     Message(Box<str>),

//     /// Some IO error occurred while serializing or deserializing.
//     Io(io::Error),

//     /// EOF while parsing a list.
//     EofWhileParsingList,

//     /// EOF while parsing an object.
//     EofWhileParsingObject,

//     /// EOF while parsing a string.
//     EofWhileParsingString,

//     /// EOF while parsing a JSON value.
//     EofWhileParsingValue,

//     /// Expected this character to be a `':'`.
//     ExpectedColon,

//     /// Expected this character to be either a `','` or a `']'`.
//     ExpectedListCommaOrEnd,

//     /// Expected this character to be either a `','` or a `'}'`.
//     ExpectedObjectCommaOrEnd,

//     /// Expected to parse either a `true`, `false`, or a `null`.
//     ExpectedSomeIdent,

//     /// Expected this character to start a JSON value.
//     ExpectedSomeValue,

//     /// Invalid hex escape code.
//     InvalidEscape,

//     /// Invalid number.
//     InvalidNumber,

//     /// Number is bigger than the maximum value of its type.
//     NumberOutOfRange,

//     /// Invalid unicode code point.
//     InvalidUnicodeCodePoint,

//     /// Control character found while parsing a string.
//     ControlCharacterWhileParsingString,

//     /// Object key is not a string.
//     KeyMustBeAString,

//     /// Lone leading surrogate in hex escape.
//     LoneLeadingSurrogateInHexEscape,

//     /// JSON has a comma after the last value in an array or map.
//     TrailingComma,

//     /// JSON has non-whitespace trailing characters after the value.
//     TrailingCharacters,

//     /// Unexpected end of hex escape.
//     UnexpectedEndOfHexEscape,

//     /// Encountered nesting of JSON maps and arrays more than 128 layers deep.
//     RecursionLimitExceeded,
// }

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
