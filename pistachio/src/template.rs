use std::{
    borrow::Cow,
    fmt,
    io,
    iter,
};

use crate::{
    lexer::Lexer,
    map,
    parser::{
        Parser,
        Spanned,
    },
    render::{
        Context,
        Render,
        Writer,
    },
    Error,
    Loader,
    LoadingDisabled,
};

/// Represents a parsed and normalised template.
#[derive(Debug)]
pub struct Template<'a> {
    size_hint: usize, // XXX: these are exposed to the grammar
    nodes: Vec<Node<'a>>,
    source: Cow<'a, str>,
    raise: bool,
}

impl<'a> Template<'a> {
    pub fn new<S>(source: S) -> Result<Template<'a>, Error>
    where
        S: Into<Cow<'a, str>>,
    {
        Template::with_loader(source.into(), &mut LoadingDisabled)
    }

    pub fn size_hint(&self) -> usize {
        self.size_hint
    }

    pub(crate) fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    #[inline]
    pub fn with_loader(
        source: Cow<'a, str>,
        loader: &mut dyn Loader<'a>,
    ) -> Result<Template<'a>, Error> {
        let raise = loader.raise_if_missing();
        if source.is_empty() {
            return Ok(Template {
                size_hint: 0,
                nodes: Vec::new(),
                source,
                raise,
            });
        }

        let unsafe_source: &'a str = unsafe { &*(&*source as *const str) };

        let mut size_hint = 0;
        let lexer = Lexer::new(&unsafe_source);
        let nodes = Parser::new().parse(&mut size_hint, loader, &unsafe_source, lexer)?;

        Ok(Template {
            size_hint,
            nodes,
            source,
            raise,
        })
    }

    pub fn render<T>(&self, vars: &T) -> Result<String, Error>
    where
        T: Render,
    {
        let mut capacity = vars.size_hint(self);

        // Add 25% for escaping and various expansions.
        capacity += capacity / 4;

        Context::new(self.raise, &self.nodes, &vars).render_to_string(capacity)
    }

    pub fn render_to_writer<T, W>(&self, vars: &T, writer: &mut W) -> Result<(), Error>
    where
        T: Render,
        W: io::Write,
    {
        let mut writer = Writer::new(writer);

        Context::new(self.raise, &self.nodes, &vars).render_to_writer(&mut writer)?;

        Ok(())
    }

    pub(crate) fn include_partial(&self, text: &'a str) -> Vec<Node<'a>> {
        let mut nodes = Vec::with_capacity(self.nodes.len() + 1);
        nodes.push(Node::content(text));
        nodes.extend_from_slice(&self.nodes);
        nodes
    }

    pub(crate) fn inherit_parent(
        &self,
        text: &'a str,
        child: Option<Vec<Node<'a>>>,
        close: Node<'a>,
    ) -> Vec<Node<'a>> {
        let child = child.unwrap_or_else(|| Vec::new());
        let mut nodes = Vec::with_capacity(self.nodes.len() + 2);
        let mut blocks: map::Map<_, _> = child
            .iter()
            .chain(iter::once(&close))
            .enumerate()
            .filter_map(|(i, node)| match node.tag {
                Tag::Block => Some((node.key, (i, node.children()))),
                _ => None,
            })
            .collect();

        nodes.push(Node::content(text));

        for node in &self.nodes {
            match node.tag {
                // For each block in the parent replace with any matching block override
                // found in `nodes`. Any blocks that aren't overriden are preserved.
                Tag::Block => {
                    if let Some((index, next)) = blocks.remove(node.key) {
                        nodes.extend_from_slice(&child[index..next as usize]);
                    } else {
                        nodes.push(node.clone());
                    }
                },

                // Any non-block tags are preserved.
                _ => nodes.push(node.clone()),
            }
        }

        nodes
    }
}

/// A node of the template abstract syntax tree.
/// Named as such to avoid confusion with the mustache `{{$block}}` tag.
#[derive(Debug, Clone, Copy)]
pub struct Node<'a> {
    pub text: &'a str,
    pub tag: Tag,
    pub key: &'a str,
    start: usize,
    children: u32,
}

impl<'a> Node<'a> {
    fn new(text: &'a str, tag: Tag, key: Key<'a>, children: usize) -> Self {
        Self {
            tag,
            start: key.start,
            key: key.ident,
            text,
            children: children as u32,
        }
    }

    pub fn content(text: &'a str) -> Self {
        Self {
            text,
            tag: Tag::Content,
            key: "",
            start: 0,
            children: 0,
        }
    }

    pub fn closing(text: &'a str, name: &'a str, start: usize) -> Self {
        Self {
            text,
            tag: Tag::Closing,
            key: name,
            start,
            children: 0,
        }
    }

    pub fn escaped(text: &'a str, name: Name<'a>) -> Vec<Node<'a>> {
        name.explode(text, Tag::Section, Tag::Escaped, None, None)
    }

    pub fn unescaped(text: &'a str, name: Name<'a>) -> Vec<Node<'a>> {
        name.explode(text, Tag::Section, Tag::Unescaped, None, None)
    }

    pub fn section(
        text: &'a str,
        name: Name<'a>,
        nodes: Option<Vec<Node<'a>>>,
        close: Node<'a>,
    ) -> Vec<Node<'a>> {
        name.explode(text, Tag::Section, Tag::Section, nodes, Some(close))
    }

    pub fn inverted(
        text: &'a str,
        name: Name<'a>,
        nodes: Option<Vec<Node<'a>>>,
        close: Node<'a>,
    ) -> Vec<Node<'a>> {
        name.explode(text, Tag::Inverted, Tag::Inverted, nodes, Some(close))
    }

    pub fn block(
        text: &'a str,
        key: Key<'a>,
        nodes: Option<Vec<Node<'a>>>,
        close: Node<'a>,
    ) -> Vec<Node<'a>> {
        let nodes = nodes.unwrap_or_else(|| Vec::new());

        iter::once(Node::new(text, Tag::Block, key, nodes.len() + 1))
            .chain(nodes)
            .chain(iter::once(close))
            .collect()
    }

    pub fn dynamic_partial(text: &'a str, name: Name<'a>) -> Vec<Node<'a>> {
        name.explode(text, Tag::Section, Tag::Partial, None, None)
    }

    pub fn dynamic_parent(
        text: &'a str,
        name: Name<'a>,
        child: Option<Vec<Node<'a>>>,
        close: Node<'a>,
    ) -> Vec<Node<'a>> {
        let child = child.unwrap_or_else(|| Vec::new());
        let mut nodes = Vec::with_capacity(child.len() + 2);
        nodes.push(Node::content(text));
        nodes.extend(child);
        nodes.push(close);

        name.explode(text, Tag::Section, Tag::Parent, Some(nodes), None)
    }

    pub fn span(&self) -> (usize, usize) {
        // let start = (self.data >> 32) as usize;
        (self.start, self.start + self.key.len())
    }

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tag {
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

/// The `Key` grammar production rule representing a single identifier with no dots.
#[derive(Debug, Clone, Copy)]
pub struct Key<'a> {
    pub start: usize,
    pub ident: &'a str,
}

impl<'a> Key<'a> {
    pub const DOT: &'static str = ".";

    pub fn new(start: usize, ident: &'a str) -> Self {
        Self { start, ident }
    }

    pub fn dot(start: usize) -> Self {
        Self {
            start,
            ident: Self::DOT,
        }
    }
}

impl PartialEq<&str> for Key<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.ident == *other
    }
}

impl fmt::Display for Key<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}

impl Spanned for Key<'_> {
    fn span(&self) -> (usize, usize) {
        (self.start, self.ident).span()
    }
}

/// The `Name` grammar production rule representing a non-empty list of dotted
/// `Key`s such as `foo.bar.baz`.
#[derive(Debug)]
pub struct Name<'a> {
    head: Key<'a>,
    tail: Vec<Key<'a>>,
}

impl PartialEq<&str> for Name<'_> {
    fn eq(&self, other: &&str) -> bool {
        if self.head.ident == *other {
            true
        } else {
            iter::once(&self.head)
                .chain(self.tail.iter())
                .map(|key| (*key).ident)
                .eq(other.split('.'))
        }
    }
}

// Since `Name` is crate internal, this is only used when displaying errors.
impl fmt::Display for Name<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.head)?;

        for key in &self.tail {
            write!(f, ".{}", key)?;
        }

        Ok(())
    }
}

impl Spanned for Name<'_> {
    fn span(&self) -> (usize, usize) {
        let (start, mut end) = self.head.span();

        for key in &self.tail {
            end += 1; // '.'
            end += key.ident.len();
        }

        (start, end)
    }
}

impl<'a> Name<'a> {
    pub fn new(head: Key<'a>, tail: Vec<Key<'a>>) -> Self {
        Self { head, tail }
    }

    fn explode(
        self,
        text: &'a str,
        parent_tag: Tag,
        target_tag: Tag,
        nodes: Option<Vec<Node<'a>>>,
        close: Option<Node<'a>>,
    ) -> Vec<Node<'a>> {
        // The number of nested sections to insert.
        let dots = self.tail.len();
        // The total number of child nodes for the first `self.head` section.
        let mut children = dots + nodes.as_ref().map(|n| n.len()).unwrap_or(0);
        if close.is_some() {
            children += 1;
        }

        // head              : children = dots + nodes.len() + close
        //   tail1           : children = head.children - 1
        //     tail2         : children = head.children - 2
        //       tailN       : children = head.children - N
        //         [..nodes] : children = unchanged
        //         close     : children = 0

        iter::once(self.head)
            .chain(self.tail.into_iter())
            .enumerate()
            .map(|(index, key)| {
                let head = index == 0;
                let last = index == dots;
                let node = Node::new(
                    if head { text } else { "" },
                    if last { target_tag } else { parent_tag },
                    key,
                    children,
                );

                if children > 0 {
                    children -= 1;
                }

                node
            })
            .chain(nodes.into_iter().flatten())
            .chain(close.into_iter())
            .collect()
    }
}
