#![allow(unused)]

mod attr;

use crate::ser::attr::AttrValueSer;
use minidom::{Element, IntoAttributeValue, Node};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};
use std::fmt::{format, Display};
use std::sync::atomic::{AtomicUsize, Ordering};
use thiserror::Error;

macro_rules! write_primitive {
    ($method:ident ( $ty:ty )) => {
        fn $method(self, value: $ty) -> Result<Self::Ok, Self::Error> {
            self.output.append_node(Node::Text(v.to_string()));
            Ok(())
        }
    };
}

#[derive(Debug)]
pub enum Output {
    Ele(Element),
    Vec(Vec<Element>),
    Empty,
}

impl Output {
    fn append_child(&mut self, out: Output) -> Result<(), XmlSerErr> {
        match self {
            Output::Ele(e) => {
                match out {
                    Output::Ele(o) => {
                        e.append_child(o);
                    }
                    Output::Vec(l) => {
                        for o in l.into_iter() {
                            e.append_child(o);
                        }
                    }
                    Output::Empty => {}
                }
                Ok(())
            }
            _ => Err(XmlSerErr::Common),
        }
    }
    fn append_child_ele(&mut self, ele: Element) -> Result<(), XmlSerErr> {
        match self {
            Output::Ele(e) => {
                e.append_child(ele);
                Ok(())
            }
            _ => Err(XmlSerErr::Common),
        }
    }
    fn set_attr<S: Into<String>, V: IntoAttributeValue>(&mut self, key: S, value: V) {}
}

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_ID: usize = usize::MAX / 2;

fn generate_id() -> usize {
    // 检查两次溢出，否则直接加一可能导致溢出
    let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_val > MAX_ID {
        panic!("Factory ids overflowed");
    }
    GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let next_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("Factory ids overflowed");
    }
    next_id
}

const SER_TAG: &str = "[xml_ser] ";
#[derive(Debug, PartialEq)]
enum XmlSerState {
    Seq,
    Ele,
    StructVariant,
}

pub struct XmlSer {
    name: String,
    ns: String,
    uid: usize,
    output: Output,
    is_none: bool,
    state: XmlSerState,
    seq_output: Vec<Element>,
    temp_ele: Option<Element>,
    xml_seq_ser: XmlSeqSer,
}

impl XmlSer {
    pub fn new_with_prefix<N, NS>(name: N, namespace: NS, prefix: Option<String>) -> XmlSer
    where
        N: AsRef<str>,
        NS: Into<String>,
    {
        let ns = namespace.into();
        let name = name.as_ref();
        let ele = Element::builder(name, &ns)
            .prefix(prefix, &ns)
            .unwrap()
            .build();
        XmlSer {
            name: String::from(name),
            ns,
            uid: generate_id(),
            output: Output::Ele(ele),
            is_none: false,
            state: XmlSerState::Ele,
            seq_output: vec![],
            // sub_ser: None,
            temp_ele: None,
            xml_seq_ser: XmlSeqSer::new_empty(),
        }
    }
    pub fn new<N, NS>(name: N, namespace: NS) -> XmlSer
    where
        N: AsRef<str>,
        NS: Into<String>,
    {
        let name = name.as_ref();
        let ns = namespace.into();
        let ele = Element::bare(name, &ns);
        // ele.set_attr()
        XmlSer {
            name: String::from(name),
            ns,
            uid: generate_id(),
            output: Output::Ele(ele),
            is_none: false,
            state: XmlSerState::Ele,
            seq_output: vec![],
            temp_ele: None,
            xml_seq_ser: XmlSeqSer::new_empty(),
        }
    }
    // type Result<T> = std::result::Result<T, XmlSerErr>;

    pub fn der_to_element<T>(mut self, value: &T) -> Result<Element, XmlSerErr>
    where
        T: Serialize,
    {
        value.serialize(&mut self)?;
        match self.output {
            Output::Ele(e) => Ok(e),
            _ => Err(XmlSerErr::Common),
        }
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
        match &mut self.output {
            Output::Ele(e) => {
                e.append_node(Node::Text(v.to_string()));
                Ok(())
            }
            _ => Err(XmlSerErr::Common),
        }
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
        self.serialize_str(v.to_string().as_str())
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
        match &mut self.output {
            Output::Ele(e) => {
                e.append_node(Node::Text(v.to_string()));
                Ok(())
            }
            _ => Err(XmlSerErr::Common),
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        match &mut self.output {
            Output::Ele(e) => {
                e.append_node(Node::Text(v.to_string()));
                Ok(())
            }
            _ => Err(XmlSerErr::Common),
        }
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        match &mut self.output {
            Output::Ele(e) => {
                e.append_node(Node::Text(v.to_string()));
                Ok(())
            }
            _ => Err(XmlSerErr::Common),
        }
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        if (self.state == XmlSerState::Ele) {
            self.is_none = true;
        }
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
        let uid = self.uid;
        let msg = format!("{SER_TAG}{uid} serialize_unit_variant {name}::{variant}");
        dbg!(msg);
        let ele = Element::bare(variant, &self.ns);
        self.output.append_child_ele(ele)?;
        Ok(())
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        dbg!(name);
        value.serialize(self)
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
        let uid = self.uid;
        let msg = format!("{SER_TAG}{uid} begin serialize_newtype_variant {_name}:{variant}");
        dbg!(msg);
        let mut ser = XmlSer::new(variant, self.ns.clone());
        value.serialize(&mut ser)?;

        self.output.append_child(ser.output)?;
        Ok(())
        // todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let uid = self.uid;
        let msg = format!("{SER_TAG}{uid} begin serialize_seq");
        dbg!(msg);
        self.state = XmlSerState::Seq;
        // self.xml_seq_ser.output =self.seq_output;

        Ok(self)
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
        let uid = self.uid;
        let msg = format!("{SER_TAG}{uid} begin serialize_struct {name}");
        dbg!(msg);
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let uid = self.uid;
        let msg = format!("{SER_TAG}{uid} serialize_struct_variant {name}:{variant}");
        dbg!(msg);

        // self.state = XmlSerState::StructVariant;
        // let element = Element::bare(variant, self.output.ns());
        // let out = std::mem::replace(&mut self.output, element);
        // self.temp_ele = Some(out);
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
        let uid = self.uid;
        let msg = format!("{SER_TAG}{uid} begin serialize seq element");
        dbg!(msg);
        // let mut ser = XmlSer::new(self.output.name(), self.output.ns());
        // value.serialize(&mut ser)?;
        // self.seq_output.push(ser.output);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let uid = self.uid;
        let msg = format!("{SER_TAG}{uid} serialize seq end");
        dbg!(msg);
        // if self.
        // if(self.output)
        // dbg!()
        // self.seq_output.push();
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
        let uid = self.uid;
        let msg = format!("{SER_TAG}{uid} serialize_field key = {key}");
        dbg!(msg);
        match key {
            s if s.starts_with("@") => {
                let v = AttrValueSer::to_string(&value)?;
                let name = s[1..].to_string();
                self.output.set_attr(name, v);
            }
            "$value" => {
                // let mut  ser = XmlSer::new(self.output.name(), self.output.ns());
                // value.serialize(&mut ser)?;
                value.serialize(&mut **self)?;
                if uid == 12 {
                    dbg!(&self.output);
                    dbg!(&self.seq_output);
                }
                // self.
            }
            _ => {
                let mut ser = XmlSer::new(key, self.ns.clone());
                value.serialize(&mut ser)?;
                if !ser.is_none {
                    self.output.append_child(ser.output)?;
                } else {
                    let msg = format!("{SER_TAG}{uid} ser field {key} is empty!");
                    dbg!(msg);
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
        let root = self.temp_ele.take();
        if let Some(mut element) = root {
            // let ele = std::mem::replace(&mut self.output, element);
            // self.output.append_child(ele);
        }
        Ok(())
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

    #[test]
    fn ser_enum() -> Result<()> {
        #[derive(Serialize)]
        enum A {
            Va,
            Vb(u64),
            Vc { size: f32 },
        }

        // test unit variant
        let data = A::Va;
        let ser = XmlSer::new_with_prefix("test", "https://123.com", None);
        let e = ser.der_to_element(&data)?;
        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
        println!("{}", xml_str);

        // test newtype variant
        let data = A::Vb(32);
        let ser = XmlSer::new_with_prefix("test", "https://123.com", None);
        let e = ser.der_to_element(&data)?;
        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
        println!("{}", xml_str);

        let data = A::Vc { size: 1.0 };
        let ser = XmlSer::new_with_prefix("test", "https://123.com", None);
        let e = ser.der_to_element(&data)?;
        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
        println!("{}", xml_str);

        Ok(())
    }
}

pub struct XmlSeqSer {
    name: String,
    ns: String,
    output: Vec<Element>,
}
impl XmlSeqSer {
    fn new_empty() -> XmlSeqSer {
        XmlSeqSer {
            output: vec![],
            name: String::new(),
            ns: String::new(),
        }
    }
}
