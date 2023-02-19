use std::{
    self,
    ops::Range,
};

use super::{
    stack::{
        Frame,
        PushStack,
        RenderStack,
        Trace,
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
    pub fn push<X>(self, name: &'a str, data: &'a X) -> Context<'a, PushStack<S, Frame<'a, X>>>
    where
        X: Render,
    {
        Context {
            raise: self.raise,
            stack: self.stack.push(Frame { name, data }),
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
                        let mut trace = self.stack.trace();
                        trace.push(node.key);
                        let trace = trace.join(".").into();

                        return Err(RenderError::MissingVariable(trace));
                    }
                },

                Tag::Unescaped => {
                    let found = self
                        .stack
                        .render_stack_escape(node.key, Escape::None, writer)?;

                    if !found && self.raise {
                        let mut trace = self.stack.trace();
                        trace.push(node.key);
                        let trace = trace.join(".").into();

                        return Err(RenderError::MissingVariable(trace));
                    }
                },

                Tag::Section => {
                    let children = node.len;
                    self.stack.render_stack_section(
                        node.key,
                        self.children(index..index + children),
                        writer,
                    )?;

                    index += children;
                },

                Tag::Inverted => {
                    let children = node.len;
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
                    writer.write_escape(node.raw, Escape::None)?;
                },
            }
        }

        Ok(())
    }
}
