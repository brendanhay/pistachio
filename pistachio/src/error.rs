use std::{
    convert::Infallible,
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

    /// Loading templates at runtime is disabled.
    LoadingDisabled,

    /// An attempt to include a partial or parent failed.
    InvalidPartial(String),

    /// An LR parser error occurred.
    ParsingFailed((usize, usize), String),

    /// A variable wasn't found on the stack and raising errors is enabled.
    MissingVariable((usize, usize), String),
}

impl Error {
    pub fn span(&self) -> Option<(usize, usize)> {
        match self {
            Error::ParsingFailed(span, _) => Some(*span),
            Error::MissingVariable(span, _) => Some(*span),
            _ => None,
        }
    }

    pub fn render_span(&self, source: &str) -> Option<String> {
        if source.is_empty() {
            return None;
        }

        let (start, end) = match self.span() {
            Some(span) => span,
            None => {
                return None;
            },
        };

        let mut position = 0;
        let mut line = 0;
        let mut indent = 0;

        for chr in source.chars() {
            position += 1;
            if chr == '\n' {
                line += 1;
                indent = 0;
            } else {
                indent += 1;
            }

            if position == start {
                break;
            }
        }

        let width = (start.max(end) - start).min(1);
        let span = "^".repeat(width);
        let mark = format!("{:indent$}{}", "", span, indent = indent);

        println!("mark: {}", mark.len());

        let mut lines = source.lines().collect::<Vec<_>>();
        lines.insert(line + 1, &mark);

        Some(lines.join("\n"))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => fmt::Display::fmt(err, f),
            Error::LoadingDisabled => {
                f.write_str("loading templates from the filesystem is disabled")
            },
            Error::InvalidPartial(msg) => write!(f, "partial path {} is invalid", msg),
            Error::ParsingFailed(_span, msg) => f.write_str(msg),
            Error::MissingVariable(span, ident) => write!(
                f,
                "missing variable `{{{{{}}}}}` at position {:?}",
                ident, span
            ),
        }
    }
}

impl From<ParseError<Token<'_>>> for Error {
    fn from(err: ParseError<Token<'_>>) -> Self {
        let msg = match &err {
            ParseError::User { .. } => String::new(),
            error => error.to_string(),
        };

        match err {
            ParseError::InvalidToken { location: start } => {
                Error::ParsingFailed((start, start), msg)
            },

            ParseError::UnrecognizedEOF {
                location: start, ..
            } => Error::ParsingFailed((start, start), msg),

            ParseError::UnrecognizedToken {
                token: (start, _token, end),
                ..
            } => Error::ParsingFailed((start, end), msg),

            ParseError::ExtraToken {
                token: (start, _token, end),
            } => Error::ParsingFailed((start, end), msg),

            ParseError::User { error } => error,
        }
    }
}
