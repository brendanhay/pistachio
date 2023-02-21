use std::{
    borrow::Cow,
    convert::Infallible,
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

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

macro_rules! impl_pointers {
    ($( $ty:ty $(: $bounds:ident)? ),*) => {
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
                fn render(
                    &self,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<(), Infallible> {
                    self.deref().render(context, writer)
                }

                #[inline]
                fn render_named(
                    &self,
                    key: &str,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<bool, Infallible> {
                    self.deref().render_named(key, context, writer)
                }

                #[inline]
                fn render_section(
                    &self,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<(), Infallible> {
                    self.deref().render_section(context, writer)
                }

                #[inline]
                fn render_named_section(
                    &self,
                    key: &str,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<bool, Infallible> {
                    self.deref().render_named_section(key, context, writer)
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
