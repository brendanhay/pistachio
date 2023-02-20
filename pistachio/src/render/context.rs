use std::{
    self,
    convert::Infallible,
    fmt,
    io,
};

use super::{
    trace::Stack,
    // stack::{
    //     PushStack,
    //     RenderStack,
    // },
    // writer::Writer,
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
pub struct Context<'a> {
    raise: bool,
    stack: Vec<&'a dyn Render>,
    trace: Vec<&'a str>,
    write: &'a mut dyn io::Write,
}

pub struct Nodes<'a> {
    slice: &'a [Node<'a>],
}

impl<'a> Context<'a> {
    pub fn push_render(
        &'a mut self,
        // _label: &'a str,
        frame: &'a dyn Render,
        nodes: Nodes<'_>,
    ) -> Result<(), Infallible> {
        self.stack.push(frame); // (label, frame));
        let () = self.render_nodes(nodes)?;
        let _ = self.stack.pop();
        Ok(())
    }

    pub fn write(&mut self, escape: Escape, string: &str) -> Result<(), Infallible> {
        let _ = self.write.write_all(string.as_bytes());

        Ok(())
    }

    fn render_nodes(&mut self, nodes: Nodes<'_>) -> Result<(), Infallible> {
        let mut index = 0;

        while let Some(node) = nodes.slice.get(index) {
            index += 1;

            match node.tag {
                Tag::Escaped => {
                    // let len = self.stack.len();
                    // for frame in len..0 {
                    //     let boxed = self.stack[frame].as_ref();
                    //     let _found = boxed.variable_key(node.key, Escape::Html, self)?;
                    // }

                    // if !found && self.raise {
                    //     return Ok(()); // Err(RenderError::MissingVariable(node.start, node.key.into()));
                    // }
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
