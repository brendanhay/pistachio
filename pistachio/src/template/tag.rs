use std::iter;

use super::name::Name;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TagKind {
    /// `{{escaped}}`
    Escaped,

    /// `{{&unescaped}}`
    Unescaped,

    /// `{{#section}}`
    Section,

    /// `{{^section}}`
    Inverted,

    /// `{{$block}}`
    Block,

    /// `{{<parent}}`
    Parent,

    /// `{{>partial}}`
    Partial,

    /// `{{/section}}`
    Closing,

    /// UTF8 text.
    Content,
}

/// A node of the template abstract syntax tree.
/// Named as such to avoid confusion with the mustache `{{$block}}` tag.
#[derive(Debug, Clone)]
pub struct Tag<'a> {
    /// Raw text content preceeding this tag.
    pub text: &'a str,
    /// The control type of this tag.
    pub kind: TagKind,
    /// Dotted key identifiers, like `foo.bar.baz`.
    pub name: Name<'a>,
    /// Since a function could be used to capture the raw source of a section,
    /// the start and end span of the source needs to be tracked.
    pub capture: &'a str,
    /// The number of child sub-tags below this tag.
    pub children: u32,
}

impl<'a> Tag<'a> {
    fn new(text: &'a str, kind: TagKind, name: Name<'a>, children: usize) -> Self {
        Self {
            text,
            kind,
            name,
            children: children as u32,
            capture: "",
        }
    }

    pub fn content(text: &'a str) -> Self {
        Self {
            text,
            kind: TagKind::Content,
            name: Name {
                start: 0,
                keys: vec![],
            },
            capture: "",
            children: 0,
        }
    }

    pub fn escaped(text: &'a str, name: Name<'a>) -> Tag<'a> {
        Self::new(text, TagKind::Escaped, name, 0)
    }

    pub fn unescaped(text: &'a str, name: Name<'a>) -> Tag<'a> {
        Self::new(text, TagKind::Unescaped, name, 0)
    }

    pub fn closing(text: &'a str, name: Name<'a>) -> Tag<'a> {
        Self::new(text, TagKind::Closing, name, 0)
    }

    pub fn section(
        text: &'a str,
        name: Name<'a>,
        tags: Option<Vec<Tag<'a>>>,
        capture: &'a str,
        close: Tag<'a>,
    ) -> Vec<Tag<'a>> {
        let children = (tags.as_deref().map(|tags| tags.len()).unwrap_or(0) + 1) as u32;

        iter::once(Self {
            text,
            kind: TagKind::Section,
            name,
            capture,
            children,
        })
        .chain(tags.into_iter().flatten())
        .chain(iter::once(close))
        .collect()
    }

    pub fn inverted(
        text: &'a str,
        name: Name<'a>,
        tags: Option<Vec<Tag<'a>>>,
        close: Tag<'a>,
    ) -> Vec<Tag<'a>> {
        let children = (tags.as_deref().map(|tags| tags.len()).unwrap_or(0) + 1) as u32;

        iter::once(Self {
            text,
            kind: TagKind::Inverted,
            name,
            capture: "",
            children,
        })
        .chain(tags.into_iter().flatten())
        .chain(iter::once(close))
        .collect()
    }

    // pub fn partial(text: &'a str, name: Name<'a>) -> Tag<'a> {
    //     Self::new(text, TagKind:b:Partial, name, 0)
    // }

    // pub fn parent(
    //     text: &'a str,
    //     name: Name<'a>,
    //     tags: Option<Vec<Tag<'a>>>,
    //     close: Tag<'a>,
    // ) -> Vec<Tag<'a>> {
    //     let children = (tags.as_deref().map(|tags| tags.len()).unwrap_or(0) + 1) as u32;
    //     let tags = tags
    //         .into_iter()
    //         .flatten()
    //         .chain(iter::once(close))
    //         .filter_map(|mut child| match child.kind {
    //             TagKind::Block | TagKind::Closing => {
    //                 child.text = "";
    //                 Some(child)
    //             },
    //             _ => None,
    //         });

    //     iter::once(Self {
    //         text,
    //         kind: TagKind::Parent,
    //         name,
    //         capture: "",
    //         children,
    //     })
    //     .chain(tags)
    //     .collect()
    // }

    pub fn block(
        text: &'a str,
        name: Name<'a>,
        tags: Option<Vec<Tag<'a>>>,
        close: Tag<'a>,
    ) -> Vec<Tag<'a>> {
        let tags = tags.unwrap_or_else(|| Vec::new());

        iter::once(Self::new(text, TagKind::Block, name, tags.len() + 1))
            .chain(tags)
            .chain(iter::once(close))
            .collect()
    }

    // pub fn dynamic_partial(text: &'a str, name: Name<'a>) -> Vec<Node<'a>> {
    //     name.explode(text, Tag::Section, Tag::Partial, None, None)
    // }

    // pub fn dynamic_parent(
    //     text: &'a str,
    //     name: Name<'a>,
    //     child: Option<Vec<Node<'a>>>,
    //     close: Node<'a>,
    // ) -> Vec<Node<'a>> {
    //     let child = child.unwrap_or_else(|| Vec::new());
    //     let mut tags = Vec::with_capacity(child.len() + 2);
    //     tags.push(Node::content(text));
    //     tags.extend(child);
    //     tags.push(close);

    //     name.explode(text, Tag::Section, Tag::Parent, Some(tags), None)
    // }

    // pub fn span(&self) -> (usize, usize) {
    //     // let start = (self.data >> 32) as usize;
    //     (self.start, self.start + self.key.len())
    // }

    #[inline]
    pub fn children(&self) -> u32 {
        self.children
    }

    // fn pack(start: usize, children: usize) -> u64 {
    //     // The span is potentially truncated since it's only used for
    //     // error messages and this lets us avoid storing 2 u64 on x64.
    //     let hi = start as u64;

    //     // Potentially truncate the number of children to u32 since
    //     // we'll be doing (usize - u32) arthimetic with it.
    //     let lo = children as u32;

    //     hi << 32 | (lo as u64)
    // }
}
