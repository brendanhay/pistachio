pub use self::{
    context::Context,
    stack::Stack,
    writer::{
        WriteEscaped,
        Writer,
    },
};
use crate::{
    Error,
    Template,
    Variable,
};

mod context;
mod stack;
mod writer;

// Put the W on the trait allows boxing the trait object for Render<String>.
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
    fn render_escaped(&self, _context: Context, _writer: &mut Writer) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn render_unescaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        self.render_escaped(context, writer)
    }

    #[inline]
    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        context.push(self).render_to_writer(writer)
    }
}

// #[derive(Debug)]
// pub struct Source {
//     pub source: String,
// }

// impl Render for Source {
//     #[inline]
//     fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
//         let template = Template::new(&self.source)?;
//         context.fork(&template).render_to_writer(writer)
//     }
// }

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
    fn render_escaped(&self, _context: Context, writer: &mut Writer) -> Result<(), Error> {
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
    fn render_escaped(&self, _context: Context, writer: &mut Writer) -> Result<(), Error> {
        writer.write_escaped(self)
    }

    #[inline]
    fn render_unescaped(&self, _context: Context, writer: &mut Writer) -> Result<(), Error> {
        writer.write_unescaped(self)
    }
}
