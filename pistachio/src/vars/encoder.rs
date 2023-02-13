use std::{
    fmt,
    result,
};

use serde::ser;

use crate::{
    map::{
        self,
        Map,
    },
    vars::{
        Vars,
        VarsError,
    },
};

pub type Result<T, E = VarsError> = result::Result<T, E>;

#[derive(Default)]
pub struct Encoder;

impl Encoder {
    pub fn new() -> Encoder {
        Encoder::default()
    }
}

impl ser::Serializer for Encoder {
    type Ok = Vars;
    type Error = VarsError;

    type SerializeSeq = SerializeVec;
    type SerializeTuple = SerializeVec;
    type SerializeTupleStruct = SerializeVec;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeMap;
    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Vars> {
        Ok(Vars::Bool(v))
    }

    fn serialize_char(self, v: char) -> Result<Vars> {
        Ok(Vars::Str(v.to_string()))
    }

    fn serialize_u8(self, v: u8) -> Result<Vars> {
        Ok(Vars::Str(v.to_string()))
    }

    fn serialize_i8(self, v: i8) -> Result<Vars> {
        Ok(Vars::Str(v.to_string()))
    }

    fn serialize_u16(self, v: u16) -> Result<Vars> {
        Ok(Vars::Str(v.to_string()))
    }

    fn serialize_i16(self, v: i16) -> Result<Vars> {
        Ok(Vars::Str(v.to_string()))
    }

    fn serialize_u32(self, v: u32) -> Result<Vars> {
        Ok(Vars::Str(v.to_string()))
    }

    fn serialize_i32(self, v: i32) -> Result<Vars> {
        Ok(Vars::Str(v.to_string()))
    }

    fn serialize_i64(self, v: i64) -> Result<Vars> {
        Ok(Vars::Str(v.to_string()))
    }

    fn serialize_u64(self, v: u64) -> Result<Vars> {
        Ok(Vars::Str(v.to_string()))
    }

    fn serialize_f32(self, v: f32) -> Result<Vars> {
        Ok(Vars::Str(v.to_string()))
    }

    fn serialize_f64(self, v: f64) -> Result<Vars> {
        Ok(Vars::Str(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Vars> {
        Ok(Vars::Str(v.to_string()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Vars> {
        // FIXME: Perhaps this could be relaxed to just 'do nothing'
        Err(VarsError::UnsupportedType)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Vars> {
        // FIXME: Perhaps this could be relaxed to just 'do nothing'
        Ok(Vars::Str(variant.to_string()))
    }

    fn serialize_unit(self) -> Result<Vars> {
        Ok(Vars::Null)
    }

    fn serialize_none(self) -> Result<Vars> {
        Ok(Vars::Null)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Vars>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
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
            name: String::from(variant),
            map: map::with_capacity(len),
        })
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Vars>
    where
        T: ser::Serialize,
    {
        // Ignore newtype name
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Vars>
    where
        T: ser::Serialize,
    {
        // Ignore newtype name
        value.serialize(self)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Vars> {
        let vec = value.iter().map(|&b| Vars::Str(b.to_string())).collect();

        Ok(Vars::Vec(vec))
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
            name: String::from(variant),
            vec: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMap {
            map: map::with_capacity(len.unwrap_or(0)),
            next_key: None,
        })
    }
}

#[doc(hidden)]
pub struct SerializeVec {
    vec: Vec<Vars>,
}

#[doc(hidden)]
pub struct SerializeTupleVariant {
    name: String,
    vec: Vec<Vars>,
}

#[doc(hidden)]
pub struct SerializeMap {
    map: Map<String, Vars>,
    next_key: Option<String>,
}

#[doc(hidden)]
pub struct SerializeStructVariant {
    name: String,
    map: Map<String, Vars>,
}

impl ser::SerializeSeq for SerializeVec {
    type Ok = Vars;
    type Error = VarsError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        self.vec.push(Vars::encode(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Vars> {
        Ok(Vars::Vec(self.vec))
    }
}

impl ser::SerializeTuple for SerializeVec {
    type Ok = Vars;
    type Error = VarsError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Vars> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SerializeVec {
    type Ok = Vars;
    type Error = VarsError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Vars> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Vars;
    type Error = VarsError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        self.vec.push(Vars::encode(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Vars> {
        let mut object = map::with_capacity(1);

        object.insert(self.name, Vars::Vec(self.vec));

        Ok(Vars::Map(object))
    }
}

impl ser::SerializeMap for SerializeMap {
    type Ok = Vars;
    type Error = VarsError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        match Vars::encode(key)? {
            Vars::Str(s) => {
                self.next_key = Some(s);
                Ok(())
            },
            _ => Err(VarsError::KeyIsNotString),
        }
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        // XXX: from rust-mustache:
        // Taking the key should only fail if this gets called before
        // serialize_key, which is a bug in the library.
        let key = self.next_key.take().ok_or(VarsError::MissingElements)?;
        self.map.insert(key, Vars::encode(&value)?);

        Ok(())
    }

    fn end(self) -> Result<Vars> {
        Ok(Vars::Map(self.map))
    }
}

impl ser::SerializeStruct for SerializeMap {
    type Ok = Vars;
    type Error = VarsError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        ser::SerializeMap::serialize_key(self, key)?;
        ser::SerializeMap::serialize_value(self, value)
    }

    fn end(self) -> Result<Vars> {
        ser::SerializeMap::end(self)
    }
}

impl ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Vars;
    type Error = VarsError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        self.map.insert(String::from(key), Vars::encode(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Vars> {
        let mut object = map::with_capacity(1);

        object.insert(self.name, Vars::Map(self.map));

        Ok(Vars::Map(object))
    }
}
