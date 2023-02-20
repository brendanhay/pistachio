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

// XXX: doesn't consider custom delimiters
macro_rules! balanced {
    ($token:literal, $open:expr, $close:expr, $action:expr) => {
        if $open == $close {
            $action
        } else {
            Err(crate::parser::ParseError::User {
                error: crate::error::Error::Parser(Box::from(format!(
                    "{{{{{token}{open}}}}} is missing the corresponding {{{{/{close}}}}} close tag",
                    token = $token,
                    open = $open,
                    close = $close,
                ))),
            })
        }
    };
}

pub(crate) use balanced;
