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

use crate::render::{
    stack,
    writer::Writer,
    Context,
    Escape,
    Render,
    RenderError,
};

macro_rules! impl_map {
    () => {
        fn is_truthy(&self) -> bool {
            !self.is_empty()
        }

        #[inline]
        fn render_section<S, W>(
            &self,
            key: &str,
            context: Context<S>,
            writer: &mut W,
        ) -> Result<(), RenderError<W::Error>>
        where
            S: stack::RenderStack,
            W: Writer,
        {
            if self.is_truthy() {
                context.push(key, self).render(writer)
            } else {
                Ok(())
            }
        }

        #[inline]
        fn render_field_escape<W>(
            &self,
            key: &str,
            escape: Escape,
            writer: &mut W,
        ) -> Result<bool, RenderError<W::Error>>
        where
            W: Writer,
        {
            match self.get(key) {
                Some(v) => v.render_escape(escape, writer).map(|_| true),
                None => Ok(false),
            }
        }

        #[inline]
        fn render_field_section<S, W>(
            &self,
            key: &str,
            context: Context<S>,
            writer: &mut W,
        ) -> Result<bool, RenderError<W::Error>>
        where
            S: stack::RenderStack,
            W: Writer,
        {
            match self.get(key) {
                Some(v) => v.render_section(key, context, writer).map(|_| true),
                None => Ok(false),
            }
        }

        #[inline]
        fn render_field_inverted_section<S, W>(
            &self,
            key: &str,
            context: Context<S>,
            writer: &mut W,
        ) -> Result<bool, RenderError<W::Error>>
        where
            S: stack::RenderStack,
            W: Writer,
        {
            match self.get(key) {
                Some(v) => v
                    .render_inverted_section(key, context, writer)
                    .map(|_| true),
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
