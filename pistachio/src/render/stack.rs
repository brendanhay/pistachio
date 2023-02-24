use std::fmt;

use super::{
    Context,
    Writer,
};
use crate::{
    error::Error,
    render::Render,
    template::Name,
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
    pub(crate) fn new() -> Self {
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

    // #[inline]
    // pub fn pop(self) -> Self {
    //     Self {
    //         a: self.b,
    //         b: &(),
    //         // c: self.d,
    //         // d: self.e,
    //         // e: self.f,
    //         // f: &(),
    //     }
    // }

    #[inline]
    pub fn peek(&self) -> &dyn Render {
        self.a
    }

    // pub fn resolve_name(self, name: &Name) -> Option<Self> {
    //     let name = name.keys;
    //     if name.is_empty() {
    //         Some(self)
    //     // } else if name[0] == "." {
    //     //     Some(context)
    //     } else {
    //         let head = name[0];

    //         if let Some(stack) = self.a.push_key(head, self) {
    //             stack.push_name(&name[1..])
    //         } else if let Some(stack) = self.b.push_key(head, self) {
    //             stack.push_name(&name[1..])
    //         } else {
    //             None
    //         }
    //     }
    // }

    // // fn push_name(self, name: &[&str]) -> Option<Self> {
    // //     if name.is_empty() {
    // //         Some(self)
    // //     } else {
    // //         self.peek().push_key(name[0], self)?.push_name(&name[1..])
    // //     }
    // // }

    #[inline]
    pub fn render_named_escaped(
        &self,
        name: &Name,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        if name.is_dot() {
            return self.peek().render_escaped(context, writer).map(|_| true);
        }

        let name = &name.keys;
        if name.is_empty() {
            return Ok(false);
        }

        println!("stack:render_named_escaped {:?}", name);

        Ok(self.a.render_named_escaped(name, context, writer)?
            || self.b.render_named_escaped(name, context, writer)?
            || self.c.render_named_escaped(name, context, writer)?
            || self.d.render_named_escaped(name, context, writer)?
            || self.e.render_named_escaped(name, context, writer)?
            || self.f.render_named_escaped(name, context, writer)?)
    }

    #[inline]
    pub fn render_named_unescaped(
        &self,
        name: &Name,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        if name.is_dot() {
            return self.peek().render_unescaped(context, writer).map(|_| true);
        }

        let name = &name.keys;
        if name.is_empty() {
            return Ok(false);
        }

        Ok(self.a.render_named_unescaped(name, context, writer)?
            || self.b.render_named_unescaped(name, context, writer)?
            || self.c.render_named_unescaped(name, context, writer)?
            || self.d.render_named_unescaped(name, context, writer)?
            || self.e.render_named_unescaped(name, context, writer)?
            || self.f.render_named_unescaped(name, context, writer)?)
    }

    #[inline]
    pub fn render_named_section(
        &self,
        name: &Name,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        if name.is_dot() {
            return self.peek().render_section(context, writer).map(|_| true);
        }

        let name = &name.keys;
        if name.is_empty() {
            return Ok(false);
        }

        Ok(self.a.render_named_section(name, context, writer)?
            || self.b.render_named_section(name, context, writer)?
            || self.c.render_named_section(name, context, writer)?
            || self.d.render_named_section(name, context, writer)?
            || self.e.render_named_section(name, context, writer)?
            || self.f.render_named_section(name, context, writer)?)
    }

    #[inline]
    pub fn render_named_inverted(
        &self,
        name: &Name,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        if name.is_dot() {
            return self.peek().render_inverted(context, writer).map(|_| true);
        }

        let name = &name.keys;
        if name.is_empty() {
            return Ok(false);
        }

        if !self.a.render_named_section(name, context, writer)?
            && !self.b.render_named_section(name, context, writer)?
            && !self.c.render_named_section(name, context, writer)?
            && !self.d.render_named_section(name, context, writer)?
        {
            // None of the above succeeded.
            ().render_inverted(context, writer).map(|_| true)
        } else {
            // One of the predicates succeeded.
            Ok(true)
        }
    }

    // #[inline]
    // pub fn render_field_section(
    //     &self,
    //     name: Name<'a>,
    //     context: Context,
    //     writer: &mut Writer,
    // ) -> Result<(), Error> {
    //     if name.is_dot() {
    //         self.a.render_section(context, writer)?;
    //     } else {
    //         if !self.a.render_field_section(name, context, writer)? {
    //             let context = context.pop();
    //             if !self.b.render_field_section(name, context, writer)? {
    //                 let context = context.pop();
    //                 if !self.c.render_field_section(name, context, writer)? {
    //                     let context = context.pop();
    //                     if !self.d.render_field_section(name, context, writer)? {
    //                         let context = context.pop();
    //                         if !self.e.render_field_section(name, context, writer)? {
    //                             let context = context.pop();
    //                             self.f.render_field_section(name, context, writer)?;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     Ok(())
    // }

    // #[inline]
    // pub fn render_field_inverted(
    //     &self,
    //     key: &str,
    //     context: Context,
    //     writer: &mut Writer,
    // ) -> Result<(), Error> {
    //     if key == Key::DOT {
    //         self.a.render_inverted(context, writer)?;
    //     } else {
    //         if !self.a.render_field_inverted(key, context, writer)?
    //             && !self.b.render_field_inverted(key, context, writer)?
    //             && !self.c.render_field_inverted(key, context, writer)?
    //             && !self.d.render_field_inverted(key, context, writer)?
    //             && !self.e.render_field_inverted(key, context, writer)?
    //             && !self.f.render_field_inverted(key, context, writer)?
    //         {
    //             context.render_to_writer(writer)?;
    //         }
    //     }

    //     Ok(())
    // }
}
