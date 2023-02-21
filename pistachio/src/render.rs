use std::{
    convert::Infallible,
    fmt,
    io,
    rc::Rc,
};

pub use self::{
    context::Context,
    stack::Stack,
};
use crate::template::Template;

mod context;
mod stack;
mod value;
mod writer;

// pub mod stack;
// mod trace;
// pub(crate) mod value;

pub struct Writer<'a> {
    inner: &'a mut dyn io::Write,
}

impl Writer<'_> {
    pub fn write(&mut self, escape: Escape, string: &str) -> Result<(), Infallible> {
        // let _ = self.write.write_all(string.as_bytes());

        Ok(())
    }

    pub fn write_format(
        &mut self,
        escape: Escape,
        display: impl fmt::Display,
    ) -> Result<(), Infallible> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Escape {
    Html,
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum Section {
    Positive,
    Negative,
}

pub trait Render {
    #[inline]
    fn size_hint(&self, _template: &Template) -> usize {
        0
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        true
    }

    #[inline]
    fn variable(
        &self,
        _escape: Escape,
        _context: Context,
        _writer: &mut Writer,
    ) -> Result<(), Infallible> {
        // XXX: what about erroring by default - this way trying to use
        // something like {{ foo.bar.baz }} where baz is actually a lambda, will error.
        Ok(())
    }

    /// XXX: maybe push_variable, push_section is more indicative?

    #[inline]
    fn variable_key(
        &self,
        _key: &str,
        _escape: Escape,
        _context: Context,
        _writer: &mut Writer,
    ) -> Result<bool, Infallible> {
        Ok(false)
    }

    #[inline]
    fn section_is_truthy(&self, section: Section) -> bool {
        let truthy = self.is_truthy();
        match section {
            Section::Positive => truthy,
            Section::Negative => !truthy,
        }
    }

    #[inline]
    fn section(
        &self,
        section: Section,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Infallible> {
        if self.section_is_truthy(section) {
            context.push(&self).render(writer)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn section_key(
        &self,
        _key: &str,
        _section: Section,
        _context: Context,
        _writer: &mut Writer,
    ) -> Result<bool, Infallible> {
        Ok(false)
    }
}
