pub use self::{
    context::Context,
    ser::{
        to_variable,
        Variable,
    },
    writer::{
        WriteEscaped,
        Writer,
    },
};
use crate::error::Error;

mod context;
mod ser;
// mod stack;
// mod value;
mod stack;
mod writer;

pub trait Render {
    #[inline]
    fn size_hint(&self) -> usize {
        0
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        true
    }

    #[inline]
    fn render_escaped<W: WriteEscaped>(&self, _writer: &mut W) -> Result<(), Error> {
        // XXX: what about erroring by default - this way trying to use
        // something like {{ foo.bar.baz }} where baz is actually a lambda, will error.
        Ok(())
    }

    #[inline]
    fn render_unescaped<W: WriteEscaped>(&self, writer: &mut W) -> Result<(), Error> {
        // XXX: what about erroring by default - this way trying to use
        // something like {{ foo.bar.baz }} where baz is actually a lambda, will error.
        self.render_escaped(writer)
    }
}

impl Render for bool {
    #[inline]
    fn is_truthy(&self) -> bool {
        *self
    }

    #[inline]
    fn size_hint(&self) -> usize {
        5
    }

    #[inline]
    fn render_escaped<W: WriteEscaped>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_escaped(if *self { "true" } else { "false" })
    }
}

impl Render for String {
    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn render_escaped<W: WriteEscaped>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_escaped(self)
    }

    #[inline]
    fn render_unescaped<W: WriteEscaped>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_unescaped(self)
    }
}

impl Variable {
    #[inline]
    pub fn size_hint(&self) -> usize {
        match self {
            Variable::Null => 0,
            Variable::Bool(b) => b.size_hint(),
            Variable::Number(n) => n.size_hint(),
            Variable::String(s) => s.size_hint(),
            Variable::Vec(v) => v.iter().map(|x| x.size_hint()).sum::<usize>(),
            Variable::Map(m) => m.values().map(|x| x.size_hint()).sum::<usize>(),
        }
    }

    #[inline]
    pub fn is_truthy(&self) -> bool {
        match self {
            Variable::Null => false,
            Variable::Bool(b) => b.is_truthy(),
            Variable::Number(n) => n.is_truthy(),
            Variable::String(s) => s.is_truthy(),
            Variable::Vec(v) => !v.is_empty(),
            Variable::Map(m) => !m.is_empty(),
        }
    }
}
