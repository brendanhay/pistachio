use std::{
    borrow::Cow,
    convert::Infallible,
    fmt,
    io,
    iter,
};

use crate::{
    lexer::Lexer,
    map,
    parser::Parser,
    render::{
        Context,
        Render,
    },
    Error,
    Loader,
    LoadingDisabled,
};

/// Represents a parsed and normalised template.
#[derive(Debug)]
pub struct Template<'a> {
    pub(crate) size_hint: usize,
    pub(crate) nodes: Vec<Node<'a>>,
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

    pub fn source(&self) -> &str {
        &self.source
    }

    #[inline]
    pub(crate) fn with_loader(
        source: Cow<'a, str>,
        loader: &mut dyn Loader<'a>,
    ) -> Result<Template<'a>, Error> {
        if source.is_empty() {
            return Ok(Template {
                size_hint: 0,
                nodes: Vec::new(),
                source,
                raise: true,
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
            raise: true,
        })
    }

    // pub fn render<S: Render>(&self, vars: &S) -> Result<String, RenderError<Infallible>> {
    //     // let data = encoder::to_data(data)?;
    //     let mut capacity = vars.size_hint(self);

    //     // Add 25% for escaping and various expansions.
    //     capacity += capacity / 4;

    //     let mut buffer = String::with_capacity(capacity);
    //     let _ = self.render_to_string(vars, &mut buffer)?;

    //     Ok(buffer)
    // }

    // pub fn render_to_string<S: Render>(
    //     &self,
    //     data: &S,
    //     buffer: &mut String,
    // ) -> Result<(), RenderError<Infallible>> {
    //     // Writing to a String is Infallible
    //     Context::new(self.raise, &self.nodes)
    //         .push(&data)
    //         .render(buffer)
    // }

    // pub fn render_to_writer<S, W>(
    //     &self,
    //     data: &S,
    //     writer: &mut W,
    // ) -> Result<(), RenderError<io::Error>>
    // where
    //     S: Render,
    //     W: io::Write,
    // {
    //     let mut writer = EscapedWriter::new(writer);
    //     let () = Context::new(self.raise, &self.nodes)
    //         .push(&data)
    //         .render(&mut writer)?;

    //     Ok(())
    // }

    pub(crate) fn include(&self) -> Vec<Node<'a>> {
        self.nodes.clone()
    }

    pub(crate) fn inherit(&self, nodes: Vec<Node<'a>>) -> Vec<Node<'a>> {
        let mut buffer = Vec::with_capacity(self.nodes.len());
        let mut blocks: map::Map<_, _> = nodes
            .iter()
            .enumerate()
            .filter_map(|(i, node)| match node.tag {
                Tag::Block => Some((node.key, (i, node.children))),
                _ => None,
            })
            .collect();

        for node in &self.nodes {
            match node.tag {
                // For each block in the parent replace with any matching block override
                // found in `nodes`. Any blocks that aren't overriden are preserved.
                Tag::Block => {
                    if let Some((index, next)) = blocks.remove(node.key) {
                        buffer.extend_from_slice(&nodes[index..next]);
                    } else {
                        buffer.push(node.clone());
                    }
                },

                // Any non-block tags are preserved.
                _ => buffer.push(node.clone()),
            }
        }

        buffer
    }
}

// #[test]
// fn node_data() {
//     let key = Key { start: 592, ident: "foo" };
//     let children = 3;
//     let node = Node::new(Tag::Section, key, "", children)

// }

/// A node of the template abstract syntax tree.
/// Named as such to avoid confusion with the mustache `{{$block}}` tag.
#[derive(Debug, Clone, Copy)]
pub struct Node<'a> {
    pub tag: Tag,
    pub start: usize,
    pub key: &'a str,
    pub text: &'a str,
    pub children: usize,
}

impl<'a> Node<'a> {
    fn new(tag: Tag, key: Key<'a>, text: &'a str, children: usize) -> Self {
        Self {
            tag,
            start: key.start,
            key: key.ident,
            text,
            children,
        }
    }

    pub fn content(start: usize, text: &'a str) -> Self {
        Node {
            tag: Tag::Content,
            start,
            key: "",
            text,
            children: 0,
        }
    }

    pub fn block(key: Key<'a>, nodes: Vec<Node<'a>>) -> Vec<Node<'a>> {
        iter::once(Node::new(Tag::Block, key, "", nodes.len()))
            .chain(nodes)
            .collect()
    }
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

    /// UTF8 text.
    Content,
}

/// The `Key` grammar production rule representing a single identifier with no dots.
#[derive(Debug, Clone, Copy)]
pub struct Key<'a> {
    start: usize,
    ident: &'a str,
}

impl<'a> Key<'a> {
    pub fn new(start: usize, ident: &'a str) -> Self {
        Self { start, ident }
    }
}

impl PartialEq<str> for Key<'_> {
    fn eq(&self, other: &str) -> bool {
        self.ident == other
    }
}

impl fmt::Display for Key<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}

/// The `Name` grammar production rule representing a non-empty list of dotted
/// `Key`s such as `foo.bar.baz`.
#[derive(Debug)]
pub struct Name<'a> {
    head: Key<'a>,
    tail: Vec<Key<'a>>,
}

impl PartialEq<str> for Name<'_> {
    fn eq(&self, other: &str) -> bool {
        if self.head.ident == other {
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

impl<'a> Name<'a> {
    pub fn new(head: Key<'a>, tail: Vec<Key<'a>>) -> Self {
        Self { head, tail }
    }

    pub fn section(self, nodes: Vec<Node<'a>>) -> Vec<Node<'a>> {
        self.explode(Tag::Section, Tag::Section, Some(nodes))
    }

    pub fn inverted(self, nodes: Vec<Node<'a>>) -> Vec<Node<'a>> {
        self.explode(Tag::Inverted, Tag::Inverted, Some(nodes))
    }

    pub fn parent(self, nodes: Vec<Node<'a>>) -> Vec<Node<'a>> {
        self.explode(Tag::Section, Tag::Parent, Some(nodes))
    }

    pub fn partial(self) -> Vec<Node<'a>> {
        self.explode(Tag::Section, Tag::Partial, None)
    }

    pub fn escaped(self) -> Vec<Node<'a>> {
        self.explode(Tag::Section, Tag::Escaped, None)
    }

    pub fn unescaped(self) -> Vec<Node<'a>> {
        self.explode(Tag::Section, Tag::Unescaped, None)
    }

    fn explode(
        self,
        parent_tag: Tag,
        target_tag: Tag,
        nodes: Option<Vec<Node<'a>>>,
    ) -> Vec<Node<'a>> {
        let dots = self.tail.len();
        let children = nodes.as_ref().map(|n| n.len()).unwrap_or(0);

        println!("{:?} {:?} {:?}", self, parent_tag, target_tag);

        iter::once(self.head)
            .chain(self.tail.into_iter())
            .enumerate()
            .map(|(index, key)| {
                let last = index == dots;
                let tag = if last { target_tag } else { parent_tag };
                let next = if last { children } else { dots - index };

                Node::new(tag, key, "", next)
            })
            .chain(nodes.into_iter().flatten())
            .collect()
    }
}
