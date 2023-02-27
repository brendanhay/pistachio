use crate::{
    template::Name,
    Render,
};

pub type Var<'a> = &'a (dyn Render + 'a);

#[derive(Clone, Copy)]
pub struct Stack<'a> {
    a: Var<'a>,
    b: Var<'a>,
    c: Var<'a>,
    d: Var<'a>,
    e: Var<'a>,
    f: Var<'a>,
}

impl<'a> Stack<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            a: &(),
            b: &(),
            c: &(),
            d: &(),
            e: &(),
            f: &(),
        }
    }

    #[inline]
    pub fn push(self, frame: Var<'a>) -> Self {
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
            f: &(),
        }
    }

    #[inline]
    pub fn peek(&self) -> Var<'a> {
        self.a
    }

    #[inline]
    pub fn resolve(&self, name: &Name) -> Option<Var<'a>> {
        if name.is_dot() {
            Some(self.peek())
        } else {
            // Find the root of the name on the stack.
            let mut var = self.resolve_root(name.keys[0])?;

            // Walk the rest of the name's keys down the stack until resolution occurs.
            for tail in &name.keys[1..] {
                match var.resolve(*tail) {
                    Some(value) => {
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
    pub fn resolve_root(&self, key: &str) -> Option<Var<'a>> {
        macro_rules! try_get {
            ( $($field:expr),* ) => {
                $(
                    match $field.resolve(key) {
                        Some(value) => return Some(value),
                        None => {}
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
