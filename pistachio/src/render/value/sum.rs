use crate::{
    render::{
        stack::RenderStack,
        writer::Writer,
        Context,
        Escape,
        Render,
        RenderError,
    },
    Template,
};

impl<T: Render> Render for Option<T> {
    #[inline]
    fn is_truthy(&self) -> bool {
        self.is_some()
    }

    #[inline]
    fn size_hint(&self, template: &Template) -> usize {
        match self {
            Some(inner) => inner.size_hint(template),
            _ => 0,
        }
    }

    #[inline]
    fn variable<W: Writer>(
        &self,
        escape: Escape,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>> {
        if let Some(inner) = self {
            inner.variable(escape, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn section<S, W>(
        &self,
        context: Context<S>,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>>
    where
        S: RenderStack,
        W: Writer,
    {
        if let Some(item) = self {
            item.section(context, writer)?;
        }

        Ok(())
    }
}

impl<T: Render, E> Render for Result<T, E> {
    #[inline]
    fn is_truthy(&self) -> bool {
        self.is_ok()
    }

    #[inline]
    fn size_hint(&self, template: &Template) -> usize {
        match self {
            Ok(inner) => inner.size_hint(template),
            _ => 0,
        }
    }

    #[inline]
    fn variable<W: Writer>(
        &self,
        escape: Escape,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>> {
        if let Ok(inner) = self {
            inner.variable(escape, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn section<S, W>(
        &self,
        context: Context<S>,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>>
    where
        S: RenderStack,
        W: Writer,
    {
        if let Ok(item) = self {
            item.section(context, writer)?;
        }

        Ok(())
    }
}
