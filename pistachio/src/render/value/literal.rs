use crate::{
    render::{
        writer::Writer,
        Escape,
        Render,
    },
    Template,
};

impl Render for () {
    #[inline]
    fn is_truthy(&self) -> bool {
        false
    }
}

impl Render for String {
    #[inline]
    fn is_truthy(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn size_hint(&self, _template: &Template) -> usize {
        self.len()
    }

    #[inline]
    fn render_escape<W: Writer>(&self, escape: Escape, writer: &mut W) -> Result<(), W::Error> {
        writer.write_escape(self, escape)
    }
}

impl Render for str {
    #[inline]
    fn is_truthy(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn size_hint(&self, _template: &Template) -> usize {
        self.len()
    }

    #[inline]
    fn render_escape<W: Writer>(&self, escape: Escape, writer: &mut W) -> Result<(), W::Error> {
        writer.write_escape(self, escape)
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
    fn render_escape<W: Writer>(&self, escape: Escape, writer: &mut W) -> Result<(), W::Error> {
        writer.write_escape(if *self { "true" } else { "false" }, escape)
    }
}
