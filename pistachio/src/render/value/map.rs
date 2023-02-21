use std::{
    borrow::Borrow,
    collections::{
        BTreeMap,
        HashMap,
    },
    convert::Infallible,
    hash::{
        BuildHasher,
        Hash,
    },
};

use crate::render::{
    Context,
    Render,
    Writer,
};

macro_rules! impl_map {
    () => {
        #[inline]
        fn is_truthy(&self) -> bool {
            !self.is_empty()
        }

        #[inline]
        fn render_named(
            &self,
            key: &str,
            context: Context,
            writer: &mut Writer,
        ) -> Result<bool, Infallible> {
            match self.get(key) {
                Some(v) => v.render(context, writer).map(|_| true),
                None => Ok(false),
            }
        }

        #[inline]
        fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Infallible> {
            if self.section_is_truthy(context.section) {
                context.push(self).render(writer)
            } else {
                Ok(())
            }
        }

        #[inline]
        fn render_named_section(
            &self,
            key: &str,
            context: Context,
            writer: &mut Writer,
        ) -> Result<bool, Infallible> {
            match self.get(key) {
                Some(v) => v.render_section(context, writer).map(|_| true),
                None => Ok(false),
            }
        }
    };
}

pub(crate) use impl_map;

impl<K, V, H> Render for HashMap<K, V, H>
where
    K: Borrow<str> + Hash + Eq,
    V: Render,
    H: BuildHasher,
{
    impl_map! {}
}

impl<K, V> Render for BTreeMap<K, V>
where
    K: Borrow<str> + Ord,
    V: Render,
{
    impl_map! {}
}
