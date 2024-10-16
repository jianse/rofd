#![allow(unused)]

mod attr;

use crate::ser::attr::AttrValueSer;
use minidom::{Element, Node};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};
use std::fmt::Display;
use thiserror::Error;

enum XmlSerState {
    Seq,
    Ele,
}

pub struct XmlSer {
    output: Element,
    is_none: bool,
    state: XmlSerState,
    seq_output: Vec<Element>,
    // sub_ser: Option<Box<XmlSer>>,
}

impl XmlSer {
    pub fn new_with_prefix<N, NS>(name: N, namespace: NS, prefix: Option<String>) -> XmlSer
    where
        N: AsRef<str>,
        NS: Into<String>,
    {
        let ns = namespace.into();
        XmlSer {
            output: Element::builder(name, &ns)
                .prefix(prefix, &ns)
                .unwrap()
                .build(),
            is_none: false,
            state: XmlSerState::Ele,
            seq_output: vec![],
            // sub_ser: None,
        }
    }
    pub fn new<N, NS>(name: N, namespace: NS) -> XmlSer
    where
        N: AsRef<str>,
        NS: Into<String>,
    {
        let name = name.as_ref();
        let ns = namespace.into();
        XmlSer {
            output: Element::bare(name, ns),
            is_none: false,
            state: XmlSerState::Ele,
            seq_output: vec![],
            // sub_ser: None,
        }
    }
    // type Result<T> = std::result::Result<T, XmlSerErr>;

    pub fn der_to_element<T>(mut self, value: &T) -> Result<Element, XmlSerErr>
    where
        T: Serialize,
    {
        value.serialize(&mut self)?;
        Ok(self.output)
    }
}

#[derive(Error, Debug)]
pub enum XmlSerErr {
    #[error("xml serialization error")]
    Common,
    #[error("xml serialization error: {0}")]
    Message(String),
}
impl serde::ser::Error for XmlSerErr {
    fn custom<T: Display>(msg: T) -> Self {
        XmlSerErr::Message(msg.to_string())
    }
}

impl<'a> Serializer for &'a mut XmlSer {
    type Ok = ();
    type Error = XmlSerErr;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output.append_node(Node::Text(v.to_string()));
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output.append_node(Node::Text(v.to_string()));
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output.append_node(Node::Text(v.to_string()));
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output.append_node(Node::Text(v.to_string()));
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.is_none = true;
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let msg = format!("[xml_ser] begin serialize_newtype_variant {_name}:{variant}");
        dbg!(msg);
        let mut ser = XmlSer::new(variant, self.output.ns());
        value.serialize(&mut ser)?;
        self.output.append_child(ser.output);
        Ok(())
        // todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.state = XmlSerState::Seq;
        Ok(self)

        // todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        // dbg!(name);
        // todo!()
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let msg = format!("[xml_ser] serialize_struct_variant {name}:{variant}");
        dbg!(msg);
        let _ser = XmlSer::new(variant, self.output.ns());
        // self.sub_ser = Some(Box::new(ser));
        // TODO: MAYBE WE NEED NEW SER
        Ok(self)
    }
}

impl<'a> SerializeSeq for &'a mut XmlSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // todo!()
        let msg = "begin serializing element";
        dbg!(msg);
        let mut ser = XmlSer::new(self.output.name(), self.output.ns());
        value.serialize(&mut ser)?;
        self.seq_output.push(ser.output);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // self.output.
        // let mut nodes = self.output.take_nodes();
        // if nodes.is_empty() {
        //     Ok(())
        // } else {
        //     nodes.remove(nodes.len() - 1);
        //     for node in nodes {
        //         self.output.append_node(node);
        //     }
        //     Ok(())
        // }
        Ok(())
    }
}

impl<'a> SerializeTuple for &'a mut XmlSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeTupleStruct for &'a mut XmlSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeTupleVariant for &'a mut XmlSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeMap for &'a mut XmlSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // dbg!(key);
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeStruct for &'a mut XmlSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        dbg!(key);
        match key {
            s if s.starts_with("@") => {
                let v = AttrValueSer::to_string(&value)?;
                let name = s[1..].to_string();
                self.output.set_attr(name, v);
            }
            "$value" => {
                value.serialize(&mut **self)?;
            }
            _ => {
                let mut ser = XmlSer::new(key, self.output.ns());
                value.serialize(&mut ser)?;
                if !ser.is_none {
                    match ser.state {
                        XmlSerState::Seq => {
                            for ele in ser.seq_output {
                                self.output.append_child(ele);
                            }
                        }
                        XmlSerState::Ele => {
                            self.output.append_child(ser.output);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeStructVariant for &'a mut XmlSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
        // todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;
    #[test]
    fn ser_works() -> Result<()> {
        #[derive(Serialize)]
        struct A {
            #[serde(rename = "@Attr1")]
            attr1: String,

            #[serde(rename = "Ele1")]
            ele: String,

            #[serde(rename = "@Attr2")]
            option_attr: Option<String>,

            #[serde(rename = "VecEle")]
            ele2s: Vec<f32>,

            #[serde(rename = "@VecAttr")]
            vec_attr: Vec<String>,

            idx: usize,
        }
        let a = A {
            attr1: "foo".to_string(),
            ele: "bar".to_string(),
            option_attr: Some("baz".to_string()),
            ele2s: vec![1.0, 2.0],
            idx: 1,
            vec_attr: vec!["foo".to_string(), "bar".to_string()],
        };

        let ser = XmlSer::new_with_prefix("test", "https://123.com", None);
        let e = ser.der_to_element(&a)?;

        // into string
        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
        // dbg!(&xml_str);
        println!("{}", xml_str);
        Ok(())
    }
}
