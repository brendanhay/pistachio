use std::convert::Infallible;

pub use self::context::Context;
use self::context::Nodes;
use crate::template::Template;

mod context;
mod trace;
mod value;
mod writer;

// pub mod stack;
// mod trace;
// pub(crate) mod value;

#[derive(Debug, Clone, Copy)]
pub enum Escape {
    Html,
    None,
}

// Since this is disjoint over the writers, maybe move this into the associated Writer::Error type:
//     IOWriter:     IOError | MissingVariable
//     StringWriter: Infallible | MissingVariable
#[derive(Debug)]
pub enum RenderError<W> {
    MissingVariable(usize, Box<str>),
    WriteError(W),
}

impl<W> From<W> for RenderError<W> {
    fn from(err: W) -> Self {
        RenderError::WriteError(err)
    }
}

pub trait Render<'a> {
    #[inline]
    fn size_hint(&self, _template: &Template) -> usize {
        0
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        true
    }

    #[inline]
    fn variable(&self, _escape: Escape, _context: &mut Context) -> Result<(), Infallible> {
        Ok(())
    }

    #[inline]
    fn variable_key(
        &self,
        _key: &str,
        _escape: Escape,
        _context: &mut Context,
    ) -> Result<bool, Infallible> {
        Ok(false)
    }

    #[inline]
    fn section(&self, context: &mut Context, nodes: Nodes) -> Result<(), Infallible> {
        if self.is_truthy() {
            context.push_render(&self as &dyn Render, nodes)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn section_key(
        &self,
        _key: &str,
        _context: &mut Context,
        _nodes: Nodes,
    ) -> Result<bool, Infallible> {
        Ok(false)
    }

    // #[inline]
    // fn inverted_section(&self, context: &mut Context) -> Result<(), Infallible>;
    // // {
    // // if !self.is_truthy() {
    // //     context.render()
    // // } else {
    // //     Ok(())
    // // }
    // // }

    #[inline]
    fn inverted_section_key(
        &self,
        _key: &str,
        _context: &mut Context,
        _nodes: Nodes,
    ) -> Result<bool, Infallible> {
        Ok(false)
    }
}
