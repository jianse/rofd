use crate::ser::XmlSerErr;
use serde::ser::{Impossible, SerializeSeq};
use serde::{Serialize, Serializer};

pub(crate) struct AttrValueSer {
    output: Option<String>,
}
impl AttrValueSer {
    pub fn convert_to_string<T>(value: &T) -> Result<Option<String>, XmlSerErr>
    where
        T: Serialize,
    {
        let mut ser = Self { output: None };
        value.serialize(&mut ser)?;
        Ok(ser.output)
    }
}
impl<'a> SerializeSeq for &'a mut AttrValueSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let msg = "[attr.rs] serialize_element";
        dbg!(msg);
        let result = AttrValueSer::convert_to_string(&value)?;
        if let Some(ele_v) = result {
            if let Some(output) = &mut self.output {
                if !output.is_empty() {
                    output.push(' ');
                    output.push_str(&ele_v);
                } else {
                    output.push_str(&ele_v);
                }
            } else {
                self.output = Some(ele_v);
            }
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // dbg!(&self.output);
        if let Some(output) = &self.output {
            if output.is_empty() {
                self.output = None;
            }
        }
        Ok(())
    }
}

impl<'a> Serializer for &'a mut AttrValueSer {
    type Ok = ();
    type Error = XmlSerErr;
    type SerializeSeq = Self;
    type SerializeTuple = Impossible<(), Self::Error>;
    type SerializeTupleStruct = Impossible<(), Self::Error>;
    type SerializeTupleVariant = Impossible<(), Self::Error>;
    type SerializeMap = Impossible<(), Self::Error>;
    type SerializeStruct = Impossible<(), Self::Error>;
    type SerializeStructVariant = Impossible<(), Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output = Some(v.to_string());
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output = Some(v.to_string());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output = Some(v.to_string());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output = Some(v.to_string());
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output = Some(v.to_string());
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.output = Some(v.to_string());
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output = Some(v.to_string());
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // HOW TO?
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.output = None;
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.output = None;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.output = Some(name.to_string());
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        let msg = format!("[attr.rs] serialize_unit_variant {name}::{variant}");
        dbg!(msg);
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // just forward to self handling primitive types
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // just forward to self handling primitive types
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let msg = "[attr.rs] serialize seq";
        dbg!(msg);
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(XmlSerErr::Message(
            "tuple can not be serialized to attr.".into(),
        ))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(XmlSerErr::Message(
            "tuple_struct can not be serialized to attr.".into(),
        ))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(XmlSerErr::Message(
            "map can not be serialized to attr.".into(),
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(XmlSerErr::Message(
            "tuple_variant can not be serialized to attr.".into(),
        ))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let msg = format!("[attr] serialize struct {name}");
        dbg!(msg);
        Err(XmlSerErr::Message(
            "struct can not be serialized to attr.".to_string(),
        ))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(XmlSerErr::Message(
            "struct_variant can not be serialized to attr.".into(),
        ))
    }
}
