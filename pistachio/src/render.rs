pub use self::{
    context::Context,
    stack::RenderStack,
    writer::{
        EscapedString,
        EscapedWriter,
        Writer,
    },
};
use crate::template::Template;

mod context;
mod stack;
mod writer;

pub(crate) mod value;

#[derive(Debug, Clone, Copy)]
pub enum Escape {
    Html,
    None,
}

pub trait Render {
    #[inline]
    fn is_truthy(&self) -> bool {
        true
    }

    #[inline]
    fn size_hint(&self, _template: &Template) -> usize {
        0
    }

    #[inline]
    fn render_escape<W: Writer>(&self, _escape: Escape, _writer: &mut W) -> Result<(), W::Error> {
        Ok(())
    }

    #[inline]
    fn render_section<S, W>(&self, context: Context<S>, writer: &mut W) -> Result<(), W::Error>
    where
        S: RenderStack,
        W: Writer,
    {
        if self.is_truthy() {
            context.render(writer)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn render_inverted_section<S, W>(
        &self,
        context: Context<S>,
        writer: &mut W,
    ) -> Result<(), W::Error>
    where
        S: RenderStack,
        W: Writer,
    {
        if !self.is_truthy() {
            context.render(writer)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn render_field_escape<W: Writer>(
        &self,
        _key: &str,
        _escape: Escape,
        _writer: &mut W,
    ) -> Result<bool, W::Error> {
        Ok(false)
    }

    #[inline]
    fn render_field_section<S, W>(
        &self,
        _key: &str,
        _context: Context<S>,
        _writer: &mut W,
    ) -> Result<bool, W::Error>
    where
        S: RenderStack,
        W: Writer,
    {
        Ok(false)
    }

    #[inline]
    fn render_field_inverted_section<S, W>(
        &self,
        _key: &str,
        _context: Context<S>,
        _writer: &mut W,
    ) -> Result<bool, W::Error>
    where
        S: RenderStack,
        W: Writer,
    {
        Ok(false)
    }
}
