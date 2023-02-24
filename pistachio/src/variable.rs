use std::{
    borrow::Cow,
    fmt,
};

pub use self::ser::to_variable;
use crate::{
    Context,
    Error,
    Map,
    Render,
    Writer,
};

mod ser;

// pub enum Number {
//     Positive(u64),
//     /// Always less than zero.
//     Negative(i64),
//     /// Always finite.
//     Float(f64),
// }

#[derive(Debug)]
pub enum Lit {
    Bool(bool),
    // Number(Number),
    String(String),
}

impl Render for Lit {
    #[inline]
    fn size_hint(&self) -> usize {
        match self {
            Lit::Bool(b) => b.size_hint(),
            Lit::String(s) => s.size_hint(),
        }
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        true
    }

    #[inline]
    fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        match self {
            Lit::Bool(b) => b.render_escaped(context, writer),
            Lit::String(s) => s.render_escaped(context, writer),
        }
    }

    #[inline]
    fn render_unescaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        match self {
            Lit::Bool(b) => b.render_unescaped(context, writer),
            Lit::String(s) => s.render_unescaped(context, writer),
        }
    }
}

/// `{{baz("string", 2.3, true, ident_var)}}`
#[derive(Debug)]
pub enum Arg {
    Lit(Lit),
    Var(String),
}

pub type Return = Box<dyn Render + Send>;

pub type Lambda = Box<dyn Fn(Vec<Arg>) -> Result<Return, Error>>;

#[derive(Default)]
pub enum Var {
    #[default]
    Null,
    Lit(Lit),
    Vec(Vec<Var>),
    Map(Map<Cow<'static, str>, Var>),
    Fun(Lambda),
}

impl Default for &Var {
    fn default() -> Self {
        &Var::Null
    }
}

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Var::Null => f.write_str("Null"),
            Var::Lit(l) => write!(f, "Lit({:?})", l),
            Var::Vec(v) => write!(f, "Vec({:?})", v),
            Var::Map(m) => write!(f, "Map({:?})", m),
            Var::Fun(_) => f.write_str("Fun(..)"),
        }
    }
}

impl Var {
    #[inline]
    pub fn size_hint(&self) -> usize {
        match self {
            Var::Null => 0,
            Var::Lit(l) => l.size_hint(),
            Var::Vec(v) => v.iter().map(|x| x.size_hint()).sum::<usize>(),
            Var::Map(m) => m.values().map(|x| x.size_hint()).sum::<usize>(),
            Var::Fun(_) => 0,
        }
    }

    #[inline]
    pub fn is_truthy(&self) -> bool {
        match self {
            Var::Null => false,
            Var::Lit(l) => l.is_truthy(),
            Var::Vec(v) => !v.is_empty(),
            Var::Map(m) => !m.is_empty(),
            Var::Fun(_) => true,
        }
    }
}
