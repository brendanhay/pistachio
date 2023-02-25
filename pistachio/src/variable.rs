use std::{
    borrow::Cow,
    fmt,
};

// pub use self::ser::to_variable;
use crate::{
    Context,
    Error,
    Map,
    Render,
    Writer,
};

// mod ser;

pub type Var<'a> = &'a (dyn Render + 'a);

// pub enum Number {
//     Positive(u64),
//     /// Always less than zero.
//     Negative(i64),
//     /// Always finite.
//     Float(f64),
// }

// #[derive(Debug)]
// pub enum Lit {
//     Bool(bool),
//     // Number(Number),
//     String(String),
// }

// /// `{{baz("string", 2.3, true, ident_var)}}`
// #[derive(Debug)]
// pub enum Arg {
//     Bool(bool),
//     Number(String),
//     String(String),
//     Var(String),
// }

// pub type Return = Box<dyn Render + Send>;

// pub type Lambda = Box<dyn Fn(Vec<Arg>) -> Result<Return, Error>>;

// #[derive(Default)]
// pub enum Var {
//     #[default]
//     Null,
//     Bool(bool),
//     Number(String),
//     String(String),
//     Vec(Vec<Var>),
//     Map(Map<Cow<'static, str>, Var>),
//     Fun(Lambda),
// }

// impl Default for &Var {
//     fn default() -> Self {
//         &Var::Null
//     }
// }

// impl fmt::Debug for Var {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Var::Null => f.write_str("Null"),
//             Var::Bool(b) => write!(f, "Bool({:?})", b),
//             Var::Number(n) => write!(f, "Number({:?})", n),
//             Var::String(s) => write!(f, "String({:?})", s),
//             Var::Vec(v) => write!(f, "Vec({:?})", v),
//             Var::Map(m) => write!(f, "Map({:?})", m),
//             Var::Fun(_) => f.write_str("Fun(..)"),
//         }
//     }
// }

// impl Var {
//     #[inline]
//     pub fn size_hint(&self) -> usize {
//         match self {
//             Var::Null => 0,
//             Var::Lit(l) => l.size_hint(),
//             Var::Vec(v) => v.iter().map(|x| x.size_hint()).sum::<usize>(),
//             Var::Map(m) => m.values().map(|x| x.size_hint()).sum::<usize>(),
//             Var::Fun(_) => 0,
//         }
//     }

//     #[inline]
//     pub fn is_truthy(&self) -> bool {
//         match self {
//             Var::Null => false,
//             Var::Lit(l) => l.is_truthy(),
//             Var::Vec(v) => !v.is_empty(),
//             Var::Map(m) => !m.is_empty(),
//             Var::Fun(_) => true,
//         }
//     }
// }
