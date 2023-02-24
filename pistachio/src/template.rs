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

    // pub(crate) fn include_partial(&self, text: &'a str) -> Vec<Node<'a>> {
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

/// XXX: Tag -> Control, Node -> Tag/Block?

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

/// A node of the template abstract syntax tree.
/// Named as such to avoid confusion with the mustache `{{$block}}` tag.
#[derive(Debug, Clone)]
pub struct Node<'a> {
    /// Raw text content preceeding this tag.
    pub text: &'a str,
    /// The control type of this tag.
    pub tag: Tag,
    /// Dotted key identifiers, like `foo.bar.baz`.
    pub name: Name<'a>,
    /// The number of child sub-nodes below to this node.
    children: u32,
}

impl<'a> Node<'a> {
    fn new(text: &'a str, tag: Tag, name: Name<'a>, children: usize) -> Self {
        Self {
            text,
            tag,
            name,
            children: children as u32,
        }
    }

    pub fn content(text: &'a str) -> Self {
        Self {
            text,
            tag: Tag::Content,
            name: Name {
                start: 0,
                keys: vec![],
            },
            children: 0,
        }
    }

    pub fn escaped(text: &'a str, name: Name<'a>) -> Node<'a> {
        Self::new(text, Tag::Escaped, name, 0)
    }

    pub fn unescaped(text: &'a str, name: Name<'a>) -> Node<'a> {
        Self::new(text, Tag::Unescaped, name, 0)
    }

    pub fn closing(text: &'a str, name: Name<'a>) -> Self {
        Self::new(text, Tag::Closing, name, 0)
    }

    pub fn section(
        text: &'a str,
        name: Name<'a>,
        nodes: Option<Vec<Node<'a>>>,
        close: Node<'a>,
    ) -> Vec<Node<'a>> {
        let children = (nodes.as_deref().map(|nodes| nodes.len()).unwrap_or(0) + 1) as u32;

        iter::once(Self {
            text,
            tag: Tag::Section,
            name,
            children,
        })
        .chain(nodes.into_iter().flatten())
        .chain(iter::once(close))
        .collect()
    }

    // pub fn inverted(
    //     text: &'a str,
    //     name: Name<'a>,
    //     nodes: Option<Vec<Node<'a>>>,
    //     close: Node<'a>,
    // ) -> Vec<Node<'a>> {
    //     name.explode(text, Tag::Inverted, Tag::Inverted, nodes, Some(close))
    // }

    // pub fn block(
    //     text: &'a str,
    //     name: Name<'a>,
    //     nodes: Option<Vec<Node<'a>>>,
    //     close: Node<'a>,
    // ) -> Vec<Node<'a>> {
    //     let nodes = nodes.unwrap_or_else(|| Vec::new());

    //     iter::once(Node::new(text, Tag::Block, key, nodes.len() + 1))
    //         .chain(nodes)
    //         .chain(iter::once(close))
    //         .collect()
    // }

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
    //     let mut nodes = Vec::with_capacity(child.len() + 2);
    //     nodes.push(Node::content(text));
    //     nodes.extend(child);
    //     nodes.push(close);

    //     name.explode(text, Tag::Section, Tag::Parent, Some(nodes), None)
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

/// A non-empty list of dotted keys such as `foo.bar.baz`.
#[derive(Debug, Clone)]
pub struct Name<'a> {
    pub start: usize,
    pub keys: Vec<&'a str>,
}

impl PartialEq<Name<'_>> for Name<'_> {
    fn eq(&self, other: &Name<'_>) -> bool {
        self.keys == other.keys
    }
}

// This is used when displaying errors to the user.
impl fmt::Display for Name<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.keys.is_empty() {
            return write!(f, "Name()");
        }

        write!(f, "Name({}", self.keys[0])?;

        for key in &self.keys[1..] {
            write!(f, ".{}", key)?;
        }

        write!(f, ")")?;

        Ok(())
    }
}

impl Spanned for Name<'_> {
    fn span(&self) -> (usize, usize) {
        let dot = self.keys.len() - 1;
        let end = self.keys.iter().map(|s| s.len()).sum::<usize>() + dot;

        (self.start, end)
    }
}

impl Name<'_> {
    #[inline]
    pub fn is_dot(&self) -> bool {
        self.keys.len() == 1 && self.keys[0] == "."
    }
}

//     pub fn new(head: Key<'a>, tail: Vec<Key<'a>>) -> Self {
//         Self { head, tail }
//     }

//     fn explode(
//         self,
//         text: &'a str,
//         tag: Tag,
//         nodes: Option<Vec<Node<'a>>>,
//         close: Option<Node<'a>>,
//     ) -> Vec<Node<'a>> {
//         // The number of nested sections to insert.
//         let dots = self.tail.len();
//         // The total number of child nodes for the first `self.head` section.
//         let mut children = dots + nodes.as_ref().map(|n| n.len()).unwrap_or(0);
//         if close.is_some() {
//             children += 1;
//         }

//         // head              : children = dots + nodes.len() + close
//         //   tail1           : children = head.children - 1
//         //     tail2         : children = head.children - 2
//         //       tailN       : children = head.children - N
//         //         [..nodes] : children = unchanged
//         //         close     : children = 0

//         iter::once(self.head)
//             .chain(self.tail.into_iter())
//             .enumerate()
//             .map(|(index, key)| {
//                 let head = index == 0;
//                 let last = index == dots;
//                 let node = Node::new(
//                     if head { text } else { "" },
//                     if last { target_tag } else { parent_tag },
//                     key,
//                     children,
//                 );

//                 if children > 0 {
//                     children -= 1;
//                 }

//                 node
//             })
//             .chain(nodes.into_iter().flatten())
//             .chain(close.into_iter())
//             .collect()
//     }
// }
