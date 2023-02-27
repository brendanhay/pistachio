use std::fmt;

use crate::parser::Spanned;

/// A non-empty list of dotted keys such as `foo.bar.baz`.
#[derive(Debug, Clone)]
pub struct Name<'a> {
    pub start: usize,
    pub keys: Vec<&'a str>,
}

impl Name<'_> {
    #[inline]
    pub fn is_dot(&self) -> bool {
        self.keys.len() == 1 && self.keys[0] == "."
    }

    pub fn path(&self) -> Option<&str> {
        self.keys.first().copied()
    }
}

// The lexer emits unparsed closing tags so we can error if the tags are
// unbalanced, which is more useful than a "failed expecting . | IDENT"
// when parsing a closing tag style error.
impl PartialEq<Name<'_>> for Name<'_> {
    fn eq(&self, other: &Name<'_>) -> bool {
        println!("{:?} == {:?}", &self, &other);

        // Normalize
        self.keys.join(".").eq(&other.keys.join("."))
    }
}

// This is used when displaying errors to the user.
impl fmt::Display for Name<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_assert!(!self.keys.is_empty(), "name contains no keys");

        if self.keys.is_empty() {
            return write!(f, "<unknown>");
        }

        f.write_str(self.keys[0])?;
        for key in &self.keys[1..] {
            write!(f, ".{}", key)?;
        }

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
