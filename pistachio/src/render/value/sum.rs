use std::convert::Infallible;

use crate::{
    render::{
        Context,
        Escape,
        Render,
        Section,
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
    fn variable(
        &self,
        escape: Escape,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Infallible> {
        if let Some(inner) = self {
            inner.variable(escape, context, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn section(
        &self,
        section: Section,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Infallible> {
        if let Some(item) = self {
            item.section(section, context, writer)?;
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
    fn variable(
        &self,
        escape: Escape,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Infallible> {
        if let Ok(inner) = self {
            inner.variable(escape, context, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn section(
        &self,
        section: Section,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Infallible> {
        if let Ok(item) = self {
            item.section(section, context, writer)?;
        }

        Ok(())
    }
}
