use std::{
    borrow::Cow,
    io,
};

pub use self::{
    name::Name,
    tag::{
        Tag,
        TagKind,
    },
};
use crate::{
    lexer::Lexer,
    map::Set,
    parser::Parser,
    render::{
        Context,
        Render,
        Writer,
    },
    Error,
    Templates,
};

mod name;
mod tag;

// XXX: Something to record if the template has no non-content nodes,
// ie. it doesn't need to be rendered - we just use the source.

// lambdas
// unparsed lambdas (something like Source vs String)

/// A self-contained mustache template guaranteed to not reference
/// external parents or partials.
#[derive(Debug, Clone)]
pub struct Template<'a> {
    size_hint: usize,
    tags: Vec<Tag<'a>>,
    source: Cow<'a, str>,
}

impl<'a> Template<'a> {
    pub fn new<S>(source: S) -> Result<Template<'a>, Error>
    where
        S: Into<Cow<'a, str>>,
    {
        let (template, partials) = Self::load(source.into())?;
        if partials.is_empty() {
            Ok(template)
        } else {
            Err(Error::LoadingDisabled)
        }
    }

    pub fn size_hint(&self) -> usize {
        self.size_hint
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn render<T>(&self, raise: bool, value: T) -> Result<String, Error>
    where
        T: Render,
    {
        let partials = Templates::default();
        let mut capacity = self.size_hint + value.size_hint();

        // Add 25% for escaping and various expansions.
        capacity += capacity / 4;

        Context::new(raise, &partials, &self.tags)
            .push(&value)
            .render(capacity)
    }

    pub fn render_to_writer<T, W>(&self, raise: bool, value: T, writer: &mut W) -> Result<(), Error>
    where
        T: Render,
        W: io::Write,
    {
        let partials = Templates::default();
        let mut writer = Writer::new(writer);

        Context::new(raise, &partials, &self.tags)
            .push(&value)
            .render_to_writer(&mut writer)
    }

    pub(crate) fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub(crate) fn empty() -> Self {
        Template {
            size_hint: 0,
            tags: Vec::new(),
            source: "".into(),
        }
    }

    #[inline]
    pub(crate) fn load(source: Cow<'a, str>) -> Result<(Template<'a>, Set<&'a str>), Error> {
        let mut partials = Set::default();
        if source.is_empty() {
            return Ok((Self::empty(), partials));
        }

        let unsafe_source: &'a str = unsafe { &*(&*source as *const str) };

        let mut size_hint = 0;
        let lexer = Lexer::new(&unsafe_source);
        let tags = Parser::new().parse(&mut size_hint, &mut partials, &unsafe_source, lexer)?;

        Ok((
            Template {
                size_hint,
                tags,
                source,
            },
            partials,
        ))
    }

    // pub(crate) fn to_partial(&self, text: &'a str) -> Vec<Node<'a>> {
    //     let mut nodes = Vec::with_capacity(self.nodes.len() + 1);
    //     nodes.push(Node::content(text));
    //     nodes.extend_from_slice(&self.nodes);
    //     nodes
    // }

    // pub(crate) fn inherit_parent(
    //     &self,
    //     text: &'a str,
    //     child: Option<Vec<Node<'a>>>,
    //     close: Node<'a>,
    // ) -> Vec<Node<'a>> {
    //     let child = child.unwrap_or_else(|| Vec::new());
    //     let mut nodes = Vec::with_capacity(self.nodes.len() + 2);
    //     let mut blocks: map::Map<_, _> = child
    //         .iter()
    //         .chain(iter::once(&close))
    //         .enumerate()
    //         .filter_map(|(i, node)| match node.tag {
    //             Tag::Block => Some((node.key, (i, node.children()))),
    //             _ => None,
    //         })
    //         .collect();

    //     nodes.push(Node::content(text));

    //     for node in &self.nodes {
    //         match node.tag {
    //             // For each block in the parent replace with any matching block override
    //             // found in `nodes`. Any blocks that aren't overriden are preserved.
    //             Tag::Block => {
    //                 if let Some((index, next)) = blocks.remove(node.key) {
    //                     nodes.extend_from_slice(&child[index..next as usize]);
    //                 } else {
    //                     nodes.push(node.clone());
    //                 }
    //             },

    //             // Any non-block tags are preserved.
    //             _ => nodes.push(node.clone()),
    //         }
    //     }

    //     nodes
    // }
}
