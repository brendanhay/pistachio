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
    Escape,
    Render,
    Section,
    Writer,
};

macro_rules! impl_map {
    () => {
        #[inline]
        fn is_truthy(&self) -> bool {
            !self.is_empty()
        }

        #[inline]
        fn variable_key(
            &self,
            key: &str,
            escape: Escape,
            context: Context,
            writer: &mut Writer,
        ) -> Result<bool, Infallible> {
            match self.get(key) {
                Some(v) => v.variable(escape, context, writer).map(|_| true),
                None => Ok(false),
            }
        }

        #[inline]
        fn section(
            &self,
            section: Section,
            context: Context,
            writer: &mut Writer,
        ) -> Result<(), Infallible> {
            if self.section_is_truthy(section) {
                context.push(self).render(writer)
            } else {
                Ok(())
            }
        }

        #[inline]
        fn section_key(
            &self,
            key: &str,
            section: Section,
            context: Context,
            writer: &mut Writer,
        ) -> Result<bool, Infallible> {
            match self.get(key) {
                Some(v) => v.section(section, context, writer).map(|_| true),
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
