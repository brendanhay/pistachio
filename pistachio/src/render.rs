use std::fmt;

pub use self::{
    context::Context,
    stack::Stack,
    writer::Writer,
};
use crate::{
    error::Error,
    template::Template,
};

mod context;
mod ser;
mod stack;
mod value;
mod writer;

pub trait Render: fmt::Debug {
    #[inline]
    fn size_hint(&self, _template: &Template) -> usize {
        0
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        true
    }

    #[inline]
    fn render_escaped(&self, _context: Context, _writer: &mut Writer) -> Result<(), Error> {
        println!("default:render_escaped {:?}", self);

        // XXX: what about erroring by default - this way trying to use
        // something like {{ foo.bar.baz }} where baz is actually a lambda, will error.
        Ok(())
    }

    #[inline]
    fn render_unescaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        println!("default:render_unescaped {:?}", self);

        // XXX: what about erroring by default - this way trying to use
        // something like {{ foo.bar.baz }} where baz is actually a lambda, will error.
        self.render_escaped(context, writer)
    }

    #[inline]
    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        println!("default:render_section {:?}", self);

        if self.is_truthy() {
            context.render_to_writer(writer)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn render_inverted(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        println!("default:render_inverted {:?}", self);

        if !self.is_truthy() {
            context.render_to_writer(writer)
        } else {
            Ok(())
        }
    }

    /// XXX: throw missing variable errors by default?

    #[inline]
    fn render_named_escaped(
        &self,
        name: &[&str],
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        // If the name is empty the stack is fully prepared.
        if name.is_empty() {
            self.render_escaped(context, writer).map(|_| true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    fn render_named_unescaped(
        &self,
        name: &[&str],
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        // If the name is empty the stack is fully prepared.
        if name.is_empty() {
            self.render_unescaped(context, writer).map(|_| true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    fn render_named_section(
        &self,
        name: &[&str],
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        // If the name is empty the stack is fully prepared.
        if name.is_empty() {
            self.render_section(context, writer).map(|_| true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    fn render_named_inverted(
        &self,
        name: &[&str],
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        // If the name is empty the stack is fully prepared.
        if name.is_empty() {
            self.render_inverted(context, writer).map(|_| true)
        } else {
            Ok(false)
        }
    }
}
