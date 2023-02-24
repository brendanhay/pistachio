use std::{
    borrow::Borrow,
    collections::{
        BTreeMap,
        HashMap,
    },
    fmt,
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
            println!("map:render_section {:?}", self);

            if self.is_truthy() {
                context.push(self).render_to_writer(writer)
            } else {
                Ok(())
            }
        }

        #[inline]
        fn render_inverted(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
            println!("map:render_inverted {:?}", self);

            if !self.is_truthy() {
                context.push(self).render_to_writer(writer)
            } else {
                Ok(())
            }
        }

        #[inline]
        fn render_named_escaped(
            &self,
            name: &[&str],
            context: Context,
            writer: &mut Writer,
        ) -> Result<bool, Error> {
            println!("map:render_named_escaped {:?}", name);

            // If the name is empty the stack is fully prepared.
            if name.is_empty() {
                context.peek().render_escaped(context, writer).map(|_| true)
            } else {
                match self.get(name[0]) {
                    Some(v) => v.render_named_escaped(&name[1..], context.push(v), writer),
                    // For normal sections a missing key = failure.
                    None => Ok(false), // XXX: return Err(MissingVariable)
                }
            }
        }

        #[inline]
        fn render_named_unescaped(
            &self,
            name: &[&str],
            context: Context,
            writer: &mut Writer,
        ) -> Result<bool, Error> {
            // If the name is empty the stack is fully prepared.
            if name.is_empty() {
                context
                    .peek()
                    .render_unescaped(context, writer)
                    .map(|_| true)
            } else {
                match self.get(name[0]) {
                    Some(v) => v.render_named_unescaped(&name[1..], context.push(v), writer),
                    // For normal sections a missing key = failure.
                    None => Ok(false), // XXX: return Err(MissingVariable)
                }
            }
        }

        #[inline]
        fn render_named_section(
            &self,
            name: &[&str],
            context: Context,
            writer: &mut Writer,
        ) -> Result<bool, Error> {
            // If the name is empty the stack is fully prepared.
            if name.is_empty() {
                context.peek().render_section(context, writer).map(|_| true)
            } else {
                match self.get(name[0]) {
                    Some(v) => v.render_named_section(&name[1..], context.push(v), writer),
                    // For normal sections a missing key = failure.
                    None => Ok(false), // XXX: return Err(MissingVariable)
                }
            }
        }

        #[inline]
        fn render_named_inverted(
            &self,
            name: &[&str],
            context: Context,
            writer: &mut Writer,
        ) -> Result<bool, Error> {
            // If the name is empty the stack is fully prepared.
            if name.is_empty() {
                context
                    .peek()
                    .render_inverted(context, writer)
                    .map(|_| true)
            } else {
                match self.get(name[0]) {
                    Some(v) => v.render_named_inverted(&name[1..], context.push(v), writer),
                    // For inverted sections a missing key = success.
                    None => Ok(false),
                }
            }
        }
    };
}

pub(crate) use impl_map;

impl<K, V, H> Render for HashMap<K, V, H>
where
    K: Borrow<str> + Hash + Eq + fmt::Debug,
    V: Render,
    H: BuildHasher,
{
    impl_map! {}
}

impl<K, V> Render for BTreeMap<K, V>
where
    K: Borrow<str> + Ord + fmt::Debug,
    V: Render,
{
    impl_map! {}
}
