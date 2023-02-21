use std::{
    self,
    convert::Infallible,
    fmt,
    io,
    ops::Range,
    rc::Rc,
};

use super::{
    // stack::{
    //     PushStack,
    //     RenderStack,
    // },
    // writer::Writer,
    Escape,
    Render,
    Section,
    Stack,
    Writer,
};
use crate::{
    template::{
        Node,
        Tag,
    },
    Template,
};

/// The mustache context containing the execution stack and current sub-tree of nodes.
#[derive(Clone, Copy)]
pub struct Context<'a> {
    stack: Stack<'a>,
    nodes: &'a [Node<'a>],
    pub section: Section,
    pub escape: Escape,
    pub raise: bool,
}

impl<'a> Context<'a> {
    pub fn new(self, raise: bool, nodes: &'a [Node<'a>]) -> Self {
        Self {
            stack: Stack::new(),
            nodes,
            section: Section::Positive,
            escape: Escape::Html,
            raise,
        }
    }

    pub fn fork(self, nodes: &'a [Node<'a>]) -> Self {
        Self { nodes, ..self }
    }

    pub fn slice(self, range: Range<usize>) -> Self {
        Self {
            nodes: &self.nodes[range],
            ..self
        }
    }

    pub fn push(self, frame: &'a dyn Render) -> Self {
        Self {
            stack: self.stack.push(frame),
            ..self
        }
    }

    pub fn pop(self) -> Self {
        Self {
            stack: self.stack.pop(),
            ..self
        }
    }

    fn inverted(self) -> Self {
        Self {
            section: Section::Negative,
            ..self
        }
    }

    fn unescaped(self) -> Self {
        Self {
            escape: Escape::None,
            ..self
        }
    }

    pub fn capture(self) -> Result<String, Infallible> {
        let mut buffer: Vec<u8> = Vec::with_capacity(8);
        let mut writer: Writer = Writer { inner: &mut buffer };

        self.render(&mut writer)?;

        Ok(unsafe {
            // We do not emit invalid UTF-8.
            String::from_utf8_unchecked(buffer)
        })
    }

    pub fn render(self, writer: &mut Writer) -> Result<(), Infallible> {
        let mut index = 0;

        while let Some(node) = self.nodes.get(index) {
            index += 1;

            match node.tag {
                Tag::Escaped => {
                    let found = self.stack.render_stack(node.key, self, writer)?;

                    if !found && self.raise {
                        return Ok(()); // Err(RenderError::MissingVariable(node.start, node.key.into()));
                    }
                },

                Tag::Unescaped => {
                    let found = self
                        .stack
                        .render_stack(node.key, self.unescaped(), writer)?;

                    if !found && self.raise {
                        return Ok(()); // Err(RenderError::MissingVariable(node.start, node.key.into()));
                    }
                },

                Tag::Section => {
                    self.stack.render_stack_section(
                        node.key,
                        self.slice(index..index + node.children),
                        writer,
                    )?;

                    index += node.children;
                },

                Tag::Inverted => {
                    self.stack.render_stack_section(
                        node.key,
                        self.slice(index..index + node.children).inverted(),
                        writer,
                    )?;

                    index += node.children;
                },

                Tag::Block => {},

                Tag::Parent => {},

                Tag::Partial => {},

                Tag::Content => {
                    writer.write(Escape::None, node.text)?;
                },
            }
        }

        Ok(())
    }
}

// pub fn trace(&mut self, trace: &'a str) -> Result<(), Infallible> {}

// pub fn push<F>(&mut self, frame: &'a dyn Render, action: F) -> Result<(), Infallible>
// where
//     F: FnOnce(&mut Self) -> Result<(), Infallible>,
// {
//     self.stack.push(frame);
//     let result = action(self);
//     self.stack.pop();

//     result
// }

// impl<'a> Context<'a, ()> {
//     pub fn new(raise: bool, nodes: &'a [Node<'a>]) -> Self {
//         Self {
//             raise,
//             stack: (),
//             nodes,
//         }
//     }
// }

// impl<'a, S> Context<'a, S>
// where
//     S: RenderStack,
// {
//     #[inline]
//     pub fn push<X>(self, frame: &X) -> Context<'a, PushStack<S, &X>>
//     where
//         X: ?Sized + Render,
//     {
//         Context {
//             raise: self.raise,
//             stack: self.stack.push(frame),
//             nodes: self.nodes,
//         }
//     }

//     #[inline]
//     pub fn pop(self) -> Context<'a, S::Previous> {
//         Context {
//             raise: self.raise,
//             stack: self.stack.pop(),
//             nodes: self.nodes,
//         }
//     }

//     #[inline]
//     fn children(self, range: Range<usize>) -> Self {
//         Context {
//             raise: self.raise,
//             stack: self.stack,
//             nodes: &self.nodes[range],
//         }
//     }
