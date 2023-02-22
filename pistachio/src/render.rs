pub use self::{
    context::Context,
    stack::Stack,
    value::Source,
    writer::Writer,
};
use crate::{
    error::Error,
    template::Template,
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
        _key: &str,
        _context: Context,
        _writer: &mut Writer,
    ) -> Result<bool, Error> {
        Ok(false)
    }

    #[inline]
    fn render_field_unescaped(
        &self,
        _key: &str,
        _context: Context,
        _writer: &mut Writer,
    ) -> Result<bool, Error> {
        Ok(false)
    }

    #[inline]
    fn render_field_section(
        &self,
        _key: &str,
        _context: Context,
        _writer: &mut Writer,
    ) -> Result<bool, Error> {
        Ok(false)
    }

    #[inline]
    fn render_field_inverted(
        &self,
        _key: &str,
        _context: Context,
        _writer: &mut Writer,
    ) -> Result<bool, Error> {
        Ok(false)
    }
}
