#![allow(unused)]
// #![feature(error_generic_member_access)]

mod attr;
mod field;

use crate::ser::attr::AttrValueSer;
use minidom::{Element, IntoAttributeValue, Node};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};
use std::backtrace::Backtrace;
use std::fmt::{format, Display};
use std::sync::atomic::{AtomicUsize, Ordering};
use thiserror::Error;

macro_rules! write_primitive {
    ($method:ident ( $ty:ty )) => {
        fn $method(self, value: $ty) -> Result<Self::Ok, Self::Error> {
            let msg = format!("{} {} {}", SER_TAG, self.uid, stringify!($method));
            dbg!(msg);
            self.output.append_child_node(value.to_string())?;
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
        // dbg!(&self);
        // dbg!(&out);
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
                    Output::Empty => {
                        // no op
                    }
                }
                Ok(())
            }
            _ => Err(XmlSerErr::Message("append_child".to_string())),
        }
    }
    fn append_child_ele(&mut self, ele: Element) -> Result<(), XmlSerErr> {
        match self {
            Output::Ele(e) => {
                e.append_child(ele);
                Ok(())
            }
            _ => Err(XmlSerErr::Message("append_child_ele".to_string())),
        }
    }

    fn append_child_node<N>(&mut self, node: N) -> Result<(), XmlSerErr>
    where
        N: Into<Node>,
    {
        match self {
            Output::Ele(e) => {
                e.append_node(node.into());
                Ok(())
            }
            _ => Err(XmlSerErr::Message("append_child_node".to_string())),
        }
    }
    fn set_attr<S: Into<String>, V: IntoAttributeValue>(
        &mut self,
        key: S,
        value: V,
    ) -> Result<(), XmlSerErr> {
        // dbg!(&self);
        if let Output::Ele(e) = self {
            e.set_attr(key, value);
            Ok(())
        } else {
            Err(XmlSerErr::Message("set_attr".to_string()))
        }
    }

    fn take(&mut self) -> Output {
        std::mem::replace(self, Output::Empty)
    }
    fn push(&mut self, value: Output) -> Result<(), XmlSerErr> {
        match self {
            Output::Vec(v) => {
                match value {
                    Output::Ele(e) => {
                        v.push(e);
                    }
                    Output::Vec(mut ov) => {
                        v.append(&mut ov);
                    }
                    Output::Empty => {
                        // no op
                    }
                };
                Ok(())
            }
            _ => Err(XmlSerErr::Message("push output error".to_string())),
        }
    }
}

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_ID: usize = usize::MAX / 2;

pub(crate) fn generate_id() -> usize {
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

const SER_TAG: &str = "[xml_ser]";

#[derive(Debug, Default)]
pub struct XmlSerBuilder {
    name: Option<String>,
    ns: String,
    prefix: Option<Option<String>>,
    create_element: bool,
    ele: Option<Element>,
}

impl XmlSerBuilder {
    fn new() -> Self {
        XmlSerBuilder::default()
    }

    fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    fn ns(mut self, ns: impl Into<String>) -> Self {
        self.ns = ns.into();
        self
    }
    fn prefix(mut self, prefix: impl Into<Option<String>>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }
    fn create_element(mut self, create_element: bool) -> Self {
        self.create_element = create_element;
        self
    }

    fn build(self) -> Result<XmlSer, XmlSerErr> {
        let mut ser = XmlSer {
            name: self.name,
            ns: self.ns,
            prefix: self.prefix,
            uid: generate_id(),
            output: Output::Empty,
            seq_output: vec![],
            temp_ele: None,
        };
        if self.create_element {
            let ele = ser.create_ele_default()?;
            ser.output = Output::Ele(ele);
        }

        Ok(ser)
    }
}

pub struct XmlSer {
    // replace element name if it has value
    // or use struct name when this is `None`
    name: Option<String>,
    // namespace always inherit
    ns: String,
    // prefix for namespace
    prefix: Option<Option<String>>,
    // serializer uid
    uid: usize,
    // serialized content
    output: Output,
    seq_output: Vec<Element>,
    temp_ele: Option<Element>,
}

impl XmlSer {
    fn with_ele(element: Element) -> Self {
        XmlSer {
            name: Some(element.name().to_string()),
            ns: element.ns(),
            prefix: None,
            uid: generate_id(),
            output: Output::Ele(element),
            seq_output: vec![],
            temp_ele: None,
        }
    }
    pub fn new_with_prefix<N, NS>(name: N, namespace: NS, prefix: Option<String>) -> XmlSer
    where
        N: AsRef<str>,
        NS: Into<String>,
    {
        let ns = namespace.into();
        let name = name.as_ref();

        XmlSer {
            name: Some(String::from(name)),
            ns,
            prefix: Some(prefix.clone()),
            uid: generate_id(),
            output: Output::Empty,
            seq_output: vec![],
            temp_ele: None,
        }
    }
    pub fn new<N, NS>(namespace: NS) -> XmlSer
    where
        NS: Into<String>,
    {
        let ns = namespace.into();
        XmlSer {
            name: None,
            ns,
            prefix: None,
            uid: generate_id(),
            output: Output::Empty,
            seq_output: vec![],
            temp_ele: None,
        }
    }
    pub fn new_with_name<N, NS>(name: N, namespace: NS) -> XmlSer
    where
        N: AsRef<str>,
        NS: Into<String>,
    {
        let name = name.as_ref();
        let ns = namespace.into();
        XmlSer {
            name: Some(String::from(name)),
            ns,
            prefix: None,
            uid: generate_id(),
            output: Output::Empty,
            seq_output: vec![],
            temp_ele: None,
        }
    }

    pub fn der_to_element<T>(mut self, value: &T) -> Result<Element, XmlSerErr>
    where
        T: Serialize,
    {
        value.serialize(&mut self)?;
        dbg!(&self.output);
        match self.output {
            Output::Ele(e) => Ok(e),
            _ => Err(XmlSerErr::Message("result is not an element".to_string())),
        }
    }
    fn create_ele(&self, name: &str) -> Result<Element, XmlSerErr> {
        if let Some(prefix) = &self.prefix {
            Ok(Element::builder(name, &self.ns)
                .prefix(prefix.clone(), &self.ns)?
                .build())
        } else {
            Ok(Element::builder(name, &self.ns).build())
        }
    }

    fn create_ele_default(&self) -> Result<Element, XmlSerErr> {
        if let Some(name) = self.name.as_ref() {
            self.create_ele(name)
        } else {
            Err(XmlSerErr::Message(
                "create_element error: missing name".to_string(),
            ))
        }
    }
}

#[derive(Error, Debug)]
pub enum XmlSerErr {
    #[error("xml serialization error")]
    Common,
    #[error("xml serialization error: {0}")]
    Message(String),

    #[error(transparent)]
    CreateElementError(#[from] minidom::Error),
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

    write_primitive!(serialize_bool(bool));

    write_primitive!(serialize_i8(i8));
    write_primitive!(serialize_i16(i16));
    write_primitive!(serialize_i32(i32));
    write_primitive!(serialize_i64(i64));

    write_primitive!(serialize_u8(u8));
    write_primitive!(serialize_u16(u16));
    write_primitive!(serialize_u32(u32));
    write_primitive!(serialize_u64(u64));

    write_primitive!(serialize_f32(f32));
    write_primitive!(serialize_f64(f64));

    write_primitive!(serialize_char(char));
    write_primitive!(serialize_str(&str));

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        let msg = format!("{} {} serialize_none", SER_TAG, self.uid);
        dbg!(msg);
        self.output = Output::Empty;
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
        let msg = format!("{SER_TAG} {uid} serialize_unit_variant {name}::{variant}");
        dbg!(msg);

        let mut root = self.create_ele(variant)?;
        // root.append_child(Element::bare(variant, self.ns.clone()));
        self.output = Output::Ele(root);
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
        let msg = format!("{SER_TAG} {uid} serialize_newtype_variant {_name}:{variant}");
        dbg!(msg);
        let mut ser = XmlSer::with_ele(self.create_ele(variant)?);
        value.serialize(&mut ser)?;
        self.output = ser.output;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let uid = self.uid;
        let msg = format!("{SER_TAG} {uid} serialize_seq");
        dbg!(msg);
        // match self.output.take() {
        //     Output::Ele(e) => {
        //         self.temp_ele = Some(e);
        //     }
        //     Output::Vec(_) => {
        //         return Err(XmlSerErr::Message("seq of seq".to_string()));
        //     }
        //     Output::Empty => {}
        // }
        self.output = Output::Vec(vec![]);

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
        let msg = format!("{} {} serialize_map", SER_TAG, self.uid);
        dbg!(msg);

        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let uid = self.uid;
        let msg = format!("{SER_TAG} {uid} serialize_struct struct = {name}",);
        dbg!(msg);
        // let name = &mut self.name.as_ref().map(|e|e.as_str()).unwrap_or(name);
        if let Some(name) = self.name.clone() {
            self.output = Output::Ele(self.create_ele(name.as_str())?);
        } else {
            self.output = Output::Ele(self.create_ele(name)?);
        }
        // self.serialize_map(Some(len))
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let uid = self.uid;
        let msg = format!("{SER_TAG} {uid} serialize_struct_variant {name}:{variant}");
        dbg!(msg);
        self.output = Output::Ele(self.create_ele(variant)?);
        self.output = Output::Ele(Element::bare(variant, self.ns.clone()));

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
        let msg = format!("{SER_TAG} {uid} SerializeSeq::serialize_element");
        dbg!(msg);
        let mut ser = if let Some(name) = self.name.as_ref() {
            XmlSerBuilder::new()
                .name(name)
                .ns(self.ns.clone())
                .create_element(true)
                .build()?
        } else {
            XmlSerBuilder::new().ns(self.ns.clone()).build()?
        };

        value.serialize(&mut ser)?;
        self.output.push(ser.output)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let uid = self.uid;
        let msg = format!("{SER_TAG} {uid} SerializeSeq::end");
        dbg!(msg);
        if let Some(root) = self.temp_ele.take() {
            let output = self.output.take();
            self.output = Output::Ele(root);
            self.output.append_child(output)?;
        }
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
        let msg = format!("{SER_TAG} {uid} SerializeStruct::serialize_field key = {key}");
        dbg!(msg);
        match key {
            s if s.starts_with("@") => {
                let v = AttrValueSer::to_string(&value)?;
                // dbg!(&self.output);
                // dbg!(&v);
                let name = s[1..].to_string();
                // dbg!(&name);
                self.output.set_attr(name, v)?;
                // dbg!(&self.output);
            }
            "$text" => {
                let v = AttrValueSer::to_string(&value)?;

                if let Some(v) = v {
                    if let Output::Ele(ele) = &mut self.output {
                        ele.append_node(Node::Text(v));
                    } else {
                        return Err(XmlSerErr::Message("$text on wrong element".to_string()));
                    }
                }

                // self.output.append_node("")
                // match self.output {  }
            }
            "$value" => {
                let name = self.name.take();
                value.serialize(&mut **self)?;
                self.name = name;
            }
            _ => {
                let mut ser = XmlSer::with_ele(Element::bare(key, &self.ns));
                value.serialize(&mut ser)?;
                if let Output::Empty = ser.output {
                    let msg = format!("{SER_TAG}{uid} ser field {key} is empty!");
                    dbg!(msg);
                };
                self.output.append_child(ser.output)?;
            }
        }
        let msg = "serialize_field end".to_string();
        dbg!(msg);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let uid = self.uid;
        let msg = format!("{SER_TAG} {uid} SerializeStruct::end");
        dbg!(msg);
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
        let msg = format!(
            "{} {} SerializeStructVariant::serialize_field key = {}",
            SER_TAG, self.uid, key
        );
        dbg!(msg);
        SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let msg = format!("{} {} SerializeStructVariant::end", SER_TAG, self.uid);
        dbg!(msg);
        let root = self.temp_ele.take();
        if let Some(mut element) = root {
            let mut output = Output::Ele(element);
            let sub = self.output.take();
            output.append_child(sub)?;
            self.output = output;
        }
        Ok(())
    }
}

#[cfg(test)]
pub(crate) fn to_string(element: &Element) -> eyre::Result<String> {
    let mut buf = Vec::new();
    element.write_to_decl(&mut buf)?;
    let xml_str = String::from_utf8(buf)?;
    Ok(xml_str)
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

        let ser = XmlSerBuilder::new()
            .name("test")
            .ns("https://123.com")
            .prefix(None)
            .build()?;
        let e = ser.der_to_element(&a)?;

        let ns = "https://123.com";

        // into string
        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
        // dbg!(&xml_str);
        println!("{}", xml_str);
        let expected = Element::builder("test", ns)
            .prefix(None, ns)?
            .attr("Attr1", "foo")
            .attr("Attr2", "baz")
            .attr("VecAttr", "foo bar")
            .append(Node::Element(
                Element::builder("Ele1", ns)
                    .append(Node::Text("bar".to_string()))
                    .build(),
            ))
            .append(Node::Element(
                Element::builder("VecEle", ns)
                    .append(Node::Text("1".to_string()))
                    .build(),
            ))
            .append(Node::Element(
                Element::builder("VecEle", ns)
                    .append(Node::Text("2".to_string()))
                    .build(),
            ))
            .append(Node::Element(
                Element::builder("idx", ns)
                    .append(Node::Text("1".to_string()))
                    .build(),
            ))
            .build();
        // assert_eq!(e, expected);
        Ok(())
    }
    #[derive(Serialize)]
    enum A {
        // test unit variant
        Va,
        // newtype variant with primitive type
        Vb(u64),
        // newtype variant with struct
        Vb1(),

        Vc {
            size: f32,
            #[serde(rename = "@Weight")]
            weight: f32,
        },
    }
    #[test]
    fn ser_enum() -> Result<()> {
        // test unit variant
        let data = A::Va;
        // "test", "https://123.com", None
        let ser = XmlSerBuilder::new()
            .name("test")
            .ns("https://123.com")
            .prefix(None)
            .build()?;
        let e = ser.der_to_element(&data)?;

        assert_eq!(
            e,
            Element::builder("Va", "https://123.com")
                .prefix(None, "https://123.com")?
                .build()
        );
        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
        println!("{}", xml_str);

        // test newtype variant
        let data = A::Vb(32);
        let ser = XmlSer::new_with_prefix("test", "https://123.com", None);
        let e = ser.der_to_element(&data)?;

        assert_eq!(
            e,
            Element::builder("Vb", "https://123.com")
                .append(Node::Text("32".to_string()))
                .prefix(None, "https://123.com")?
                .build()
        );

        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
        println!("{}", xml_str);

        let data = A::Vc {
            size: 1.0,
            weight: 20.0,
        };
        let ser = XmlSer::new_with_prefix("test", "https://123.com", None);
        let e = ser.der_to_element(&data)?;
        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
        println!("{}", xml_str);

        Ok(())
    }

    #[derive(Serialize)]
    #[serde(rename = "AAA")]
    struct SA {
        enums: Vec<A>,
    }
    #[test]
    fn test_vec_enum() -> Result<()> {
        let data = SA {
            enums: vec![
                A::Va,
                A::Va,
                A::Vb(12),
                A::Vc {
                    size: 12.0,
                    weight: 24.0,
                },
            ],
        };
        let ser = XmlSer::new_with_prefix("test", "https://123.com", None);
        let e = ser.der_to_element(&data)?;
        // dbg!(e);
        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
        println!("{}", xml_str);
        Ok(())
    }

    #[derive(Serialize)]
    #[serde(rename = "AAA")]
    struct SB {
        #[serde(rename = "$value")]
        enums: Vec<A>,
    }
    #[test]
    fn test_value_vec_enum() -> Result<()> {
        let data = SB {
            enums: vec![
                A::Va,
                A::Va,
                A::Vb(12),
                A::Vc {
                    size: 12.0,
                    weight: 24.0,
                },
            ],
        };
        let ser = XmlSer::new_with_prefix("test", "https://123.com", None);
        let e = ser.der_to_element(&data)?;
        // dbg!(e);
        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
        println!("{}", xml_str);
        Ok(())
    }
}
