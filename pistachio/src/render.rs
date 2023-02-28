use std::{
    borrow::{
        Borrow,
        Cow,
    },
    collections::HashMap,
    fmt,
    hash::{
        BuildHasher,
        Hash,
    },
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

pub use self::{
    context::Context,
    stack::Stack,
    writer::{
        WriteEscaped,
        Writer,
    },
};
use crate::Error;

#[cfg(feature = "json")]
mod json;

mod context;
mod lambda;
mod stack;
mod writer;

// Put the W on the trait allows boxing the trait object for Render<String>.
pub trait Render {
    #[inline]
    fn size_hint(&self) -> usize {
        0
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        true
    }

    #[inline]
    fn render_escaped(&self, _context: Context, _writer: &mut Writer) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn render_unescaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        self.render_escaped(context, writer)
    }

    #[inline]
    fn render_section(
        &self,
        _capture: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Error> {
        context.push(&self).render_to_writer(writer)
    }

    #[inline]
    fn resolve(&self, _key: &str) -> Option<&dyn Render> {
        None
    }
}

impl Render for () {
    #[inline]
    fn is_truthy(&self) -> bool {
        false
    }
}

impl Render for bool {
    #[inline]
    fn is_truthy(&self) -> bool {
        *self
    }

    #[inline]
    fn size_hint(&self) -> usize {
        5
    }

    #[inline]
    fn render_escaped(&self, _context: Context, writer: &mut Writer) -> Result<(), Error> {
        writer.write_escaped(if *self { "true" } else { "false" })
    }

    #[inline]
    fn render_section(
        &self,
        _capture: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Error> {
        context.render_to_writer(writer)
    }
}

impl Render for String {
    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn render_escaped(&self, _context: Context, writer: &mut Writer) -> Result<(), Error> {
        writer.write_escaped(self)
    }

    #[inline]
    fn render_unescaped(&self, _context: Context, writer: &mut Writer) -> Result<(), Error> {
        writer.write_unescaped(self)
    }
}

impl<T: Render> Render for Vec<T> {
    #[inline]
    fn is_truthy(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn render_section(
        &self,
        _capture: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Error> {
        for item in self.iter() {
            context.push(item).render_to_writer(writer)?;
        }

        Ok(())
    }
}

impl<K, V, H> Render for HashMap<K, V, H>
where
    K: Borrow<str> + Hash + Eq + fmt::Debug,
    V: Render,
    H: BuildHasher,
{
    #[inline]
    fn is_truthy(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn resolve(&self, key: &str) -> Option<&dyn Render> {
        self.get(key).map(|v| v as &dyn Render)
    }
}

macro_rules! impl_numbers {
    ( $( $ty:ty ),* ) => {
        $(
            impl Render for $ty {
                #[inline]
                fn size_hint(&self) -> usize {
                    5
                }

                #[inline]
                fn is_truthy(&self) -> bool {
                    *self != 0 as $ty
                }

                #[inline]
                fn render_escaped(
                    &self,
                    _context: Context,
                    writer: &mut Writer
                ) -> Result<(), Error> {
                    writer.write_unescaped(itoa::Buffer::new().format(*self))
                }
            }
        )*
    };
}

impl_numbers!(u8, u16, u32, u64, u128, usize);
impl_numbers!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_float {
    ( $($ty:ty : $epsilon:path),* ) => {
        $(
            impl Render for $ty  {
                #[inline]
                fn size_hint(&self) -> usize {
                    5
                }

                #[inline]
                fn is_truthy(&self) -> bool {
                    self.abs() > $epsilon
                }

                #[inline]
                fn render_escaped(
                    &self,
                    _context: Context,
                    writer: &mut Writer
                ) -> Result<(), Error> {
                    writer.write_unescaped(ryu::Buffer::new().format(*self))
                }
            }
        )*
    };
}

impl_float!(f32: f32::EPSILON, f64: f64::EPSILON);

macro_rules! impl_pointers {
    ( $($ty:ty $(: $bounds:ident)?),* ) => {
        $(
            impl<T: Render $(+ $bounds)? + ?Sized> Render for $ty {
                #[inline]
                fn size_hint(&self) -> usize {
                    self.deref().size_hint()
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
                    capture: &str,
                    context: Context,
                    writer: &mut Writer,
                ) -> Result<(), Error> {
                    self.deref().render_section(capture, context, writer)
                }

                #[inline]
                fn resolve(&self, key: &str) -> Option<&dyn Render> {
                    self.deref().resolve(key)
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
