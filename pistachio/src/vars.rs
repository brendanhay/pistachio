use std::{
    cell::RefCell,
    error,
    fmt,
};

mod encoder;

use serde::Serialize;

use crate::map::Map;

/// Error type to represent encoding failure.
///
/// This type is not intended to be matched exhaustively as new variants
/// may be added in future without a version bump.
#[derive(Debug)]
pub enum VarsError {
    NestedOptions,
    UnsupportedType,
    MissingElements,
    KeyIsNotString,
    NoDataToEncode,
    Message(String),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl error::Error for VarsError {}

impl serde::ser::Error for VarsError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        VarsError::Message(msg.to_string())
    }
}

impl fmt::Display for VarsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                VarsError::NestedOptions => "nested Option types are not supported",
                VarsError::UnsupportedType => "unsupported type",
                VarsError::MissingElements => "no elements in value",
                VarsError::KeyIsNotString => "key is not a string",
                VarsError::NoDataToEncode => "the encodable type created no data",
                VarsError::Message(ref s) => s,
                VarsError::__Nonexhaustive => unreachable!(),
            }
        )
    }
}

/// The mustache template variable representation. Similar to JSON, but with lambda support.
pub enum Vars {
    Null,
    Bool(bool),
    Str(String),
    Vec(Vec<Vars>),
    Map(Map<String, Vars>),
    Fun(RefCell<Box<dyn FnMut(String) -> String + Send>>),
}

impl Vars {
    pub fn encode<S: Serialize>(value: S) -> Result<Self, VarsError> {
        value.serialize(encoder::Encoder)
    }
}

impl fmt::Debug for Vars {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Vars::Null => write!(f, "Null"),
            Vars::Bool(b) => write!(f, "Bool({:?})", b),
            Vars::Str(s) => write!(f, "Str({})", s),
            Vars::Vec(v) => write!(f, "Vec({:?})", v),
            Vars::Map(m) => write!(f, "Map({:?})", m),
            Vars::Fun(_) => write!(f, "Fun(...)"),
        }
    }
}
