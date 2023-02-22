use std::{
    borrow::Borrow,
    collections::{
        BTreeMap,
        HashMap,
    },
    hash::{
        BuildHasher,
        Hash,
    },
};

use crate::{
    error::Error,
    render::{
        Context,
        Render,
        Writer,
    },
};

macro_rules! impl_map {
    () => {
        #[inline]
        fn is_truthy(&self) -> bool {
            !self.is_empty()
        }

        #[inline]
        fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
            if self.is_truthy() {
                context.push(self).render_to_writer(writer)
            } else {
                Ok(())
            }
        }

        #[inline]
        fn render_field_escaped(
            &self,
            key: &str,
            context: Context,
            writer: &mut Writer,
        ) -> Result<bool, Error> {
            match self.get(key) {
                Some(v) => v.render_escaped(context, writer).map(|_| true),
                None => Ok(false),
            }
        }

        #[inline]
        fn render_field_unescaped(
            &self,
            key: &str,
            context: Context,
            writer: &mut Writer,
        ) -> Result<bool, Error> {
            match self.get(key) {
                Some(v) => v.render_unescaped(context, writer).map(|_| true),
                None => Ok(false),
            }
        }

        #[inline]
        fn render_field_section(
            &self,
            key: &str,
            context: Context,
            writer: &mut Writer,
        ) -> Result<bool, Error> {
            match self.get(key) {
                Some(v) => v.render_section(context, writer).map(|_| true),
                None => Ok(false),
            }
        }

        #[inline]
        fn render_field_inverted(
            &self,
            key: &str,
            context: Context,
            writer: &mut Writer,
        ) -> Result<bool, Error> {
            match self.get(key) {
                Some(v) => v.render_inverted(context, writer).map(|_| true),
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
