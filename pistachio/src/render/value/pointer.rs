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
                fn variable(
                    &self,
                    escape: Escape,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<(), Infallible> {
                    self.deref().variable(escape, context, writer)
                }

                #[inline]
                fn variable_key(
                    &self,
                    key: &str,
                    escape: Escape,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<bool, Infallible> {
                    self.deref().variable_key(key, escape, context, writer)
                }

                #[inline]
                fn section(
                    &self,
                    section: Section,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<(), Infallible> {
                    self.deref().section(section, context, writer)
                }

                #[inline]
                fn section_key(
                    &self,
                    key: &str,
                    section: Section,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<bool, Infallible> {
                    self.deref().section_key(key, section, context, writer)
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
