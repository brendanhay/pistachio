//! Pistachio uses a [LALR parser] that requires the [mustache] grammar to be unambigous
//! and [context-free]. In order to achieve this while keeping the parsing productions
//! simple, this lexer tokenizes the source and switches between lexer modes as enter `{{`
//! and exit `}}` tags are encountered in the source.
//!
//! You can think of this as multiple sub-lexers that are context-aware and produce
//! different token streams depending on whether the raw text or the text between
//! `{{ .. }}` tags is being tokenized. Nominally, this means the lexer step is
//! context-aware and the parser is context-free.
//!
//! In addition to the the above, there are additional reasons to specifically use
//! a hand-rolled lexer:
//!
//! * No dependencies.
//! * No regexes - ie. no regex crate, see above.
//! * Better error messages.
//! * Support mustache's [Set Delimiter] feature.
//!
//! [lalr parser]: https://en.wikipedia.org/wiki/LALR_parser
//! [context-free]: https://en.wikipedia.org/wiki/Deterministic_context-free_language
//! [mustache]: https://jgonggrijp.gitlab.io/wontache/mustache.5.html
//! [set delimiter]: https://jgonggrijp.gitlab.io/wontache/mustache.5.html#Set-Delimiter

use std::str::pattern::Pattern;

use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Eof(&'a str, Layout),    // Raw textual content leading up to EOF.
    Ident(&'a str),          // An identifier - the single component of a key, no dots!
    String(&'a str),         // A string inside a tag.
    Enter(&'a str, &'a str), // `{{` tag start
    Leave(&'a str, Layout),  // `}}` tag end
    RSlash,                  // `/`
    Hash,                    // #
    Caret,                   // ^
    LAngle,                  // <
    RAngle,                  // >
    LBrace,                  // {
    Dollar,                  // $
    Bang,                    // !
    Ampersand,               // &
    Asterisk,                // *
    Period,                  // .
    Equals,                  // =
}

use std::fmt;

use Token::*;

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Eof(s, _) => write!(f, "{}", s),
            Ident(s) => write!(f, "{}", s),
            String(s) => write!(f, "{}", s),
            Enter(s, d) => write!(f, "{}{}", s, d),
            Leave(d, _) => write!(f, "{}", d),
            RSlash => write!(f, "/"),
            Hash => write!(f, "#"),
            Caret => write!(f, "^"),
            LAngle => write!(f, "<"),
            RAngle => write!(f, ">"),
            LBrace => write!(f, "{{"),
            Dollar => write!(f, "$"),
            Bang => write!(f, "!"),
            Ampersand => write!(f, "&"),
            Asterisk => write!(f, "*"),
            Period => write!(f, "."),
            Equals => write!(f, "="),
        }
    }
}

/// If a tag is preceeded by a newline + whitespace and a trailining newline,
/// it is considered standalone.
#[derive(Debug, Clone, Copy)]
pub enum Layout {
    Preserve,
    Standalone,
}

impl Layout {
    pub fn trim(self, text: &str) -> &str {
        match self {
            Layout::Preserve => text,
            Layout::Standalone => text.trim_end_matches(&[' ']),
        }
    }
}

enum Braces {
    Two,
    Three,
}

enum Mode {
    /// Outside mucking around in the raw text.
    Content,

    /// Entered a tag that supports dotted identifiers, such as `foo.bar.baz`.
    Control,

    /// Inside a tag that supports arbitrary strings, like filepaths.
    Literal,
}

type Spanned<'a> = (usize, Token<'a>, usize);

pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
    mode: Mode,
    previous: Option<Token<'a>>,
    braces: Braces,
    layout: Layout,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Spanned<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan().transpose()
    }
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source,
            position: 0,
            mode: Mode::Content,
            previous: None,
            braces: Braces::Two,
            layout: Layout::Standalone,
        }
    }

    fn scan(&mut self) -> Result<Option<Spanned<'a>>, Error> {
        if self.is_eof() {
            return Ok(None);
        }

        macro_rules! scan {
            ( $( $expr:expr )* ) => {
                if false { unreachable!() }
                $(
                    else if let Some(token) = $expr {
                        Ok(token)
                    }
                )*
                else {
                    Err(Error::ParsingFailed(
                        (self.position, self.position),
                        format!("failed to scan {}", &self.source)
                    ))
                }
            };
        }

        // It's the parser that skips comments since it provides better
        // error message when end }} delimiters aren't balanced.

        let layout = self.layout;
        let start = self.position;
        let mut token = match self.mode {
            Mode::Content => {
                scan! {
                    self.until_inclusive("{{", Enter)
                    self.drain(|s| Eof(s, layout))
                }
            },

            Mode::Control => {
                scan! {
                    match self.braces {
                        Braces::Three => self.token("}}}", |d| Leave(d, layout)),
                        Braces::Two   => self.token("}}",  |d| Leave(d, layout)),
                    }

                    self.token("/",  |_| RSlash)
                    self.token("#",  |_| Hash)
                    self.token("^",  |_| Caret)
                    self.token("<",  |_| LAngle)
                    self.token(">",  |_| RAngle)
                    self.token("$",  |_| Dollar)
                    self.token("*",  |_| Asterisk)
                    self.token(".",  |_| Period)
                    self.token("=",  |_| Equals)
                    self.token("!",  |_| Bang)
                    self.token("&",  |_| Ampersand)
                    self.token("{",  |_| LBrace)
                    self.until(&['.', ' ', '{', '}'], Ident)
                }
            },

            Mode::Literal => {
                scan! {
                    self.token("}}", |d| Leave(d, layout))
                    self.until("}}", String)
                }
            },
        }?;

        // Get the position before we skip trailing whitespace.
        let end = self.position;
        // Avoid running multiple times when inlined into the match guard.
        let dynamic = self.lookahead("*");
        // Note if the previously parsed token was tag entry, so we can
        // determine what to do with subsequent control characters.
        let enter = matches!(self.previous, Some(Enter(..)));

        match token {
            // {{
            Enter(content, _delim) => {
                // XXX: it would be safe to iterate over bytes here.
                // Was there a newline + any amount of whitspace prior to the enter tag?
                for c in content.chars().rev() {
                    if c == '\n' {
                        self.layout = Layout::Standalone;
                        break;
                    }

                    if !c.is_whitespace() {
                        self.layout = Layout::Preserve;
                        break;
                    }
                }

                self.skip_whitespace();
                self.mode = Mode::Control;
            },
            // }}
            Leave(delim, layout) => {
                // BUG: non-control tags like `{{ ..  }}` shouldn't trim whitespace.
                if matches!(layout, Layout::Standalone) {
                    if !self.skip_newline() {
                        token = Leave(delim, Layout::Preserve);
                    }
                }

                self.braces = Braces::Two;
                // self.layout = Layout::Preserve;
                self.mode = Mode::Content;
            },
            // {{/ | {{!
            RSlash | Bang if enter => {
                self.skip_whitespace();
                self.mode = Mode::Literal;
            },
            // {{> | {{<
            RAngle | LAngle if enter && !dynamic => {
                self.skip_whitespace();
                self.mode = Mode::Literal;
            },
            // {{{
            LBrace if enter => {
                self.skip_whitespace();
                self.braces = Braces::Three;
                self.layout = Layout::Preserve;
            },
            // {{<ident>
            Ident(..) | Ampersand if enter => {
                self.layout = Layout::Preserve;
                self.skip_whitespace();
            },
            // .
            Period => {
                // Explicitly don't skip whitespace.
            },
            _ => {
                self.skip_whitespace();
            },
        };

        self.previous = Some(token);

        Ok(Some((start, token, end)))
    }

    /// XXX: panic if patterns are empty, otherwise the lexer won't be productive.

    /// Consume a token on match.
    fn token<F>(&mut self, token: &str, action: F) -> Option<Token<'a>>
    where
        F: FnOnce(&'a str) -> Token<'a>,
    {
        if token.is_prefix_of(self.source) {
            Some(action(self.advance(token.len())?))
        } else {
            None
        }
    }

    /// Consume everything up to (but excluding) the pattern on match.
    ///
    /// Greedy: consumes all remaining input if the pattern isn't found.
    fn until<P, F>(&mut self, pattern: P, action: F) -> Option<Token<'a>>
    where
        P: Pattern<'a>,
        F: FnOnce(&'a str) -> Token<'a>,
    {
        match self.source.match_indices(pattern).next() {
            Some((0, _)) => None,
            Some((start, _)) => Some(action(self.advance(start)?)),
            None => self.drain(action),
        }
    }

    /// Consume everything up to (and including) the pattern on match.
    ///
    /// Non-greedy: returns `None` if the pattern isn't found.
    fn until_inclusive<F>(&mut self, token: &str, action: F) -> Option<Token<'a>>
    where
        F: FnOnce(&'a str, &'a str) -> Token<'a>,
    {
        match self.source.match_indices(token).next() {
            Some((0, _)) => Some(action("", self.advance(token.len())?)),
            Some((start, _)) => Some(action(self.advance(start)?, self.advance(token.len())?)),
            None => None,
        }
    }

    /// Consume the remaining source.
    fn drain<F>(&mut self, action: F) -> Option<Token<'a>>
    where
        F: FnOnce(&'a str) -> Token<'a>,
    {
        let text = self.source;
        let text = self.advance(text.len())?;

        Some(action(text))
    }

    /// Check if the next token matches without consuming source. Whitespace is ignored.
    fn lookahead(&mut self, token: &str) -> bool {
        token.is_prefix_of(self.source.trim_start())
    }

    /// Advance the source position by `count` characters.
    fn advance(&mut self, count: usize) -> Option<&'a str> {
        let text = &self.source[0..count];

        self.source = self.source.get(count..)?;
        self.position += count;

        Some(text)
    }

    /// Skip whitespace, updating the position.
    fn skip_whitespace(&mut self) {
        let mut count = 0usize;

        self.source = self.source.trim_start_matches(|c: char| {
            let ws = c.is_whitespace();
            count += 1;
            ws
        });

        self.position += count;
    }

    /// Skip a single newline, updating the position.
    fn skip_newline(&mut self) -> bool {
        if self.source.get(0..2) == Some("\r\n") {
            self.advance(2);
            return true;
        }

        if self.source.get(0..1) == Some("\n") {
            self.advance(1);
            return true;
        }

        self.is_eof()
    }

    fn is_eof(&mut self) -> bool {
        self.source.is_empty()
    }
}

#[test]
fn print_tokens() {
    // let source = "{{#bool}}\n* first\n{{/bool}}\n* {{two}}\n{{#bool}}\n* third\n{{/bool}}\n";
    // let source = "Begin.\n{{! Comment Block! }}\nEnd.\n";
    // let source = "|{{  string  }}|";
    // let source = "{{#boolean}}This should be rendered.{{/boolean}}";
    // let source = "Hello, {{subject}}!\n";
    // let source = "{{#foo}}{{.}} is {{foo}}{{/foo}}";
    // let source = "{{#list}}({{#.}}{{.}}{{/.}}){{/list}}";
    // let source = " | {{#boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n";
    // let source = " | {{#boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n";
    let source = "|\r\n{{#boolean}}\r\n{{/boolean}}\r\n|";
    let source = r#""{{person.name}}" == "{{#person}}{{name}}{{/person}}""#;
    let source = "{{#a.b.c}}Here{{/a.b.c}}";
    let lexer = Lexer::new(source);
    let tokens = lexer.collect::<Vec<_>>();

    println!("{:#?}", tokens);
    println!("{:?}", source);
}
