use std::ops::Range;

use super::{
    ser::Variable,
    stack::Stack,
    Render,
    WriteEscaped,
    Writer,
};
use crate::{
    error::Error,
    template::{
        Node,
        Tag,
    },
};

macro_rules! bug {
    ($msg:expr) => ({
        bug!("{}", $msg)
    });
    ($fmt:expr, $($arg:tt)+) => ({
        panic!(
            concat!("bug: ",
                    $fmt,
                    ". Please report this issue on GitHub if you find \
                    an input that triggers this case."),
            $($arg)*
        )
    });
}

/// The mustache context containing the execution stack and current sub-tree of nodes.
#[derive(Debug, Clone, Copy)]
pub struct Context<'a> {
    stack: Stack<'a>,
    nodes: &'a [Node<'a>],
    raise: bool,
}

impl<'a> Context<'a> {
    pub fn new(raise: bool, nodes: &'a [Node<'a>]) -> Self {
        Self {
            stack: Stack::new(),
            nodes,
            raise,
        }
    }

    pub fn slice(self, range: Range<usize>) -> Self {
        Self {
            nodes: &self.nodes[range],
            ..self
        }
    }

    pub fn push(self, frame: &'a Variable) -> Self {
        Self {
            stack: self.stack.push(frame),
            ..self
        }
    }

    pub fn render(&self, capacity: usize) -> Result<String, Error> {
        let mut buffer = Vec::with_capacity(capacity);
        let mut writer = Writer::new(&mut buffer);

        self.render_to_writer(&mut writer)?;

        Ok(unsafe {
            // Parser is UTF-8 only and the Writer does not emit invalid UTF-8.
            String::from_utf8_unchecked(buffer)
        })
    }

    pub fn render_to_writer<W: WriteEscaped>(self, writer: &mut W) -> Result<(), Error> {
        let mut index = 0;

        while let Some(node) = self.nodes.get(index) {
            index += 1;

            writer.write_unescaped(node.text)?;

            match node.tag {
                Tag::Escaped => {
                    match self.stack.resolve(&node.name) {
                        None => {}, // return Err(Error::MissingVariable(node.span(), node.name.into()));
                        Some(var) => match var {
                            Variable::Null => {},
                            Variable::Bool(b) => b.render_escaped(writer)?,
                            Variable::Number(n) => n.render_escaped(writer)?,
                            Variable::String(s) => s.render_escaped(writer)?,
                            _ => {
                                bug!("render_escaped: unexpected value {:?}", var);
                            },
                        },
                    }
                },

                Tag::Unescaped => {
                    match self.stack.resolve(&node.name) {
                        None => {}, // return Err(Error::MissingVariable(node.span(), node.name.into()));
                        Some(var) => match var {
                            Variable::Null => {},
                            Variable::Bool(b) => b.render_unescaped(writer)?,
                            Variable::Number(n) => n.render_unescaped(writer)?,
                            Variable::String(s) => s.render_unescaped(writer)?,
                            _ => {
                                bug!("render_unescaped: unexpected value {:?}", var);
                            },
                        },
                    }
                },

                Tag::Section => {
                    let children = node.children() as usize;

                    match self.stack.resolve(&node.name) {
                        None => {},
                        Some(var) if !var.is_truthy() => {},
                        Some(var) => {
                            let slice = self.slice(index..index + children);

                            match var {
                                Variable::Null => {},
                                Variable::Bool(_) => slice.render_to_writer(writer)?,
                                Variable::Number(_) => slice.push(var).render_to_writer(writer)?,
                                Variable::String(_) => slice.push(var).render_to_writer(writer)?,
                                Variable::Map(m) => slice.push(var).render_to_writer(writer)?,
                                Variable::Vec(v) => {
                                    for item in v.iter() {
                                        slice.push(item).render_to_writer(writer)?;
                                    }
                                },
                            }
                        },
                    }

                    index += children;
                },

                Tag::Inverted => {
                    let children = node.children() as usize;

                    match self.stack.resolve(&node.name) {
                        Some(var) if var.is_truthy() => {},
                        _ => self
                            .slice(index..index + children)
                            .render_to_writer(writer)?,
                    }

                    index += children;
                },

                // Tag::Inverted => {
                //     let children = node.children() as usize;
                //     self.stack.render_named_inverted(
                //         &node.name,
                //         Self {
                //             nodes: &self.nodes[index..index + children],
                //             ..self
                //         },
                //         writer,
                //     )?;

                //     index += children;
                // },

                // Tag::Block => {},

                // Tag::Parent => {},

                // Tag::Partial => {},
                Tag::Closing => {},

                Tag::Content => {},

                _ => {
                    todo!()
                },
            }
        }

        Ok(())
    }
}

// pub fn push(self, frame: &'a dyn Render) -> Self {
//         Self {
//             stack: self.stack.push(frame),
//             ..self
//         }
//     }

//     pub fn peek(&self) -> &dyn Render {
//         self.stack.peek()
//     }

//     pub fn render_inline(self, nodes: &'a [Node<'a>], writer: &mut Writer) -> Result<(), Error> {
//         Self { nodes, ..self }.render_to_writer(writer)
//     }

//     pub fn render_to_string(self, capacity: usize) -> Result<String, Error> {
//         let mut buffer = Vec::with_capacity(capacity);
//         let mut writer: Writer = Writer::new(&mut buffer);

//         self.render_to_writer(&mut writer)?;

//         Ok(unsafe {
//             // We do not emit invalid UTF-8.
//             String::from_utf8_unchecked(buffer)
//         })
//     }

//     pub fn render_to_writer(self, writer: &mut Writer) -> Result<(), Error> {
//         let mut index = 0;

//         while let Some(node) = self.nodes.get(index) {
//             index += 1;

//             writer.write_unescaped(node.text)?;

//             println!("{:?}", self.stack);

//             match node.tag {
//                 Tag::Escaped => {
//                     let found = self.stack.render_named_escaped(&node.name, self, writer)?;
//                     if !found {
//                         // return Err(Error::MissingVariable(node.span(), node.name.into()));
//                     }
//                 },

//                 Tag::Unescaped => {
//                     let found = self
//                         .stack
//                         .render_named_unescaped(&node.name, self, writer)?;
//                     if !found {
//                         // return Err(Error::MissingVariable(node.span(), node.name.into()));
//                     }
//                 },

//                 Tag::Section => {
//                     let children = node.children() as usize;
//                     self.stack.render_named_section(
//                         &node.name,
//                         Self {
//                             nodes: &self.nodes[index..index + children],
//                             ..self
//                         },
//                         writer,
//                     )?;

//                     index += children;
//                 },

//                 Tag::Inverted => {
//                     let children = node.children() as usize;
//                     self.stack.render_named_inverted(
//                         &node.name,
//                         Self {
//                             nodes: &self.nodes[index..index + children],
//                             ..self
//                         },
//                         writer,
//                     )?;

//                     index += children;
//                 },

//                 Tag::Block => {},

//                 Tag::Parent => {},

//                 Tag::Partial => {},

//                 Tag::Closing => {},

//                 Tag::Content => {},

//                 _ => {
//                     todo!()
//                 },
//             }
//         }

//         println!("exit render");

//         Ok(())
//     }
// }

// // pub fn trace(&mut self, trace: &'a str) -> Result<(), Error> {}

// // pub fn push<F>(&mut self, frame: &'a dyn Render, action: F) -> Result<(), Error>
// // where
// //     F: FnOnce(&mut Self) -> Result<(), Error>,
// // {
// //     self.stack.push(frame);
// //     let result = action(self);
// //     self.stack.pop();

// //     result
// // }

// // impl<'a> Context<'a, ()> {
// //     pub fn new(raise: bool, nodes: &'a [Node<'a>]) -> Self {
// //         Self {
// //             raise,
// //             stack: (),
// //             nodes,
// //         }
// //     }
// // }

// // impl<'a, S> Context<'a, S>
// // where
// //     S: RenderStack,
// // {
// //     #[inline]
// //     pub fn push<X>(self, frame: &X) -> Context<'a, PushStack<S, &X>>
// //     where
// //         X: ?Sized + Render,
// //     {
// //         Context {
// //             raise: self.raise,
// //             stack: self.stack.push(frame),
// //             nodes: self.nodes,
// //         }
// //     }

// //     #[inline]
// //     pub fn pop(self) -> Context<'a, S::Previous> {
// //         Context {
// //             raise: self.raise,
// //             stack: self.stack.pop(),
// //             nodes: self.nodes,
// //         }
// //     }

// //     #[inline]
// //     fn children(self, range: Range<usize>) -> Self {
// //         Context {
// //             raise: self.raise,
// //             stack: self.stack,
// //             nodes: &self.nodes[range],
// //         }
// //     }
