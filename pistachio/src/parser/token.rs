#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    /// An identifier - the single component of a key, no dots!
    Ident(&'a str),

    /// A string inside a tag.
    String(&'a str),

    /// Raw textual content outside any tags.
    Content(&'a str),

    /// `{{` tag start
    Enter(&'a str),

    /// `}}` tag end
    Leave(&'a str),

    /// `/`
    Slash,

    /// `#`
    Pound,

    /// `^`
    Caret,

    /// `>`
    Greater,

    /// `<`
    Less,

    /// `$`
    Dollar,

    /// `$`
    Bang,

    /// `&`
    Ampersand,

    /// `*`
    Asterisk,

    /// `.`
    Period,

    /// `=`
    Equals,
}

use std::fmt;

use Token::*;

impl Token<'_> {
    pub fn skip_whitespace(&self) -> bool {
        !matches!(self, Leave(..) | Period)
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Enter(s) => write!(f, "{}", s),
            Leave(s) => write!(f, "{}", s),
            Ident(s) => write!(f, "{}", s),
            String(s) => write!(f, "{}", s),
            Content(s) => write!(f, "{}", s),
            Slash => write!(f, "/"),
            Pound => write!(f, "#"),
            Caret => write!(f, "^"),
            Greater => write!(f, ">"),
            Less => write!(f, "<"),
            Dollar => write!(f, "$"),
            Bang => write!(f, "!"),
            Ampersand => write!(f, "&"),
            Asterisk => write!(f, "*"),
            Period => write!(f, "."),
            Equals => write!(f, "="),
        }
    }
}
