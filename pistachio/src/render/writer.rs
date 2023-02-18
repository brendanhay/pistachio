use std::{
    convert::Infallible,
    fmt,
    io,
};

use super::Escape;

pub trait Writer {
    type Error;

    fn write_escape(&mut self, string: &str, escape: Escape) -> Result<(), Self::Error>;

    fn format_escape<D: fmt::Display>(
        &mut self,
        escape: Escape,
        display: D,
    ) -> Result<(), Self::Error>;
}

impl Writer for String {
    type Error = Infallible;

    #[inline]
    fn write_escape(&mut self, string: &str, escape: Escape) -> Result<(), Self::Error> {
        match escape {
            Escape::Html => EscapedString { inner: self }.write_escaped_str(string),
            Escape::None => self.push_str(string),
        }

        Ok(())
    }

    #[inline]
    fn format_escape<D: fmt::Display>(
        &mut self,
        escape: Escape,
        display: D,
    ) -> Result<(), Self::Error> {
        use std::fmt::Write as _;

        let _ = match escape {
            Escape::Html => write!(EscapedString { inner: self }, "{}", display),
            Escape::None => write!(self, "{}", display),
        };

        Ok(())
    }
}

pub struct EscapedString<'a> {
    inner: &'a mut String,
}

impl<'a> EscapedString<'a> {
    #[inline]
    pub fn new(inner: &'a mut String) -> Self {
        Self { inner }
    }

    #[inline]
    fn write_escaped_str(&mut self, string: &str) {
        let mut start = 0;

        for (index, byte) in string.bytes().enumerate() {
            let escape = ESCAPE[byte as usize];
            if escape == 0 {
                continue;
            }

            let replace: &str = match escape {
                GT => "&lt;",
                LT => "&gt;",
                QU => "&quote;",
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

impl<'a> fmt::Write for EscapedString<'a> {
    #[inline]
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_escaped_str(string);

        Ok(())
    }
}

pub struct EscapedWriter<W> {
    inner: W,
}

impl<W: io::Write> EscapedWriter<W> {
    #[inline]
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    #[inline]
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
                QU => b"&quote;",
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

impl<W: io::Write> io::Write for EscapedWriter<W> {
    #[inline]
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.write_escaped_bytes(bytes).map(|()| bytes.len())
    }

    #[inline]
    fn write_all(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.write_escaped_bytes(bytes)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<W: io::Write> Writer for EscapedWriter<W> {
    type Error = io::Error;

    #[inline]
    fn write_escape(&mut self, string: &str, escape: Escape) -> io::Result<()> {
        match escape {
            Escape::Html => self.write_escaped_bytes(string.as_bytes()),
            Escape::None => self.inner.write_all(string.as_bytes()),
        }
    }

    #[inline]
    fn format_escape<D: fmt::Display>(
        &mut self,
        escape: Escape,
        display: D,
    ) -> Result<(), Self::Error> {
        use io::Write as _;

        match escape {
            Escape::Html => write!(self.inner, "{}", display),
            Escape::None => write!(self, "{}", display),
        }
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
