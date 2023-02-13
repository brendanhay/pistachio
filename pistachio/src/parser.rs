use lalrpop_util::lalrpop_mod;

pub use self::{
    lexer::Lexer,
    token::Token,
};
use crate::error::Error;

mod lexer;
mod rule;
mod token;

lalrpop_mod!(grammar, "/parser/grammar.rs");

pub type ParseError<T, E = Error> = lalrpop_util::ParseError<usize, T, E>;

pub type Parser = grammar::MustacheParser;
