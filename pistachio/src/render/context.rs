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
};
use crate::template::{
    Node,
    Tag,
};

/// The current mustache context containing the execution stack
/// and the applicable sub-tree of nodes.
#[derive(Clone, Copy)]
pub struct Context<'a, S: RenderStack> {
    stack: S,
    nodes: &'a [Node<'a>],
}

impl<'a> Context<'a, ()> {
    pub fn new(nodes: &'a [Node<'a>]) -> Self {
        Self { stack: (), nodes }
    }
}

impl<'a, S> Context<'a, S>
where
    S: RenderStack,
{
    pub fn push<X>(self, frame: &X) -> Context<'a, PushStack<S, &X>>
    where
        X: Render + ?Sized,
    {
        Context {
            stack: self.stack.push(frame),
            nodes: self.nodes,
        }
    }

    pub fn pop(self) -> Context<'a, S::Previous> {
        Context {
            stack: self.stack.pop(),
            nodes: self.nodes,
        }
    }

    fn children(self, range: Range<usize>) -> Self {
        Self {
            stack: self.stack,
            nodes: &self.nodes[range],
        }
    }

    pub fn render<W: Writer>(&self, writer: &mut W) -> Result<(), W::Error> {
        let mut index = 0;

        while let Some(node) = self.nodes.get(index) {
            index += 1;

            match node.tag {
                Tag::Escaped => {
                    let success = self
                        .stack
                        .render_field_escape(node.key, Escape::Html, writer)?;
                },

                Tag::Unescaped => {
                    let success = self
                        .stack
                        .render_field_escape(node.key, Escape::None, writer)?;
                },

                Tag::Section => {
                    let children = node.len;
                    self.stack.render_field_section(
                        node.key,
                        self.children(index..index + children),
                        writer,
                    )?;

                    index += children;
                },

                Tag::Inverted => {
                    let children = node.len;
                    self.stack.render_field_inverted_section(
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
