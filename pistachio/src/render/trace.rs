#[derive(Debug, Clone, Copy, Default)]
struct Trace<'a> {
    a: Option<&'a str>,
    b: Option<&'a str>,
    c: Option<&'a str>,
    d: Option<&'a str>,
    e: Option<&'a str>,
    f: Option<&'a str>,
    g: Option<&'a str>,
    h: Option<&'a str>,
    i: Option<&'a str>,
    j: Option<&'a str>,
}

impl<'a> Trace<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(self, name: &'a str) -> Self {
        Self {
            a: Some(name),
            b: self.a,
            c: self.b,
            d: self.c,
            e: self.d,
            f: self.e,
            g: self.f,
            h: self.g,
            i: self.h,
            j: self.i,
        }
    }
}
