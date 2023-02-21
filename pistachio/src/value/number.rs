use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Number {
    Positive(u64),
    /// Always less than zero.
    Negative(i64),
    /// Always finite.
    Float(f64),
}

impl fmt::Debug for Number {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Number({})", self)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.n {
            N::Positive(u) => fmt.write_str(itoa::Buffer::new().format(u)),
            N::Negative(i) => fmt.write_str(itoa::Buffer::new().format(i)),
            N::Float(f) => fmt.write_str(ryu::Buffer::new().format_finite(f)),
        }
    }
}

impl serde::Serialize for Number {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.n {
            N::Positive(u) => serializer.serialize_u64(u),
            N::Negative(i) => serializer.serialize_i64(i),
            N::Float(f) => serializer.serialize_f64(f),
        }
    }
}
