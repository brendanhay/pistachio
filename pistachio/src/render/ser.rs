use std::{
    borrow::Cow,
    fmt,
};

use serde::{
    ser::Impossible,
    Serialize,
};

use crate::{
    map,
    Error,
    Result,
};

// impl serde::de::StdError for Error {
//     #[cfg(feature = "std")]
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         match &self.err.code {
//             ErrorCode::Io(err) => Some(err),
//             _ => None,
//         }
//     }
// }

pub struct Serializer;

pub enum Variable {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Vec(Vec<Variable>),
    Map(map::Map<Cow<'static, str>, Variable>),
    // Fun()
}

pub fn to_variable<T>(value: T) -> Result<Variable, Error>
where
    T: Serialize,
{
    value.serialize(Serializer)
}

impl serde::Serializer for Serializer {
    type Ok = Variable;
    type Error = Error;

    type SerializeSeq = SerializeVec;
    type SerializeTuple = SerializeVec;
    type SerializeTupleStruct = SerializeVec;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeMap;
    type SerializeStructVariant = SerializeStructVariant;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Variable> {
        Ok(Variable::Bool(value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Variable> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Variable> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Variable> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i64(self, value: i64) -> Result<Variable> {
        Ok(Variable::Number(
            itoa::Buffer::new().format(value).to_owned(),
        ))
    }

    fn serialize_i128(self, value: i128) -> Result<Variable> {
        Ok(Variable::Number(
            itoa::Buffer::new().format(value).to_owned(),
        ))
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<Variable> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<Variable> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<Variable> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<Variable> {
        Ok(Variable::Number(
            itoa::Buffer::new().format(value).to_owned(),
        ))
    }

    fn serialize_u128(self, value: u128) -> Result<Variable> {
        Ok(Variable::Number(
            itoa::Buffer::new().format(value).to_owned(),
        ))
    }

    // ryu::Buffer::new().format_finite(f).to_owned()
    #[inline]
    fn serialize_f32(self, value: f32) -> Result<Variable> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<Variable> {
        if value.is_finite() {
            Ok(Variable::Number(
                ryu::Buffer::new().format_finite(value).to_owned(),
            ))
        } else {
            Err(Error::NonFiniteFloat)
        }
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<Variable> {
        let mut s = String::new();
        s.push(value);
        Ok(Variable::String(s.into()))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Variable> {
        Ok(Variable::String(value.into()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Variable> {
        let vec = value
            .iter()
            .map(|&b| Variable::Number(itoa::Buffer::new().format(b).to_owned()))
            .collect();
        Ok(Variable::Vec(vec))
    }

    #[inline]
    fn serialize_unit(self) -> Result<Variable> {
        Ok(Variable::Null)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Variable> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Variable> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Variable>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Variable>
    where
        T: ?Sized + Serialize,
    {
        let mut values = map::with_capacity(1);

        values.insert(variant.into(), to_variable(value)?);

        Ok(Variable::Map(values))
    }

    #[inline]
    fn serialize_none(self) -> Result<Variable> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Variable>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeVec {
            vec: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SerializeTupleVariant {
            name: variant.into(),
            vec: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMap {
            map: map::with_capacity(len.unwrap_or(0)),
            next_key: None,
        })
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(SerializeStructVariant {
            name: variant.into(),
            map: map::with_capacity(len),
        })
    }

    fn collect_str<T>(self, value: &T) -> Result<Variable>
    where
        T: ?Sized + fmt::Display,
    {
        Ok(Variable::String(value.to_string().into()))
    }
}

pub struct SerializeVec {
    vec: Vec<Variable>,
}

impl serde::ser::SerializeSeq for SerializeVec {
    type Ok = Variable;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.vec.push(to_variable(value)?);
        Ok(())
    }

    fn end(self) -> Result<Variable> {
        Ok(Variable::Vec(self.vec))
    }
}

impl serde::ser::SerializeTuple for SerializeVec {
    type Ok = Variable;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Variable> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl serde::ser::SerializeTupleStruct for SerializeVec {
    type Ok = Variable;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Variable> {
        serde::ser::SerializeSeq::end(self)
    }
}

pub struct SerializeTupleVariant {
    name: &'static str,
    vec: Vec<Variable>,
}

impl serde::ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Variable;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.vec.push(to_variable(value)?);

        Ok(())
    }

    fn end(self) -> Result<Variable> {
        let mut map = map::with_capacity(1);

        map.insert(self.name.into(), Variable::Vec(self.vec));

        Ok(Variable::Map(map))
    }
}

pub struct SerializeMap {
    map: map::Map<Cow<'static, str>, Variable>,
    next_key: Option<Cow<'static, str>>,
}

impl serde::ser::SerializeMap for SerializeMap {
    type Ok = Variable;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            SerializeMap { next_key, .. } => {
                *next_key = Some(key.serialize(MapKeySerializer)?);
                Ok(())
            },
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key = self.next_key.take();
        // Panic because this indicates a bug in the program rather than an
        // expected failure.
        let key = key.expect("serialize_value called before serialize_key");

        self.map.insert(key, to_variable(value)?);

        Ok(())
    }

    fn end(self) -> Result<Variable> {
        Ok(Variable::Map(self.map))
    }
}

struct MapKeySerializer;

impl serde::Serializer for MapKeySerializer {
    type Ok = Cow<'static, str>;
    type Error = Error;

    type SerializeSeq = Impossible<Cow<'static, str>, Error>;
    type SerializeTuple = Impossible<Cow<'static, str>, Error>;
    type SerializeTupleStruct = Impossible<Cow<'static, str>, Error>;
    type SerializeTupleVariant = Impossible<Cow<'static, str>, Error>;
    type SerializeMap = Impossible<Cow<'static, str>, Error>;
    type SerializeStruct = Impossible<Cow<'static, str>, Error>;
    type SerializeStructVariant = Impossible<Cow<'static, str>, Error>;

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(variant.into())
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok> {
        Err(Error::KeyMustBeString)
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        Ok(itoa::Buffer::new().format(value).to_owned().into())
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        Ok(itoa::Buffer::new().format(value).to_owned().into())
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok> {
        Err(Error::KeyMustBeString)
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok> {
        Err(Error::KeyMustBeString)
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        Ok({
            let mut s = String::with_capacity(1);
            s.push(value);
            s.into()
        })
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        Ok(value.to_owned().into())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok> {
        Err(Error::KeyMustBeString)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::KeyMustBeString)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::KeyMustBeString)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::KeyMustBeString)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::KeyMustBeString)
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::KeyMustBeString)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::KeyMustBeString)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::KeyMustBeString)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::KeyMustBeString)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::KeyMustBeString)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::KeyMustBeString)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::KeyMustBeString)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::KeyMustBeString)
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + fmt::Display,
    {
        Ok(value.to_string().into())
    }
}

impl serde::ser::SerializeStruct for SerializeMap {
    type Ok = Variable;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Variable> {
        serde::ser::SerializeMap::end(self)
    }
}

pub struct SerializeStructVariant {
    name: &'static str,
    map: map::Map<Cow<'static, str>, Variable>,
}

impl serde::ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Variable;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.map.insert(key.into(), to_variable(value)?);
        Ok(())
    }

    fn end(self) -> Result<Variable> {
        let mut map = map::with_capacity(1);

        map.insert(self.name.into(), Variable::Map(self.map));

        Ok(Variable::Map(map))
    }
}
