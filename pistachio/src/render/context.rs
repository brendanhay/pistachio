use std::ops::Range;

use super::{
    Stack,
    Var,
    WriteEscaped,
    Writer,
};
use crate::{
    parser::Spanned,
    template::{
        Node,
        Tag,
        Template,
    },
    Error,
};

/// The mustache context containing the execution stack and current sub-tree of nodes.
#[derive(Clone, Copy)]
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

    pub fn fork(self, template: &'a Template) -> Self {
        Self {
            nodes: template.nodes(),
            ..self
        }
    }

    pub fn slice(self, range: Range<usize>) -> Self {
        Self {
            nodes: &self.nodes[range],
            ..self
        }
    }

    pub fn push(self, frame: Var<'a>) -> Self {
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
            // Only UTF-8 is parsed + written.
            String::from_utf8_unchecked(buffer)
        })
    }

    pub fn render_to_writer(self, writer: &mut Writer) -> Result<(), Error> {
        let mut index = 0;

        while let Some(node) = self.nodes.get(index) {
            index += 1;

            writer.write_unescaped(node.text)?;

            match node.tag {
                Tag::Escaped => {
                    match self.stack.resolve(&node.name) {
                        None if self.raise => {
                            return Err(Error::MissingVariable(
                                node.name.span(),
                                node.name.to_string(),
                            ))
                        },
                        None => {},
                        Some(var) => var.render_escaped(self, writer)?,
                        // Variable::Null => {},
                        // Variable::Bool(b) => b.render_escaped(self, writer)?,
                        // Variable::Number(n) => n.render_escaped(self, writer)?,
                        // Variable::String(s) => s.render_escaped(self, writer)?,
                        // Variable::Nullary(f) => f().render_escaped(self, writer)?,

                        // _ => {
                        //     bug!("render_escaped: unexpected value {:?}", var);
                        // },
                    }
                },

                Tag::Unescaped => {
                    match self.stack.resolve(&node.name) {
                        None if self.raise => {
                            return Err(Error::MissingVariable(
                                node.name.span(),
                                node.name.to_string(),
                            ))
                        },
                        None => {},
                        Some(var) => var.render_unescaped(self, writer)?,
                        // Variable::Null => {},
                        // Variable::Bool(b) => b.render_unescaped(self, writer)?,
                        // Variable::Number(n) => n.render_unescaped(self, writer)?,
                        // Variable::String(s) => s.render_unescaped(self, writer)?,
                        // Variable::Nullary(f) => f().render_unescaped(self, writer)?,

                        // _ => {
                        //     bug!("render_unescaped: unexpected value {:?}", var);
                        // },
                    }
                },

                Tag::Section => {
                    let children = node.children() as usize;

                    match self.stack.resolve(&node.name) {
                        None => {},
                        Some(var) if !var.is_truthy() => {},
                        Some(var) => {
                            let slice = self.slice(index..index + children);
                            var.render_section(slice, writer)?;

                            // match var {
                            //     Variable::Null => {},
                            //     Variable::Bool(_) => slice.render_to_writer(writer)?,
                            //     Variable::Number(_) => slice.push(var).render_to_writer(writer)?,
                            //     Variable::String(_) => slice.push(var).render_to_writer(writer)?,
                            //     Variable::Map(_) => slice.push(var).render_to_writer(writer)?,
                            //     Variable::Vec(v) => {
                            //         for item in v.iter() {
                            //             slice.push(item).render_to_writer(writer)?;
                            //         }
                            //     },
                            //     Variable::Unary(f) => {
                            //         let args = slice.render(var.size_hint())?;
                            //         f(args).render_escaped(self, writer)?;
                            //     },

                            //     _ => {
                            //         bug!("render_unescaped: unexpected value {:?}", var);
                            //     },
                            // }
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
