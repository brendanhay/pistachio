use std::{
    borrow::Cow,
    fmt,
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

use crate::{
    error::Error,
    render::{
        Context,
        Render,
        Writer,
    },
    Template,
};

macro_rules! impl_pointers {
    ( $($ty:ty $(: $bounds:ident)?),* ) => {
        $(
            impl<T: Render $(+ $bounds)? + ?Sized> Render for $ty {
                #[inline]
                fn size_hint(&self, template: &Template) -> usize {
                    self.deref().size_hint(template)
                }

                #[inline]
                fn is_truthy(&self) -> bool {
                    self.deref().is_truthy()
                }

                #[inline]
                fn render_escaped(
                    &self,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<(), Error> {
                    self.deref().render_escaped(context, writer)
                }

                #[inline]
                fn render_unescaped(
                    &self,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<(), Error> {
                    self.deref().render_unescaped(context, writer)
                }

                #[inline]
                fn render_section(
                    &self,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<(), Error> {
                    self.deref().render_section(context, writer)
                }

                #[inline]
                fn render_inverted(
                    &self,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<(), Error> {
                    self.deref().render_inverted(context, writer)
                }

                #[inline]
                fn render_field_escaped(
                    &self,
                    key: &str,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<bool, Error> {
                    self.deref().render_field_escaped(key, context, writer)
                }

                #[inline]
                fn render_field_unescaped(
                    &self,
                    key: &str,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<bool, Error> {
                    self.deref().render_field_unescaped(key, context, writer)
                }

                #[inline]
                fn render_field_section(
                    &self,
                    key: &str,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<bool, Error> {
                    self.deref().render_field_section(key, context, writer)
                }

                #[inline]
                fn render_field_inverted(
                    &self,
                    key: &str,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<bool, Error> {
                    self.deref().render_field_inverted(key, context, writer)
                }
            }
        )*
    }
}

impl_pointers! {
    &T,
    Box<T>,
    Rc<T>,
    Arc<T>
    // Cow<'_, T>: ToOwned
}
