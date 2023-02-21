use std::convert::Infallible;

pub use self::{
    context::Context,
    stack::Stack,
    value::Source,
    writer::Writer,
};
use crate::template::Template;

mod context;
mod stack;
mod value;
mod writer;

#[derive(Debug, Clone, Copy)]
pub enum Section {
    Positive,
    Negative,
}

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
    fn section_is_truthy(&self, section: Section) -> bool {
        let truthy = self.is_truthy();
        match section {
            Section::Positive => truthy,
            Section::Negative => !truthy,
        }
    }

    #[inline]
    fn render(&self, _context: Context, _writer: &mut Writer) -> Result<(), Infallible> {
        // XXX: what about erroring by default - this way trying to use
        // something like {{ foo.bar.baz }} where baz is actually a lambda, will error.
        Ok(())
    }

    #[inline]
    fn render_named(
        &self,
        _key: &str,
        _context: Context,
        _writer: &mut Writer,
    ) -> Result<bool, Infallible> {
        Ok(false)
    }

    #[inline]
    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Infallible> {
        if self.section_is_truthy(context.section) {
            context.push(&self).render(writer)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn render_named_section(
        &self,
        _key: &str,
        _context: Context,
        _writer: &mut Writer,
    ) -> Result<bool, Infallible> {
        Ok(false)
    }
}
