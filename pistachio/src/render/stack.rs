use std::convert::Infallible;

use super::{
    Context,
    Writer,
};
use crate::Render;

#[derive(Clone, Copy)]
pub struct Stack<'a> {
    a: &'a (dyn Render),
    b: &'a (dyn Render),
    c: &'a (dyn Render),
    d: &'a (dyn Render),
    e: &'a (dyn Render),
    f: &'a (dyn Render),
    // g: &'a (dyn Render,
    // h: &'a (dyn Render,
    // i: &'a (dyn Render,
    // j: &'a (dyn Render,
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
            // g: &(),
            // h: &(),
            // i: &(),
            // j: &(),
        }
    }

    #[inline]
    pub fn push(self, frame: &'a (dyn Render)) -> Self {
        todo!()
        // self.a = frame;
        // self.b = self.a;
        // self.c = self.b;
        // self.d = self.c;
        // self.e = self.d;
        // self.f = self.e;
        // self.g = self.f;
        // self.h = self.g;
        // self.i = self.h;
        // self.j = self.i;
    }

    #[inline]
    pub fn pop(self) -> Self {
        todo!()
        // let ok = self.a;
        // self.a = self.b;
        // self.b = self.c;
        // self.c = self.d;
        // self.d = self.e;
        // self.e = self.f;
        // self.f = &();
        // self.f = self.g;
        // self.g = self.h;
        // self.h = self.i;
        // self.i = self.j;
        // self.j = &();
    }

    #[inline]
    pub fn variable_key(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Infallible> {
        Ok(false)
        // if self.a.variable_key(key, escape, context)?
        //     || self.b.variable_key(key, escape, context)?
        //     || self.c.variable_key(key, escape, context)?
        //     || self.d.variable_key(key, escape, context)?
        //     || self.e.variable_key(key, escape, context)?
        //     || self.f.variable_key(key, escape, context)?
        // {
        //     Ok(true)
        // } else {
        //     Ok(false)
        // }
    }

    // #[inline]
    // fn render_stack_section<'a, S, W>(
    //     &self,
    //     key: &str,
    //     context: Context<'a, S>,
    //     writer: &mut W,
    // ) -> Result<(), RenderError<W::Error>>
    // where
    //     S: RenderStack,
    //     W: Writer,
    // {
    //     if !self.a.field_section(key, context, writer)? {
    //         let context = context.pop();
    //         if !self.b.field_section(key, context, writer)? {
    //             let context = context.pop();
    //             if !self.c.field_section(key, context, writer)? {
    //                 let context = context.pop();
    //                 if !self.d.field_section(key, context, writer)? {
    //                     let context = context.pop();
    //                     if !self.e.field_section(key, context, writer)? {
    //                         let context = context.pop();
    //                         self.f.field_section(key, context, writer)?;
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     Ok(())
    // }

    // #[inline]
    // fn render_stack_inverted_section<'a, S, W>(
    //     &self,
    //     key: &str,
    //     context: Context<'a, S>,
    //     writer: &mut W,
    // ) -> Result<(), RenderError<W::Error>>
    // where
    //     S: RenderStack,
    //     W: Writer,
    // {
    //     if !self.a.field_inverted_section(key, context, writer)?
    //         && !self.b.field_inverted_section(key, context, writer)?
    //         && !self.c.field_inverted_section(key, context, writer)?
    //         && !self.d.field_inverted_section(key, context, writer)?
    //         && !self.e.field_inverted_section(key, context, writer)?
    //         && !self.f.field_inverted_section(key, context, writer)?
    //     {
    //         context.render(writer)?;
    //     }

    //     Ok(())
    // }
}
