//! Pistachio uses a [LALR parser] that requires the [mustache] grammar to be unambigous
//! and [context-free]. In order to achieve this while keeping the parsing productions
//! simple, this lexer tokenizes the source and switches between lexer modes as enter
//! `{{` and exit `}}` tags are encountered in the source.
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
//! Note: to simplify the production rules and keep things context free,
//! a custom lexer is used - see [`crate::parser::Lexer`].
//!
//! [lalr parser]: https://en.wikipedia.org/wiki/LALR_parser
//! [context-free]: https://en.wikipedia.org/wiki/Deterministic_context-free_language
//! [mustache]: https://jgonggrijp.gitlab.io/wontache/mustache.5.html
//! [set delimiter]: https://jgonggrijp.gitlab.io/wontache/mustache.5.html#Set-Delimiter

use std::str::pattern::Pattern;

use super::Token::{
    self,
    *,
};
use crate::Error;

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
        }
    }

    fn scan(&mut self) -> Result<Option<Spanned<'a>>, Error> {
        if self.source.is_empty() {
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
                    Err(Error::Lexer(Box::from(
                        format!("failed to scan {}, {}", self.position, &self.source)
                    )))
                }
            };
        }

        // It's the parser that skips comments since it provides better
        // error message when end }} delimiters aren't balanced.
        let start = self.position;
        let token = match self.mode {
            Mode::Content => {
                scan! {
                    self.token("{{", Enter)
                    self.until("{{", Content)
                    self.drain(Content)
                }
            },

            Mode::Control => {
                scan! {
                    self.token("}}", Leave)
                    self.token("/",  |_| Slash)
                    self.token("#",  |_| Pound)
                    self.token("^",  |_| Caret)
                    self.token(">",  |_| Greater)
                    self.token("<",  |_| Less)
                    self.token("$",  |_| Dollar)
                    self.token("&",  |_| Ampersand)
                    self.token("*",  |_| Asterisk)
                    self.token(".",  |_| Period)
                    self.token("=",  |_| Equals)
                    self.token("!",  |_| Bang)
                    self.until(&['.', ' ', '{', '}'], Ident)
                }
            },

            Mode::Literal => {
                scan! {
                    self.token("}}", Leave)
                    self.until("}}", String)
                }
            },
        }?;

        // Get the position before we skip trailing whitespace.
        let end = self.position;

        if token.skip_whitespace() {
            self.skip_whitespace();
        }

        // Flip the lexer mode based on the token we just scanned.
        let enter = matches!(self.previous, Some(Enter(..)));
        let dynamic = self.lookahead("*");

        match token {
            // {{
            Enter(..) => {
                self.mode = Mode::Control;
            },
            // }}
            Leave(..) => {
                self.mode = Mode::Content;
            },
            // {{/ | {{!
            Slash | Bang if enter => {
                self.mode = Mode::Literal;
            },
            // {{> | {{<
            Greater | Less if enter && !dynamic => {
                self.mode = Mode::Literal;
            },
            // *
            _ => {},
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
}

#[test]
fn print_tokens() {
    let source = r#"
<title>{{title}}</title>

<h1>{{ title }}</h1>

{{!single line comment}}

<div>
  {{#body}}
    {{content}}
  {{/body}}
</div>

{{#list}}
<div>
  {{#item}}
    <li>{{name}}: {{ age.seconds    }}</li>
  {{/item}}
</div>
{{/list}}


{{! multi

  line comment

foo.bar.baz}}
"#;

    let lexer = Lexer::new(source);
    let tokens = lexer.collect::<Vec<_>>();

    println!("{:#?}", tokens);
}
