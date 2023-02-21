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
        match *self {
            Number::Positive(u) => fmt.write_str(itoa::Buffer::new().format(u)),
            Number::Negative(i) => fmt.write_str(itoa::Buffer::new().format(i)),
            Number::Float(f) => fmt.write_str(ryu::Buffer::new().format_finite(f)),
        }
    }
}

impl serde::Serialize for Number {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Number::Positive(u) => serializer.serialize_u64(u),
            Number::Negative(i) => serializer.serialize_i64(i),
            Number::Float(f) => serializer.serialize_f64(f),
        }
    }
}

impl Number {
    #[inline]
    pub fn is_i64(&self) -> bool {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self {
            Number::Positive(v) => v <= i64::max_value() as u64,
            Number::Negative(_) => true,
            Number::Float(_) => false,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.as_i64().is_some()
    }

    #[inline]
    pub fn is_u64(&self) -> bool {
        #[cfg(not(feature = "arbitrary_precision"))]
        match *self {
            Number::Positive(_) => true,
            Number::Negative(_) | N::Float(_) => false,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.as_u64().is_some()
    }

    #[inline]
    pub fn is_f64(&self) -> bool {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self {
            Number::Float(_) => true,
            Number::Positive(_) | N::Negative(_) => false,
        }
        #[cfg(feature = "arbitrary_precision")]
        {
            for c in self.chars() {
                if c == '.' || c == 'e' || c == 'E' {
                    return self.parse::<f64>().ok().map_or(false, f64::is_finite);
                }
            }
            false
        }
    }

    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self {
            Number::Positive(n) => {
                if n <= i64::max_value() as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            },
            Number::Negative(n) => Some(n),
            Number::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.parse().ok()
    }

    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self {
            Number::Positive(n) => Some(n),
            Number::Negative(_) | N::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.parse().ok()
    }

    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Number::Positive(n) => Some(n as f64),
            Number::Negative(n) => Some(n as f64),
            Number::Float(n) => Some(n),
        }
        #[cfg(feature = "arbitrary_precision")]
        self.parse::<f64>().ok().filter(|float| float.is_finite())
    }

    #[inline]
    pub fn from_f64(f: f64) -> Option<Number> {
        if f.is_finite() {
            Some(Number::Float(f))
        } else {
            None
        }
    }
}

macro_rules! impl_from_unsigned {
    (
        $($ty:ty),*
    ) => {
        $(
            impl From<$ty> for Number {
                #[inline]
                fn from(u: $ty) -> Self {
                    Number::Positive(u as u64)
                }
            }
        )*
    };
}

macro_rules! impl_from_signed {
    (
        $($ty:ty),*
    ) => {
        $(
            impl From<$ty> for Number {
                #[inline]
                fn from(i: $ty) -> Self {
                    if i < 0 {
                        Number::Negative(i as i64)
                    } else {
                        Number::Positive(i as u64)
                    }
                }
            }
        )*
    };
}

impl_from_unsigned!(u8, u16, u32, u64, usize);
impl_from_signed!(i8, i16, i32, i64, isize);
