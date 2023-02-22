use crate::{
    error::Error,
    render::{
        Context,
        Render,
        Writer,
    },
    Template,
};

impl Render for () {
    #[inline]
    fn is_truthy(&self) -> bool {
        false
    }
}

impl Render for bool {
    #[inline]
    fn is_truthy(&self) -> bool {
        *self
    }

    #[inline]
    fn size_hint(&self, _template: &Template) -> usize {
        5
    }

    #[inline]
    fn render_escaped(&self, _context: Context, writer: &mut Writer) -> Result<(), Error> {
        writer.write_escaped(if *self { "true" } else { "false" })
    }
}

impl Render for String {
    #[inline]
    fn size_hint(&self, _template: &Template) -> usize {
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

impl Render for str {
    #[inline]
    fn size_hint(&self, _template: &Template) -> usize {
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
