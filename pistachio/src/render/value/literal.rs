use std::convert::Infallible;

use crate::{
    render::{
        Context,
        Escape,
        Render,
    },
    Template,
};

impl Render for () {
    #[inline]
    fn is_truthy(&self) -> bool {
        false
    }
}

impl Render for String {
    #[inline]
    fn size_hint(&self, _template: &Template) -> usize {
        self.len()
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn variable(&self, escape: Escape, context: &mut Context) -> Result<(), Infallible> {
        context.write(escape, self)
    }
}

impl Render for str {
    #[inline]
    fn size_hint(&self, _template: &Template) -> usize {
        self.len()
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn variable(&self, escape: Escape, context: &mut Context) -> Result<(), Infallible> {
        context.write(escape, self)
    }
}

// impl Render for bool {
//     #[inline]
//     fn is_truthy(&self) -> bool {
//         *self
//     }

//     #[inline]
//     fn size_hint(&self, _template: &Template) -> usize {
//         5
//     }

//     #[inline]
//     fn variable<W: Writer>(
//         &self,
//         escape: Escape,
//         writer: &mut W,
//     ) -> Result<(), RenderError<W::Error>> {
//         writer
//             .write_escape(if *self { "true" } else { "false" }, escape)
//             .map_err(Into::into)
//     }
// }
