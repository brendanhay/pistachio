use std::fmt;

pub use self::{
    context::Context,
    stack::Stack,
    value::Source,
    writer::Writer,
};
use crate::{
    error::Error,
    template::{
        Key,
        Template,
    },
};

mod context;
mod stack;
mod value;
mod writer;

pub trait Render: fmt::Debug {
    #[inline]
    fn size_hint(&self, _template: &Template) -> usize {
        0
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        true
    }

    #[inline]
    fn render_escaped(&self, _context: Context, _writer: &mut Writer) -> Result<(), Error> {
        println!("default:render_escaped {:?}", self);

        // XXX: what about erroring by default - this way trying to use
        // something like {{ foo.bar.baz }} where baz is actually a lambda, will error.
        Ok(())
    }

    #[inline]
    fn render_unescaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        println!("default:render_unescaped {:?}", self);

        // XXX: what about erroring by default - this way trying to use
        // something like {{ foo.bar.baz }} where baz is actually a lambda, will error.
        self.render_escaped(context, writer)
    }

    #[inline]
    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        println!("default:render_section {:?}", self);

        if self.is_truthy() {
            context.render(&self, writer)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn render_inverted(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        println!("default:render_inverted {:?}", self);

        if !self.is_truthy() {
            context.render(&self, writer)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn render_field_escaped(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        println!("default:render_field_escaped {:?}", self);
        Ok(false)
    }

    #[inline]
    fn render_field_unescaped(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        println!("default:render_field_unescaped {:?}", self);
        Ok(false)
    }

    #[inline]
    fn render_field_section(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        println!("default:render_field_section {:?}", self);
        Ok(false)
    }

    #[inline]
    fn render_field_inverted(
        &self,
        key: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        println!("default:render_field_inverted {:?}", self);
        Ok(false)
    }
}
