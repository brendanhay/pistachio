use crate::error::Error;
pub use crate::lexer::{
    Lexer,
    Token,
};

mod grammar {
    use lalrpop_util::lalrpop_mod;

    lalrpop_mod!(pub parser);
}

pub type ParseError<T, E = Error> = lalrpop_util::ParseError<usize, T, E>;

pub type Parser = grammar::parser::MustacheParser;

pub trait Spanned {
    fn span(&self) -> (usize, usize);
}

impl<T: Spanned> Spanned for &T {
    fn span(&self) -> (usize, usize) {
        (*self).span()
    }
}

impl Spanned for (usize, &str) {
    fn span(&self) -> (usize, usize) {
        (self.0, self.1.len())
    }
}

// XXX: doesn't consider custom delimiters
macro_rules! balanced {
    ($token:literal, $open:expr, $close:expr, $span:expr, $action:expr) => {
        if $open == $close {
            $action
        } else {
            let msg = format!(
                "{{{{{token}{open}}}}} is missing the corresponding {{{{/{close}}}}} close tag",
                token = $token,
                open = $open,
                close = $close,
            );

            Err(crate::parser::ParseError::User {
                error: crate::error::Error::ParsingFailed($span, msg),
            })
        }
    };
}

pub(crate) use balanced;
