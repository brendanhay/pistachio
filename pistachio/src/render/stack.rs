use super::{
    Context,
    Writer,
};
use crate::{
    error::Error,
    render::Render,
};

#[derive(Clone, Copy)]
pub struct Stack<'a> {
    a: &'a (dyn Render),
    b: &'a (dyn Render),
    c: &'a (dyn Render),
    d: &'a (dyn Render),
    e: &'a (dyn Render),
    f: &'a (dyn Render),
}

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
    pub fn render_stack(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        if self.a.render_named(key, context, writer)?
            || self.b.render_named(key, context, writer)?
            || self.c.render_named(key, context, writer)?
            || self.d.render_named(key, context, writer)?
            || self.e.render_named(key, context, writer)?
            || self.f.render_named(key, context, writer)?
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    pub fn render_stack_section(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Error> {
        if !self.a.render_named_section(key, context, writer)? {
            let context = context.pop();
            if !self.b.render_named_section(key, context, writer)? {
                let context = context.pop();
                if !self.c.render_named_section(key, context, writer)? {
                    let context = context.pop();
                    if !self.d.render_named_section(key, context, writer)? {
                        let context = context.pop();
                        if !self.e.render_named_section(key, context, writer)? {
                            let context = context.pop();
                            self.f.render_named_section(key, context, writer)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
