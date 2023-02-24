use super::Variable;
use crate::template::Name;

#[derive(Debug, Clone, Copy, Default)]
pub struct Stack<'a> {
    a: &'a Variable,
    b: &'a Variable,
    c: &'a Variable,
    d: &'a Variable,
    e: &'a Variable,
    f: &'a Variable,
}

impl<'a> Stack<'a> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn push(self, frame: &'a Variable) -> Self {
        Self {
            a: frame,
            b: self.a,
            c: self.b,
            d: self.c,
            e: self.d,
            f: self.e,
        }
    }

    #[inline]
    pub fn pop(self) -> Self {
        Self {
            a: self.b,
            b: self.c,
            c: self.d,
            d: self.e,
            e: self.f,
            f: &Variable::Null,
        }
    }

    #[inline]
    pub fn peek(&self) -> &Variable {
        self.a
    }

    #[inline]
    pub fn resolve(&self, name: &Name) -> Option<&Variable> {
        if name.is_dot() {
            Some(self.peek())
        } else {
            // Find the root of the name on the stack.
            let mut var = self.resolve_map(name.keys[0])?;

            // Walk the rest of the name's keys down the stack until resolution occurs.
            for tail in &name.keys[1..] {
                match var {
                    Variable::Map(m) => {
                        let Some(value) = m.get(*tail) else { return None; };
                        var = value;
                    },
                    _ => {
                        return None;
                    },
                }
            }

            Some(var)
        }
    }

    #[inline]
    pub fn resolve_map(&self, key: &str) -> Option<&Variable> {
        macro_rules! try_get {
            ( $($field:expr),* ) => {
                $(
                    if let Variable::Map(m) = $field {
                        match m.get(key) {
                            Some(value) => return Some(value),
                            None => {}
                        }
                     }
                )*
            };
        }

        try_get! {
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.f
        }

        None
    }
}
