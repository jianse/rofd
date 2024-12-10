mod attr;

use crate::ser::attr::AttrValueSer;
use minidom::{Element, IntoAttributeValue, Node};
use serde::ser::{Impossible, SerializeSeq, SerializeStruct, SerializeStructVariant};
use serde::{Serialize, Serializer};
use std::fmt::Display;
use std::sync::atomic::{AtomicUsize, Ordering};
use thiserror::Error;
use tracing::trace;

macro_rules! write_primitive {
    ($method:ident ( $ty:ty )) => {
        fn $method(self, value: $ty) -> Result<Self::Ok, Self::Error> {
            trace!("{} {} {}", SER_TAG, self.uid, stringify!($method));
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
    pub fn get_ele(self) -> Option<Element> {
        match self {
            Output::Ele(e) => Some(e),
            _ => None,
        }
    }
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
                    Output::Empty => {
                        // no op
                    }
                }
                Ok(())
            }
            _ => Err(XmlSerErr::Message("append_child".to_string())),
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
}

impl XmlSerBuilder {
    pub fn new() -> Self {
        XmlSerBuilder::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn ns(mut self, ns: impl Into<String>) -> Self {
        self.ns = ns.into();
        self
    }
    pub fn prefix(mut self, prefix: impl Into<Option<String>>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }
    pub fn create_element(mut self, create_element: bool) -> Self {
        self.create_element = create_element;
        self
    }

    pub fn build(self) -> Result<XmlSer, XmlSerErr> {
        let mut ser = XmlSer {
            name: self.name,
            ns: self.ns,
            prefix: self.prefix,
            uid: generate_id(),
            output: Output::Empty,
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
    temp_ele: Option<Element>,
}

impl XmlSer {
    pub fn builder() -> XmlSerBuilder {
        XmlSerBuilder::new()
    }
    fn with_ele(element: Element) -> Self {
        XmlSer {
            name: Some(element.name().to_string()),
            ns: element.ns(),
            prefix: None,
            uid: generate_id(),
            output: Output::Ele(element),
            temp_ele: None,
        }
    }

    pub fn ser_to_element<T>(mut self, value: &T) -> Result<Element, XmlSerErr>
    where
        T: Serialize,
    {
        value.serialize(&mut self)?;
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

    #[error("serialize not supported : {0}")]
    NotSupported(&'static str),
}
impl serde::ser::Error for XmlSerErr {
    fn custom<T: Display>(msg: T) -> Self {
        XmlSerErr::Message(msg.to_string())
    }
}

impl Serializer for &mut XmlSer {
    type Ok = ();
    type Error = XmlSerErr;
    type SerializeSeq = Self;
    type SerializeTuple = Impossible<(), XmlSerErr>;
    type SerializeTupleStruct = Impossible<(), XmlSerErr>;
    type SerializeTupleVariant = Impossible<(), XmlSerErr>;
    type SerializeMap = Impossible<(), XmlSerErr>;
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

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // HOW TO?
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        trace!("{} {} serialize_none", SER_TAG, self.uid);
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
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        trace!("{} {} serialize_unit_struct {}", SER_TAG, self.uid, name);
        let ele = self.create_ele(name)?;
        self.output = Output::Ele(ele);
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        trace!(
            "{SER_TAG} {} serialize_unit_variant {name}::{variant}",
            self.uid
        );

        let root = self.create_ele(variant)?;
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
        trace!("{} {} serialize_newtype_struct {}", SER_TAG, self.uid, name);
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        trace!(
            "{} {} serialize_newtype_variant {}::{}",
            SER_TAG,
            self.uid,
            name,
            variant
        );
        let mut ser = XmlSer::with_ele(self.create_ele(variant)?);
        value.serialize(&mut ser)?;
        self.output = ser.output;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        trace!("{} {} serialize_seq", SER_TAG, self.uid);

        self.output = Output::Vec(vec![]);
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(XmlSerErr::NotSupported("tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(XmlSerErr::NotSupported("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(XmlSerErr::NotSupported("tuple variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        trace!("{} {} serialize_map", SER_TAG, self.uid);
        Err(XmlSerErr::NotSupported("map"))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        trace!("{SER_TAG} {} serialize_struct struct = {name}", self.uid);

        if let Some(name) = self.name.clone() {
            self.output = Output::Ele(self.create_ele(name.as_str())?);
        } else {
            self.output = Output::Ele(self.create_ele(name)?);
        }
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        trace!(
            "{SER_TAG} {} serialize_struct_variant {name}:{variant}",
            self.uid
        );
        self.output = Output::Ele(self.create_ele(variant)?);
        self.output = Output::Ele(Element::bare(variant, self.ns.clone()));

        Ok(self)
    }
}

impl SerializeSeq for &mut XmlSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        trace!("{SER_TAG} {} SerializeSeq::serialize_element", self.uid);

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
        trace!("{SER_TAG} {} SerializeSeq::end", self.uid);
        if let Some(root) = self.temp_ele.take() {
            let output = self.output.take();
            self.output = Output::Ele(root);
            self.output.append_child(output)?;
        }
        Ok(())
    }
}

impl SerializeStruct for &mut XmlSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let uid = self.uid;
        trace!("{SER_TAG} {uid} SerializeStruct::serialize_field key = {key}");
        match key {
            s if s.starts_with("@") => {
                let v = AttrValueSer::convert_to_string(&value)?;
                let name = s[1..].to_string();
                self.output.set_attr(name, v)?;
            }
            "$text" => {
                let v = AttrValueSer::convert_to_string(&value)?;

                if let Some(v) = v {
                    if let Output::Ele(ele) = &mut self.output {
                        ele.append_node(Node::Text(v));
                    } else {
                        return Err(XmlSerErr::Message("$text on wrong element".to_string()));
                    }
                }
            }
            // $value mostly means an element append to parent,
            // and it's name is decided by sub serializer
            "$value" => {
                let mut ser = XmlSerBuilder::new().ns(&self.ns).build()?;
                value.serialize(&mut ser)?;
                self.output.append_child(ser.output)?;
            }
            _ => {
                let mut ser = XmlSerBuilder::new()
                    .name(key)
                    .ns(&self.ns)
                    .create_element(true)
                    .build()?;
                value.serialize(&mut ser)?;
                if let Output::Empty = ser.output {
                    trace!("{SER_TAG} {uid} ser field \"{key}\" is empty!");
                };
                self.output.append_child(ser.output)?;
            }
        }
        trace!("{} {} serialize_field end", SER_TAG, uid);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let uid = self.uid;
        trace!("{SER_TAG} {uid} SerializeStruct::end");
        Ok(())
    }
}

impl SerializeStructVariant for &mut XmlSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        trace!(
            "{} {} SerializeStructVariant::serialize_field key = {}",
            SER_TAG,
            self.uid,
            key
        );
        SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        trace!("{} {} SerializeStructVariant::end", SER_TAG, self.uid);
        let root = self.temp_ele.take();
        if let Some(element) = root {
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
    use crate::init_tracing_subscriber;

    use super::*;
    use eyre::Result;
    #[test]
    fn ser_works() -> Result<()> {
        init_tracing_subscriber();

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
        let e = ser.ser_to_element(&a)?;

        let ns = "https://123.com";

        // into string
        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
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
        assert_eq!(e, expected);
        Ok(())
    }

    #[derive(Serialize)]
    struct AnyStruct {
        #[serde(rename = "@Attr1")]
        attr1: String,
        #[serde(rename = "Ele1")]
        ele1: String,
    }

    #[derive(Serialize)]
    enum A {
        // test unit variant
        Va,
        // newtype variant with primitive type
        Vb(u64),
        // newtype variant with struct
        Vb1(AnyStruct),

        Vc {
            size: f32,
            #[serde(rename = "@Weight")]
            weight: f32,
        },
    }
    #[test]
    fn ser_enum() -> Result<()> {
        init_tracing_subscriber();

        // test unit variant
        let data = A::Va;
        // "test", "https://123.com", None
        let ser = XmlSerBuilder::new()
            .name("test")
            .ns("https://123.com")
            .prefix(None)
            .build()?;
        let e = ser.ser_to_element(&data)?;

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
        let ser = XmlSer::builder()
            .name("test")
            .ns("https://123.com")
            .prefix(None)
            .build()?;
        let e = ser.ser_to_element(&data)?;

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
        let ser = XmlSer::builder()
            .name("test")
            .ns("https://123.com")
            .prefix(None)
            .build()?;
        let e = ser.ser_to_element(&data)?;
        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
        println!("{}", xml_str);

        Ok(())
    }

    #[test]
    fn ser_newtype_var() -> Result<()> {
        init_tracing_subscriber();

        let ser = XmlSerBuilder::new()
            .name("test")
            .ns("https://123.com")
            .prefix(None)
            .build()?;
        let value = A::Vb1(AnyStruct {
            attr1: "foo".to_string(),
            ele1: "bar".to_string(),
        });

        let e = ser.ser_to_element(&value)?;
        let xml_str = to_string(&e)?;
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
        init_tracing_subscriber();

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
        let ser = XmlSer::builder()
            .name("test")
            .ns("https://123.com")
            .prefix(None)
            .build()?;
        let e = ser.ser_to_element(&data)?;
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
        init_tracing_subscriber();

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
        let ser = XmlSer::builder()
            .name("test")
            .ns("https://123.com")
            .prefix(None)
            .build()?;
        let e = ser.ser_to_element(&data)?;
        let mut buf = Vec::new();
        e.write_to_decl(&mut buf)?;
        let xml_str = String::from_utf8(buf)?;
        println!("{}", xml_str);
        Ok(())
    }
}
