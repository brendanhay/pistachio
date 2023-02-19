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
    fn render(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        writer.write(context.escape, self)
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
    fn render(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        writer.write(context.escape, self)
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
    fn render(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        writer.write(context.escape, if *self { "true" } else { "false" })
    }
}
