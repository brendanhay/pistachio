use std::fmt;

pub use self::{
    map::Map,
    number::Number,
    ser::{
        Error,
        Serializer,
    },
};

mod map;
mod number;
mod ser;

#[derive(Copy, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Vec(Vec<Value>),
    Map(Map<String, Value>),
    Fun(Box<Fn(String) -> String + Send>),
}

impl fmt::Debug for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => fmt.write_str("Null"),
            Value::Bool(boolean) => write!(fmt, "Bool({})", boolean),
            Value::Number(number) => Debug::fmt(number, fmt),
            Value::String(string) => write!(fmt, "String({:?})", string),
            Value::Vec(vec) => {
                fmt.write_str("Vec ")?;
                fmt::Debug::fmt(vec, fmt)
            },
            Value::Map(map) => {
                fmt.write_str("Map ")?;
                fmt::Debug::fmt(map, fmt)
            },
            Value::Fun(_) => {
                write!(fmt, "Fun(String) -> String")
            },
        }
    }
}

// impl serde::Serialize for Value {
//     #[inline]
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match self {
//             Value::Null => serializer.serialize_unit(),
//             Value::Bool(b) => serializer.serialize_bool(*b),
//             Value::Number(n) => n.serialize(serializer),
//             Value::String(s) => serializer.serialize_str(s),
//             Value::Vec(v) => v.serialize(serializer),
//             Value::Map(m) => {
//                 use serde::ser::SerializeMap;
//                 let mut map = tri!(serializer.serialize_map(Some(m.len())));
//                 for (k, v) in m {
//                     tri!(map.serialize_entry(k, v));
//                 }
//                 map.end()
//             },
//         }
//     }
// }

pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: serde::Serialize,
{
    value.serialize(Serializer)
}
