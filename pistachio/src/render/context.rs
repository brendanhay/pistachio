use std::ops::Range;

use super::{
    Render,
    Stack,
    WriteEscaped,
    Writer,
};
use crate::{
    parser::Spanned,
    template::{
        Tag,
        TagKind,
        Template,
    },
    Error,
    Templates,
};

/// The mustache context containing the execution stack and current sub-tree of tags.
#[derive(Clone, Copy)]
pub struct Context<'a> {
    stack: Stack<'a>,
    partials: &'a Templates,
    tags: &'a [Tag<'a>],
    raise: bool,
}

impl<'a> Context<'a> {
    pub(crate) fn new(raise: bool, partials: &'a Templates, tags: &'a [Tag<'a>]) -> Self {
        Self {
            stack: Stack::new(),
            partials,
            tags,
            raise,
        }
    }

    pub fn fork(self, template: &'a Template) -> Self {
        Self {
            tags: template.tags(),
            ..self
        }
    }

    pub fn slice(self, range: Range<usize>) -> Self {
        Self {
            tags: &self.tags[range],
            ..self
        }
    }

    pub fn push(self, frame: &'a (dyn Render + 'a)) -> Self {
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

        while let Some(tag) = self.tags.get(index) {
            index += 1;

            writer.write_unescaped(tag.text)?;

            match tag.kind {
                TagKind::Escaped => {
                    match self.stack.resolve(&tag.name) {
                        None if self.raise => {
                            return Err(Error::MissingVariable(
                                tag.name.span(),
                                tag.name.to_string(),
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

                TagKind::Unescaped => {
                    match self.stack.resolve(&tag.name) {
                        None if self.raise => {
                            return Err(Error::MissingVariable(
                                tag.name.span(),
                                tag.name.to_string(),
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

                TagKind::Section => {
                    let children = tag.children() as usize;

                    match self.stack.resolve(&tag.name) {
                        None => {},
                        Some(var) if !var.is_truthy() => {},
                        Some(var) => {
                            var.render_section(
                                tag.capture,
                                self.slice(index..index + children),
                                writer,
                            )?;

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

                TagKind::Inverted => {
                    let children = tag.children() as usize;

                    match self.stack.resolve(&tag.name) {
                        Some(var) if var.is_truthy() => {},
                        _ => self
                            .slice(index..index + children)
                            .render_to_writer(writer)?,
                    }

                    index += children;
                },

                TagKind::Block => {},
                // Tag::Parent => match tag.name.path() {
                //     None => unreachable!(),
                //     Some(path) => {
                //         // Shouldn't need to raise errors here since the Loader trait
                //         // ensures missing partial errors were already raised.
                //         match self.partials.get(path) {
                //             None => {},
                //             Some(parent) => {},
                //         }
                //     },
                // },

                // TagKind::Partial => match tag.name.path() {
                //     None => unreachable!(),
                //     Some(path) => {
                //         // Shouldn't need to raise errors here since the Loader trait
                //         // ensures missing partial errors were already raised.
                //         match self.partials.get(path) {
                //             None => {},
                //             Some(partial) => {
                //                 self.fork(partial).render_to_writer(writer)?;
                //             },
                //         }
                //     },
                // },
                TagKind::Closing => {},

                TagKind::Content => {},

                _ => {
                    todo!()
                },
            }
        }

        Ok(())
    }
}
