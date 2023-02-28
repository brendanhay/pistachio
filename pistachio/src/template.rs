use std::{
    borrow::Cow,
    io,
    iter,
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
    map::Map,
    parser::Parser,
    render::{
        Context,
        Render,
        Writer,
    },
    Error,
    Loader,
    NoLoading,
    Templates,
};

mod name;
mod tag;

// XXX: Something to record if the template has no non-content nodes,
// ie. it doesn't need to be rendered - we just use the source.

/// A self-contained mustache template guaranteed to not reference
/// external parents or partials.
#[derive(Debug, Clone)]
pub struct Template<'a> {
    size_hint: usize,
    tags: Vec<Tag<'a>>,
    source: Cow<'a, str>,
    raise: bool,
}

impl<'a> Template<'a> {
    pub fn new<S>(source: S) -> Result<Template<'a>, Error>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::with_loader(source.into(), &mut NoLoading)
    }

    pub fn size_hint(&self) -> usize {
        self.size_hint
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn render<T>(&self, value: T) -> Result<String, Error>
    where
        T: Render,
    {
        let partials = Templates::default();
        let mut capacity = self.size_hint + value.size_hint();

        // Add 25% for escaping and various expansions.
        capacity += capacity / 4;

        Context::new(self.raise, &partials, &self.tags)
            .push(&value)
            .render(capacity)
    }

    pub fn render_to_writer<T, W>(&self, value: T, writer: &mut W) -> Result<(), Error>
    where
        T: Render,
        W: io::Write,
    {
        let partials = Templates::default();
        let mut writer = Writer::new(writer);

        Context::new(self.raise, &partials, &self.tags)
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
            raise: false,
        }
    }

    #[inline]
    pub(crate) fn with_loader(
        source: Cow<'a, str>,
        loader: &mut impl Loader<'a>,
    ) -> Result<Template<'a>, Error> {
        if source.is_empty() {
            return Ok(Self::empty());
        }

        let unsafe_source: &'a str = unsafe { &*(&*source as *const str) };

        let mut size_hint = 0;
        let lexer = Lexer::new(&unsafe_source);
        let tags = Parser::new().parse(&mut size_hint, loader, &unsafe_source, lexer)?;

        Ok(Template {
            size_hint,
            tags,
            source,
            raise: loader.raise(),
        })
    }

    pub(crate) fn to_partial(&self, text: &'a str) -> Vec<Tag<'a>> {
        let mut tags = Vec::with_capacity(self.tags.len() + 1);
        tags.push(Tag::content(text));
        tags.extend_from_slice(&self.tags);
        tags
    }

    pub(crate) fn inherit_parent(
        &self,
        text: &'a str,
        child: Option<Vec<Tag<'a>>>,
        close: Tag<'a>,
    ) -> Vec<Tag<'a>> {
        let child = child.unwrap_or_else(|| Vec::new());
        let mut buffer = Vec::with_capacity(self.tags.len() + 2);
        let mut blocks: Map<_, _> = child
            .iter()
            .chain(iter::once(&close))
            .enumerate()
            .filter_map(|(i, tag)| match tag.kind {
                TagKind::Block => Some((tag.name.clone(), (i, tag))),
                _ => None,
            })
            .collect();

        buffer.push(Tag::content(text));

        let mut skip = 0;

        for (index, tag) in self.tags.iter().enumerate() {
            match tag.kind {
                // For each block in the parent replace with any matching block override
                // found in `tags`. Any blocks that aren't overriden are preserved.
                TagKind::Block => {
                    if let Some((index, block)) = blocks.remove(&tag.name) {
                        skip = index + tag.children as usize;

                        let mut tag = tag.clone();
                        tag.children = block.children;

                        // Preserve the included parent block tag's leading text.
                        let slice = &child[(index + 1)..(block.children as usize) + 1];

                        buffer.push(tag);
                        buffer.extend_from_slice(slice);
                    } else {
                        buffer.push(tag.clone());
                    }
                },

                // Any non-block tags are preserved.
                _ => {
                    if index != skip {
                        buffer.push(tag.clone());
                    }
                },
            }
        }

        buffer
    }
}
