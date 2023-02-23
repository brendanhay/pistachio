use std::fmt;

use super::{
    Context,
    Writer,
};
use crate::{
    error::Error,
    render::Render,
    template::Key,
};

#[derive(Debug, Clone, Copy)]
pub struct Stack<'a> {
    a: &'a (dyn Render),
    b: &'a (dyn Render),
    c: &'a (dyn Render),
    d: &'a (dyn Render),
    e: &'a (dyn Render),
    f: &'a (dyn Render),
}

// impl fmt::Debug for Stack<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Stack")
//     }
// }

impl<'a> Stack<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            a: &(),
            b: &(),
            c: &(),
            d: &(),
            e: &(),
            f: &(),
        }
    }

    #[inline]
    pub fn push(self, frame: &'a (dyn Render)) -> Self {
        Self {
            a: frame,
            b: self.a,
            c: self.b,
            d: self.c,
            e: self.d,
            f: self.e,
        }
    }

    #[inline]
    pub fn pop(self) -> Self {
        Self {
            a: self.b,
            b: self.c,
            c: self.d,
            d: self.e,
            e: self.f,
            f: &(),
        }
    }

    #[inline]
    pub fn render_field_escaped(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        if key == Key::DOT {
            self.a.render_escaped(context, writer).map(|_| true)
        } else {
            if self.a.render_field_escaped(key, context, writer)?
                || self.b.render_field_escaped(key, context, writer)?
                || self.c.render_field_escaped(key, context, writer)?
                || self.d.render_field_escaped(key, context, writer)?
                || self.e.render_field_escaped(key, context, writer)?
                || self.f.render_field_escaped(key, context, writer)?
            {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    #[inline]
    pub fn render_field_unescaped(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        if key == Key::DOT {
            self.a.render_unescaped(context, writer).map(|_| true)
        } else {
            if self.a.render_field_unescaped(key, context, writer)?
                || self.b.render_field_unescaped(key, context, writer)?
                || self.c.render_field_unescaped(key, context, writer)?
                || self.d.render_field_unescaped(key, context, writer)?
                || self.e.render_field_unescaped(key, context, writer)?
                || self.f.render_field_unescaped(key, context, writer)?
            {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    #[inline]
    pub fn render_field_section(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Error> {
        if key == Key::DOT {
            self.a.render_section(context, writer)?;
        } else {
            if !self.a.render_field_section(key, context, writer)? {
                let context = context.pop();
                if !self.b.render_field_section(key, context, writer)? {
                    let context = context.pop();
                    if !self.c.render_field_section(key, context, writer)? {
                        let context = context.pop();
                        if !self.d.render_field_section(key, context, writer)? {
                            let context = context.pop();
                            if !self.e.render_field_section(key, context, writer)? {
                                let context = context.pop();
                                self.f.render_field_section(key, context, writer)?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    #[inline]
    pub fn render_field_inverted(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Error> {
        if key == Key::DOT {
            self.a.render_inverted(context, writer)?;
        } else {
            if !self.a.render_field_inverted(key, context, writer)?
                && !self.b.render_field_inverted(key, context, writer)?
                && !self.c.render_field_inverted(key, context, writer)?
                && !self.d.render_field_inverted(key, context, writer)?
                && !self.e.render_field_inverted(key, context, writer)?
                && !self.f.render_field_inverted(key, context, writer)?
            {
                context.render_to_writer(writer)?;
            }
        }

        Ok(())
    }
}
