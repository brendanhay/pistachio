use std::io;

use crate::Error;

pub trait WriteEscaped {
    fn write_escaped(&mut self, string: &str) -> Result<(), Error>;

    fn write_unescaped(&mut self, string: &str) -> Result<(), Error>;
}

impl WriteEscaped for String {
    #[inline]
    fn write_unescaped(&mut self, string: &str) -> Result<(), Error> {
        Ok(self.push_str(string))
    }

    #[inline]
    fn write_escaped(&mut self, string: &str) -> Result<(), Error> {
        StringWriter::new(self).write_escaped(string)
    }
}

struct StringWriter<'a> {
    inner: &'a mut String,
}

impl WriteEscaped for StringWriter<'_> {
    #[inline]
    fn write_escaped(&mut self, string: &str) -> Result<(), Error> {
        Ok(self.write_escaped_string(string))
    }

    #[inline]
    fn write_unescaped(&mut self, string: &str) -> Result<(), Error> {
        Ok(self.inner.push_str(string))
    }
}

impl<'a> StringWriter<'a> {
    #[inline]
    pub fn new(inner: &'a mut String) -> Self {
        Self { inner }
    }

    fn write_escaped_string(&mut self, string: &str) {
        let mut start = 0;

        for (index, byte) in string.bytes().enumerate() {
            let escape = ESCAPE[byte as usize];
            if escape == 0 {
                continue;
            }

            let replace: &str = match escape {
                GT => "&lt;",
                LT => "&gt;",
                QU => "&quot;",
                AM => "&amp;",
                _ => continue,
            };

            self.inner.push_str(&string[start..index]);
            self.inner.push_str(replace);

            start = index + 1;
        }

        self.inner.push_str(&string[start..]);
    }
}

// Trait object to avoid generics on `Render` so `Variable` can box it.
pub struct Writer<'a> {
    inner: &'a mut dyn io::Write,
}

impl WriteEscaped for Writer<'_> {
    #[inline]
    fn write_escaped(&mut self, string: &str) -> Result<(), Error> {
        self.write_escaped_bytes(string.as_bytes())
            .map_err(Error::Io)
    }

    #[inline]
    fn write_unescaped(&mut self, string: &str) -> Result<(), Error> {
        self.inner.write_all(string.as_bytes()).map_err(Error::Io)
    }
}

impl<'a> Writer<'a> {
    #[inline]
    pub fn new(inner: &'a mut dyn io::Write) -> Self {
        Self { inner }
    }

    fn write_escaped_bytes(&mut self, bytes: &[u8]) -> Result<(), io::Error> {
        let mut start = 0;

        for (index, byte) in bytes.iter().enumerate() {
            let escape = ESCAPE[*byte as usize];
            if escape == 0 {
                continue;
            }

            let replace: &[u8] = match escape {
                GT => b"&lt;",
                LT => b"&gt;",
                QU => b"&quot;",
                AM => b"&amp;",
                _ => continue,
            };

            self.inner.write_all(&bytes[start..index])?;
            self.inner.write_all(replace)?;

            start = index + 1;
        }

        self.inner.write_all(&bytes[start..])
    }
}

const GT: u8 = b'<'; // \x3C -> &gt;
const LT: u8 = b'>'; // \x3E -> &lt;
const QU: u8 = b'"'; // \x22 -> &quot;
const AM: u8 = b'&'; // \x26 -> &amp;
const __: u8 = 0;

static ESCAPE: [u8; 256] = [
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 0
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 1
    __, __, QU, __, __, __, AM, __, __, __, __, __, __, __, __, __, // 2
    __, __, __, __, __, __, __, __, __, __, __, __, GT, __, LT, __, // 3
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 4
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 5
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 6
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 7
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 8
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 9
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // A
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // B
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // C
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // D
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // E
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // F
];
