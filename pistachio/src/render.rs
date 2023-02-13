pub use self::{
    context::Context,
    writer::{
        EscapedString,
        EscapedWriter,
    },
};
use self::{
    stack::Stack,
    writer::{
        Escape,
        Writer,
    },
};
use crate::{
    template::Template,
    vars::Vars,
};

mod context;
mod stack;
mod writer;

// Self-closing tag
pub trait Render {
    fn is_truthy(&self) -> bool {
        true
    }

    fn size_hint(&self, _template: &Template) -> usize {
        0
    }

    fn render_void<W: Writer>(&self, _escape: Escape, _writer: &mut W) -> Result<(), W::Error> {
        Ok(())
    }
}

impl Render for Vars {}

// Non self-closing tag
pub trait RenderKey {
    fn render_key<W: Writer>(
        &self,
        _key: &str,
        _escape: Escape,
        _writer: &mut W,
    ) -> Result<bool, W::Error> {
        Ok(false)
    }

    fn render_section_key<W: Writer>(
        &self,
        _name: &str,
        _context: Context,
        _writer: &mut W,
    ) -> Result<bool, W::Error> {
        Ok(false)
    }

    fn render_inverted_key<W: Writer>(
        &self,
        _name: &str,
        _context: Context,
        _writer: &mut W,
    ) -> Result<bool, W::Error> {
        Ok(false)
    }
}

impl RenderKey for Stack<&Vars> {}

impl RenderKey for Option<&Vars> {
    fn render_key<W: Writer>(
        &self,
        _key: &str,
        escape: Escape,
        writer: &mut W,
    ) -> Result<bool, W::Error> {
        Ok(false)
    }
}

impl RenderKey for Vars {
    // fn render_key<W: Writer>(
    //     &self,
    //     _key: &str,
    //     escape: Escape,
    //     writer: &mut W,
    // ) -> Result<bool, W::Error> {
    //     Ok(false)
    // }
}

// impl Data {
//     fn render_section<W: Writer>(&self, context: Context, writer: &mut W) -> Result<(), W::Error> {
//         if !self.is_truthy() {
//             return Ok(());
//         }

//         match self {
//             Data::Null => {},

//             Data::Bool(_) => {
//                 context.render(writer)?;
//             },

//             Data::Str(_) => {
//                 context.render(writer)?;
//             },

//             Data::Vec(v) => {
//                 for data in self.iter() {
//                     data.render_section(context, writer)?;
//                 }
//             },

//             Data::Map(_) => {
//                 context.push(self).render(writer)?;
//             },

//             Data::Fun(_) => {
//                 unimplemented!()
//             },
//         }

//         Ok(())
//     }
// }

impl Render for bool {
    fn is_truthy(&self) -> bool {
        *self
    }

    fn size_hint(&self, _template: &Template) -> usize {
        5
    }

    fn render_void<W: Writer>(&self, _escape: Escape, writer: &mut W) -> Result<(), W::Error> {
        writer.write_escaped(if *self { "true" } else { "false" }, Escape::None)
    }
}

impl Render for String {
    fn is_truthy(&self) -> bool {
        !self.is_empty()
    }

    fn size_hint(&self, _template: &Template) -> usize {
        self.len()
    }

    fn render_void<W: Writer>(&self, escape: Escape, writer: &mut W) -> Result<(), W::Error> {
        writer.write_escaped(self, escape)
    }
}

// impl<T: Render> Render for Vec<T> {
//     //     fn is_truthy(&self) -> bool {
//         !self.is_empty()
//     }

//     //     fn render_section<W: Writer>(&self, context: Context, writer: &mut W) -> Result<(), W::Error> {
//         for item in self.iter() {
//             item.render_section(context, writer)?;
//         }

//         Ok(())
//     }
// }

// impl<H: BuildHasher> Render for HashMap<String, Data, H> {
//     fn is_truthy(&self) -> bool {
//         !self.is_empty()
//     }

//     //     fn render_section<W: Writer>(&self, context: Context, writer: &mut W) -> Result<(), W::Error> {
//         if self.is_truthy() {
//             context.push(Data::Map(self)).render(writer)
//         } else {
//             Ok(())
//         }
//     }

//     fn render_field_escaped<W>(&self, _: u64, name: &str, writer: &mut E) -> Result<bool, E::Error>
//     where
//         W: Writer,
//     {
//         match self.get(name) {
//             Some(v) => v.render_escaped(writer).map(|_| true),
//             None => Ok(false),
//         }
//     }

//     fn render_field_unescaped<W>(
//         &self,
//         _: u64,
//         name: &str,
//         writer: &mut W,
//     ) -> Result<bool, W::Error>
//     where
//         W: Writer,
//     {
//         match self.get(name) {
//             Some(v) => v.render_unescaped(writer).map(|_| true),
//             None => Ok(false),
//         }
//     }

//     fn render_field_section<C, W>(
//         &self,
//         _: u64,
//         name: &str,
//         section: Section<C>,
//         writer: &mut W,
//     ) -> Result<bool, W::Error>
//     where
//         C: ContentSequence,
//         W: Writer,
//     {
//         match self.get(name) {
//             Some(v) => v.render_section(section, writer).map(|_| true),
//             None => Ok(false),
//         }
//     }

//     fn render_field_inverse<C, W>(
//         &self,
//         _: u64,
//         name: &str,
//         section: Section<C>,
//         writer: &mut W,
//     ) -> Result<bool, W::Error>
//     where
//         C: ContentSequence,
//         W: Writer,
//     {
//         match self.get(name) {
//             Some(v) => v.render_inverse(section, writer).map(|_| true),
//             None => Ok(false),
//         }
//     }
// }
