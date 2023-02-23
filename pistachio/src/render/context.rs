use std::ops::Range;

use super::{
    Render,
    Stack,
    Writer,
};
use crate::{
    error::Error,
    template::{
        Node,
        Tag,
    },
};

/// The mustache context containing the execution stack and current sub-tree of nodes.
#[derive(Debug, Clone, Copy)]
pub struct Context<'a> {
    stack: Stack<'a>,
    nodes: &'a [Node<'a>],
    raise: bool,
}

impl<'a> Context<'a> {
    pub fn new(raise: bool, nodes: &'a [Node<'a>], frame: &'a dyn Render) -> Self {
        Self {
            stack: Stack::new().push(frame),
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

    pub fn pop(self) -> Self {
        Self {
            stack: self.stack.pop(),
            ..self
        }
    }

    pub fn render(self, frame: &'a dyn Render, writer: &mut Writer) -> Result<(), Error> {
        println!("pushing frame {:?}", frame);

        // XXX: note to self about bugs of a christmas past; don't leak the
        // `context.push(x).render(writer)` idiom, keep it opaque since it's
        // incorrect but incredibly easy to accidentally do when implementing
        // Render for new types - hence, always use `render` instead.
        Self {
            stack: self.stack.push(frame),
            ..self
        }
        .render_to_writer(writer)
    }

    pub fn render_to_string(self, capacity: usize) -> Result<String, Error> {
        let mut buffer = Vec::with_capacity(capacity);
        let mut writer: Writer = Writer::new(&mut buffer);

        self.render_to_writer(&mut writer)?;

        Ok(unsafe {
            // We do not emit invalid UTF-8.
            String::from_utf8_unchecked(buffer)
        })
    }

    pub fn render_to_writer(self, writer: &mut Writer) -> Result<(), Error> {
        let mut index = 0;

        while let Some(node) = self.nodes.get(index) {
            index += 1;

            writer.write_unescaped(node.text)?;

            println!("render_to_writer {:?} {} {:#?}", node.tag, node.key, self);

            match node.tag {
                Tag::Escaped => {
                    let found = self.stack.render_field_escaped(node.key, self, writer)?;
                    if !found && self.raise {
                        return Err(Error::MissingVariable(node.span(), node.key.into()));
                    }
                },

                Tag::Unescaped => {
                    let found = self.stack.render_field_unescaped(node.key, self, writer)?;
                    if !found && self.raise {
                        return Err(Error::MissingVariable(node.span(), node.key.into()));
                    }
                },

                Tag::Section => {
                    let children = node.children() as usize;
                    self.stack.render_field_section(
                        node.key,
                        self.slice(index..index + children),
                        writer,
                    )?;

                    index += children;
                },

                Tag::Inverted => {
                    let children = node.children() as usize;
                    self.stack.render_field_inverted(
                        node.key,
                        self.slice(index..index + children),
                        writer,
                    )?;

                    index += children;
                },

                Tag::Block => {},

                Tag::Parent => {},

                Tag::Partial => {},

                Tag::Closing => {},

                Tag::Content => {},
            }
        }

        println!("exit render");

        Ok(())
    }
}

// pub fn trace(&mut self, trace: &'a str) -> Result<(), Error> {}

// pub fn push<F>(&mut self, frame: &'a dyn Render, action: F) -> Result<(), Error>
// where
//     F: FnOnce(&mut Self) -> Result<(), Error>,
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
