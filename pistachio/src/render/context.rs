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
    raise: bool,
}

impl<'a> Context<'a> {
    pub fn new(self, raise: bool, nodes: &'a [Node<'a>]) -> Self {
        Self {
            stack: Stack::new(),
            nodes,
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
                    let found = self.stack.variable_key(node.key, self, writer)?;
                    if !found && self.raise {
                        return Ok(()); // Err(RenderError::MissingVariable(node.start, node.key.into()));
                    }
                },

                // Tag::Unescaped => {
                //     let found = self
                //         .stack
                //         .render_stack_escape(node.key, Escape::None, writer)?;

                //     if !found && self.raise {
                //         return Err(RenderError::MissingVariable(node.start, node.key.into()));
                //     }
                // },

                // Tag::Section => {
                //     let children = node.children;
                //     self.stack.render_stack_section(
                //         node.key,
                //         self.children(index..index + children),
                //         writer,
                //     )?;

                //     index += children;
                // },

                // Tag::Inverted => {
                //     let children = node.children;
                //     self.stack.render_stack_inverted_section(
                //         node.key,
                //         self.children(index..index + children),
                //         writer,
                //     )?;

                //     index += children;
                // },

                // Tag::Block => {},

                // Tag::Parent => {},

                // Tag::Partial => {},
                Tag::Content => {
                    // self.writer.write_escape(node.text, Escape::None)?;
                },

                _ => {},
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
