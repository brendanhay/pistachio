use std::{
    borrow::Cow,
    fmt,
    io,
};

use serde::Serialize;

use crate::{
    parser::{
        Lexer,
        Parser,
    },
    render::{
        Context,
        EscapedWriter,
        Render,
    },
    vars,
    Error,
    Loader,
    LoadingDisabled,
    Vars,
};

/// Represents a parsed and normalised template.
#[derive(Debug)]
pub struct Template<'a> {
    pub(crate) size_hint: usize,
    pub(crate) nodes: Vec<Node<'a>>,
    source: Cow<'a, str>,
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
        })
    }

    pub fn render(&self, vars: &Vars) -> String {
        // let data = encoder::to_data(data)?;
        let mut capacity = Render::size_hint(vars, self);

        // Add 25% for escaping and various expansions.
        capacity += capacity / 4;

        let mut buffer = String::with_capacity(capacity);
        let _ = self.render_to_string(vars, &mut buffer);

        buffer
    }

    pub fn render_to_string(&self, vars: &Vars, buffer: &mut String) {
        // Writing to a String is Infallible
        let _ = Context::new(&self.nodes).push(vars).render(buffer);
    }

    pub fn render_to_writer<S, W>(&self, vars: &S, writer: &mut W) -> Result<(), Error>
    where
        S: Serialize,
        W: io::Write,
    {
        let vars = Vars::encode(vars)?;
        let mut writer = EscapedWriter::new(writer);
        let () = Context::new(&self.nodes).push(&vars).render(&mut writer)?;

        Ok(())
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

#[derive(Debug, Clone, Copy)]
pub struct Node<'a> {
    pub tag: Tag,
    pub key: &'a str,
    pub raw: &'a str,
    pub len: usize,
}

/// The grammar production `Key` representing a non-empty list of dotted
/// keys such as `foo.bar.baz`.
pub struct Key<'a> {
    // Invariant: dots.len() > 0, which the grammar guarantees.
    //
    // This is more convenient for equality/iterators than something like:
    //   head: &'a str,
    //   tail: Vec<&'a str>,
    segments: Vec<&'a str>,
}

impl fmt::Display for Key<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.segments.join("."))
    }
}

impl PartialEq<str> for Key<'_> {
    fn eq(&self, other: &str) -> bool {
        self.segments.iter().map(|s| *s).eq(other.split('.'))
    }
}

impl<'a> Key<'a> {
    pub fn new(head: &'a str, mut tail: Vec<&'a str>) -> Self {
        tail.insert(0, head);
        Self { segments: tail }
    }

    pub fn dots(&self) -> usize {
        self.segments.len() - 1
    }

    pub fn into_segments(self) -> Vec<&'a str> {
        self.segments
    }
}
