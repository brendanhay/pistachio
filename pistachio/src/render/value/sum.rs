use crate::{
    error::Error,
    render::{
        Context,
        Render,
        Writer,
    },
    Template,
};

impl<T: Render> Render for Option<T> {
    #[inline]
    fn size_hint(&self, template: &Template) -> usize {
        match self {
            Some(inner) => inner.size_hint(template),
            _ => 0,
        }
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        self.is_some()
    }

    #[inline]
    fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        if let Some(inner) = self {
            inner.render_escaped(context, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn render_unescaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        if let Some(inner) = self {
            inner.render_unescaped(context, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        if let Some(item) = self {
            item.render_section(context, writer)?;
        }

        Ok(())
    }
}

impl<T: Render, E> Render for Result<T, E> {
    #[inline]
    fn size_hint(&self, template: &Template) -> usize {
        match self {
            Ok(inner) => inner.size_hint(template),
            _ => 0,
        }
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        self.is_ok()
    }

    #[inline]
    fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        if let Ok(inner) = self {
            inner.render_escaped(context, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn render_unescaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        if let Ok(inner) = self {
            inner.render_unescaped(context, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        if let Ok(item) = self {
            item.render_section(context, writer)?;
        }

        Ok(())
    }
}
