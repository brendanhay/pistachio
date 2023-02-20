use std::convert::Infallible;

use super::{
    Context,
    Escape,
};
use crate::Render;

#[derive(Clone, Copy)]
pub struct Stack<'a> {
    a: &'a dyn Render<'a>,
    b: &'a dyn Render<'a>,
    c: &'a dyn Render<'a>,
    d: &'a dyn Render<'a>,
    e: &'a dyn Render<'a>,
    f: &'a dyn Render<'a>,
    // g: &'a dyn Render<'a>,
    // h: &'a dyn Render<'a>,
    // i: &'a dyn Render<'a>,
    // j: &'a dyn Render<'a>,
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
    pub fn push(&mut self, frame: &'a dyn Render<'a>) {
        self.a = frame;
        self.b = self.a;
        self.c = self.b;
        self.d = self.c;
        self.e = self.d;
        self.f = self.e;
        // self.g = self.f;
        // self.h = self.g;
        // self.i = self.h;
        // self.j = self.i;
    }

    #[inline]
    pub fn pop(&mut self) {
        self.a = self.b;
        self.b = self.c;
        self.c = self.d;
        self.d = self.e;
        self.e = self.f;
        self.f = &();
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
        escape: Escape,
        context: &mut Context,
    ) -> Result<bool, Infallible> {
        if self.a.variable_key(key, escape, context)?
            || self.b.variable_key(key, escape, context)?
            || self.c.variable_key(key, escape, context)?
            || self.d.variable_key(key, escape, context)?
            || self.e.variable_key(key, escape, context)?
            || self.f.variable_key(key, escape, context)?
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // #[inline]
    // fn render_stack_section<'a, S, W>(
    //     &self,
    //     key: &str,
    //     context: Context<'a, S>,
    //     writer: &mut W,
    // ) -> Result<(), Render<'a>Error<W::Error>>
    // where
    //     S: Render<'a>Stack,
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
    // ) -> Result<(), Render<'a>Error<W::Error>>
    // where
    //     S: Render<'a>Stack,
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
