pub use self::{
    context::Context,
    stack::Stack,
    value::Source,
    writer::Writer,
};
use crate::{
    error::Error,
    template::{
        Key,
        Template,
    },
};

mod context;
mod stack;
mod value;
mod writer;

pub trait Render {
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
        // XXX: what about erroring by default - this way trying to use
        // something like {{ foo.bar.baz }} where baz is actually a lambda, will error.
        Ok(())
    }
    fn render_unescaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        // XXX: what about erroring by default - this way trying to use
        // something like {{ foo.bar.baz }} where baz is actually a lambda, will error.
        self.render_escaped(context, writer)
    }

    #[inline]
    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        if self.is_truthy() {
            context.push(&self).render_to_writer(writer)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn render_inverted(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        if !self.is_truthy() {
            context.push(&self).render_to_writer(writer)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn render_field_escaped(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        if key == Key::DOT {
            self.render_escaped(context, writer).map(|_| true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    fn render_field_unescaped(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        if key == Key::DOT {
            self.render_unescaped(context, writer).map(|_| true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    fn render_field_section(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        if key == Key::DOT {
            self.render_section(context, writer).map(|_| true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    fn render_field_inverted(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        if key == Key::DOT {
            self.render_inverted(context, writer).map(|_| true)
        } else {
            Ok(false)
        }
    }
}
