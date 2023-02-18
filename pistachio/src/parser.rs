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
    ($predicate:expr, $open:expr, $path:expr) => {
        if !$predicate {
            return Err(crate::parser::ParseError::User {
                error: crate::error::Error::Parser(Box::from(format!(
                    "{{{{{open}{path}}}}} is missing the corresponding {{{{/{path}}}}} end tag",
                    open = $open,
                    path = $path,
                ))),
            });
        }
    };
}

pub(crate) use balanced;
