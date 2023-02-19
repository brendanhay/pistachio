use std::{
    borrow::Cow,
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

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

macro_rules! impl_pointers {
    ($( $ty:ty $(: $bounds:ident)? ),*) => {
        $(
            impl<T: Render $(+ $bounds)? + ?Sized> Render for $ty {
                #[inline]
                fn is_truthy(&self) -> bool {
                    self.deref().is_truthy()
                }

                #[inline]
                fn size_hint(&self, template: &Template) -> usize {
                    self.deref().size_hint(template)
                }

                #[inline]
                fn render_escape<W: Writer>(&self, escape: Escape, writer: &mut W) -> Result<(), RenderError<W::Error>> {
                    self.deref().render_escape(escape, writer)
                }

                #[inline]
                fn render_section<S, W>(
                    &self,
                    context: Context<S>,
                    writer: &mut W,
                ) -> Result<(), RenderError<W::Error>>
                where
                    S: RenderStack,
                    W: Writer,
                {
                    self.deref().render_section(context, writer)
                }

                #[inline]
                fn render_inverted_section<S, W>(
                    &self,
                    context: Context<S>,
                    writer: &mut W,
                ) -> Result<(), RenderError<W::Error>>
                where
                    S: RenderStack,
                    W: Writer,
                {
                    self.deref().render_inverted_section(context, writer)
                }

                #[inline]
                fn render_field_escape<W: Writer>(
                    &self,
                    key: &str,
                    escape: Escape,
                    writer: &mut W,
                ) -> Result<bool, RenderError<W::Error>> {
                    self.deref().render_field_escape(key, escape, writer)
                }

                #[inline]
                fn render_field_section<S, W>(
                    &self,
                    key: &str,
                    context: Context<S>,
                    writer: &mut W,
                ) -> Result<bool, RenderError<W::Error>>
                where
                    S: RenderStack,
                    W: Writer,
                {
                    self.deref().render_field_section(key, context, writer)
                }

                #[inline]
                fn render_field_inverted_section<S, W>(
                    &self,
                    key: &str,
                    context: Context<S>,
                    writer: &mut W,
                ) -> Result<bool, RenderError<W::Error>>
                where
                    S: RenderStack,
                    W: Writer,
                {
                    self.deref().render_field_inverted_section(key, context, writer)
                }
            }
        )*
    }
}

impl_pointers! {
    &T,
    Box<T>,
    Rc<T>,
    Arc<T>,
    Cow<'_, T>: ToOwned
}
