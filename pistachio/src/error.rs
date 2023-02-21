use std::{
    error,
    fmt,
    io,
    result,
};

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    info: Box<ErrorInfo>,
}

impl Error {
    pub(crate) fn syntax(kind: ErrorKind, line: usize, column: usize) -> Self {
        Error {
            info: Box::new(ErrorInfo { kind, line, column }),
        }
    }

    pub(crate) fn io(error: io::Error) -> Self {
        Error {
            info: Box::new(ErrorInfo {
                kind: ErrorKind::Io(error),
                line: 0,
                column: 0,
            }),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error({:?}, line: {}, column: {})",
            self.info.code.to_string(),
            self.info.line,
            self.info.column
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&*self.info, f)
    }
}

struct ErrorInfo {
    kind: ErrorKind,
    line: usize,
    column: usize,
}

impl fmt::Display for ErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.line == 0 {
            fmt::Display::fmt(&self.kind, f)
        } else {
            write!(
                f,
                "{} at line {} column {}",
                self.kind, self.line, self.column
            )
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    /// Catchall for syntax error messages
    Message(Box<str>),

    /// An IO error occurred while parsing or rendering.
    Io(io::Error),

    /// Loading templates at runtime is disabled.
    LoadingDisabled,

    /// An attempt to include a partial or parent failed.
    InvalidPartial,

    /// The specified template wasn't found.
    NotFound,

    /// Tried to serialize a map key that was not a string.
    KeyMustBeAString,

    /// Tried to serialize a number bigger than the maximum allowable value for its type.
    NumberOutOfRange,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::Message(msg) => f.write_str(msg),
            ErrorKind::Io(err) => fmt::Display::fmt(err, f),
            ErrorKind::NumberOutOfRange => f.write_str("number out of range"),
            ErrorKind::KeyMustBeAString => f.write_str("key must be a string"),
            ErrorKind::LoadingDisabled => todo!(),
            ErrorKind::InvalidPartial => todo!(),
            ErrorKind::NotFound => todo!(),
        }
    }
}
