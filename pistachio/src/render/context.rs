use std::{
    self,
    ops::Range,
};

use super::{
    writer::{
        Escape,
        Writer,
    },
    Stack,
};
use crate::{
    render::RenderKey,
    template::{
        Node,
        Tag,
    },
    vars::Vars,
};

#[derive(Clone, Copy)]
pub struct Context<'a> {
    stack: Stack<&'a Vars>,
    nodes: &'a [Node<'a>],
}

impl<'a> Context<'a> {
    pub fn new(nodes: &'a [Node<'a>]) -> Self {
        Self {
            stack: Stack::new(),
            nodes,
        }
    }

    pub fn push(self, vars: &'a Vars) -> Self {
        Self {
            stack: self.stack.push(vars),
            nodes: self.nodes,
        }
    }

    pub fn pop(self) -> Self {
        Self {
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
                    self.stack.render_key(node.key, Escape::Html, writer)?;
                },

                Tag::Unescaped => {
                    self.stack.render_key(node.key, Escape::None, writer)?;
                },

                Tag::Section => {
                    let children = node.len;
                    self.stack.render_section_key(
                        node.key,
                        self.children(index..index + children),
                        writer,
                    )?;

                    index += children;
                },

                Tag::Inverted => {
                    let children = node.len;
                    self.stack.render_inverted_key(
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
                    writer.write_escaped(node.raw, Escape::None)?;
                },
            }
        }

        Ok(())
    }
}
