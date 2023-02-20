use std::{
    self,
    ops::Range,
};

use super::{
    stack::{
        PushStack,
        RenderStack,
    },
    writer::Writer,
    Escape,
    Render,
    RenderError,
};
use crate::template::{
    Node,
    Tag,
};

/// The current mustache context containing the execution stack
/// and the applicable sub-tree of nodes.
#[derive(Clone, Copy)]
pub struct Context<'a, S: RenderStack> {
    raise: bool,
    stack: S,
    nodes: &'a [Node<'a>],
}

impl<'a> Context<'a, ()> {
    pub fn new(raise: bool, nodes: &'a [Node<'a>]) -> Self {
        Self {
            raise,
            stack: (),
            nodes,
        }
    }
}

impl<'a, S> Context<'a, S>
where
    S: RenderStack,
{
    #[inline]
    pub fn push<X>(self, frame: &X) -> Context<'a, PushStack<S, &X>>
    where
        X: ?Sized + Render,
    {
        Context {
            raise: self.raise,
            stack: self.stack.push(frame),
            nodes: self.nodes,
        }
    }

    #[inline]
    pub fn pop(self) -> Context<'a, S::Previous> {
        Context {
            raise: self.raise,
            stack: self.stack.pop(),
            nodes: self.nodes,
        }
    }

    #[inline]
    fn children(self, range: Range<usize>) -> Self {
        Context {
            raise: self.raise,
            stack: self.stack,
            nodes: &self.nodes[range],
        }
    }

    pub fn render<W: Writer>(&self, writer: &mut W) -> Result<(), RenderError<W::Error>> {
        let mut index = 0;

        while let Some(node) = self.nodes.get(index) {
            index += 1;

            match node.tag {
                Tag::Escaped => {
                    let found = self
                        .stack
                        .render_stack_escape(node.key, Escape::Html, writer)?;

                    if !found && self.raise {
                        return Err(RenderError::MissingVariable(node.start, node.key.into()));
                    }
                },

                Tag::Unescaped => {
                    let found = self
                        .stack
                        .render_stack_escape(node.key, Escape::None, writer)?;

                    if !found && self.raise {
                        return Err(RenderError::MissingVariable(node.start, node.key.into()));
                    }
                },

                Tag::Section => {
                    let children = node.children;
                    self.stack.render_stack_section(
                        node.key,
                        self.children(index..index + children),
                        writer,
                    )?;

                    index += children;
                },

                Tag::Inverted => {
                    let children = node.children;
                    self.stack.render_stack_inverted_section(
                        node.key,
                        self.children(index..index + children),
                        writer,
                    )?;

                    index += children;
                },

                Tag::Block => {},

                Tag::Parent => {},

                Tag::Partial => {},

                Tag::Content => {
                    writer.write_escape(node.text, Escape::None)?;
                },
            }
        }

        Ok(())
    }
}
