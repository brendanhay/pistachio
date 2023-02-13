//! Smart constructors to create template nodes for the various
//! LR grammar production rules.

use std::iter;

use crate::{
    map,
    template::{
        Key,
        Node,
        Tag,
    },
};

// XXX: doesn't consider custom delimiters
macro_rules! balanced {
    ($predicate:expr, $open:expr, $path:expr) => {
        if !$predicate {
            return Err(crate::parser::ParseError::User {
                error: crate::error::Error::Parser(Box::from(format!(
                    "{{{{{open}{path}}}}} is missing the corresponding {{{{/{path}}}}} end tag",
                    open = $open,
                    path = $path,
                ))),
            });
        }
    };
}

pub(crate) use balanced;

pub fn section<'a>(key: Key<'a>, nodes: Vec<Node<'a>>) -> Vec<Node<'a>> {
    explode(key, Tag::Section, Tag::Section, Some(nodes))
}

pub fn inverted<'a>(key: Key<'a>, nodes: Vec<Node<'a>>) -> Vec<Node<'a>> {
    explode(key, Tag::Inverted, Tag::Inverted, Some(nodes))
}

pub fn block<'a>(key: &'a str, nodes: Vec<Node<'a>>) -> Vec<Node<'a>> {
    iter::once(Node {
        tag: Tag::Block,
        key,
        raw: "",
        len: nodes.len(),
    })
    .chain(nodes)
    .collect()
}

pub fn parent<'a>(key: Key<'a>, nodes: Vec<Node<'a>>) -> Vec<Node<'a>> {
    explode(key, Tag::Section, Tag::Parent, Some(nodes))
}

pub fn inherit<'a>(parent: Vec<Node<'a>>, nodes: Vec<Node<'a>>) -> Vec<Node<'a>> {
    println!("inherit");

    let mut buffer = Vec::with_capacity(parent.len());
    let mut blocks: map::Map<_, _> = nodes
        .iter()
        .enumerate()
        .filter_map(|(i, node)| match node.tag {
            Tag::Block => Some((node.key, (i, node.len))),
            _ => None,
        })
        .collect();

    for node in parent {
        match node.tag {
            // For each block in the parent replace with any matching block override
            // found in `nodes`. Any blocks that aren't overriden are preserved.
            Tag::Block => {
                if let Some((index, next)) = blocks.remove(node.key) {
                    buffer.extend_from_slice(&nodes[index..next]);
                } else {
                    buffer.push(node);
                }
            },

            // Any non-block tags are preserved.
            _ => buffer.push(node),
        }
    }

    buffer
}

pub fn partial<'a>(key: Key<'a>) -> Vec<Node<'a>> {
    explode(key, Tag::Section, Tag::Partial, None)
}

pub fn escaped<'a>(key: Key<'a>) -> Vec<Node<'a>> {
    explode(key, Tag::Section, Tag::Escaped, None)
}

pub fn unescaped<'a>(key: Key<'a>) -> Vec<Node<'a>> {
    explode(key, Tag::Section, Tag::Unescaped, None)
}

pub fn content<'a>(text: &'a str) -> Node<'a> {
    Node {
        tag: Tag::Content,
        key: "",
        raw: text,
        len: 0,
    }
}

fn explode<'a>(
    key: Key<'a>,
    parent_tag: Tag,
    target_tag: Tag,
    nodes: Option<Vec<Node<'a>>>,
) -> Vec<Node<'a>> {
    let children = nodes.as_ref().map(|n| n.len()).unwrap_or(0);
    let dots = key.dots();
    let segments = key.into_segments();

    if dots == 0 {
        return vec![Node {
            tag: target_tag,
            key: segments[0],
            raw: "",
            len: 0,
        }];
    }

    segments
        .into_iter()
        .enumerate()
        .map(|(child, name)| {
            let last = child == dots;
            let tag = if last { target_tag } else { parent_tag };
            let next = if last { children } else { dots - child };

            Node {
                tag,
                key: name,
                raw: "",
                len: next,
            }
        })
        .chain(nodes.into_iter().flatten())
        .collect()
}
