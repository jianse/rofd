// #![feature(macro_metavar_expr_concat)]
#![allow(dead_code, unused)]
mod key;
mod value;

use crate::de::key::KeyDe;
use crate::de::value::{AttrValueDe, TextValueDe};
use minidom::element::{Attrs, Texts};
use minidom::{Children, Element};
use serde::de::{DeserializeSeed, EnumAccess, MapAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum XmlDeError {
    #[error("XmlDe error: {0}")]
    Message(String),

    #[error(transparent)]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    ParseBool(#[from] std::str::ParseBoolError),

    #[error("operation not supported.")]
    NotSupported,
}

impl serde::de::Error for XmlDeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Message(msg.to_string())
    }
}

macro_rules! de_primitives {
    ($func_name: ident  ($ty: ty, $f2:ident)) => {
        fn $func_name<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let v = self.input.text();
            let parsed = v.parse::<$ty>()?;
            visitor.$f2(parsed)
        }
    };
}

pub struct XmlDe<'de> {
    input: &'de Element,
}

impl<'de> XmlDe<'de> {
    pub fn from_ele(ele: &'de Element) -> Self {
        Self { input: ele }
    }
}

fn from_ele<'a, T>(ele: &'a Element) -> Result<T, XmlDeError>
where
    T: Deserialize<'a>,
{
    let mut de = XmlDe::from_ele(ele);
    T::deserialize(&mut de)
}

impl<'de, 'a> Deserializer<'de> for &'a mut XmlDe<'de> {
    type Error = XmlDeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    de_primitives!(deserialize_bool(bool, visit_bool));

    de_primitives!(deserialize_i8(i8, visit_i8));
    de_primitives!(deserialize_i16(i16, visit_i16));
    de_primitives!(deserialize_i32(i32, visit_i32));
    de_primitives!(deserialize_i64(i64, visit_i64));

    de_primitives!(deserialize_u8(u8, visit_u8));
    de_primitives!(deserialize_u16(u16, visit_u16));
    de_primitives!(deserialize_u32(u32, visit_u32));
    de_primitives!(deserialize_u64(u64, visit_u64));

    de_primitives!(deserialize_f32(f32, visit_f32));
    de_primitives!(deserialize_f64(f64, visit_f64));

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.input.text();
        if text.chars().count() == 1 {
            visitor.visit_char(text.chars().next().unwrap())
        } else {
            Err(XmlDeError::Message("not a char".into()))
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input.text().as_str())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.input.text())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(XmlDeError::NotSupported)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(XmlDeError::NotSupported)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // The name is struct name.
        // if you want to assert that element name is same as field name, you should pass it in.

        // let x = self.input.name();
        // assert_eq!(x, name);
        visitor.visit_map(AttrChild::new(self))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let enum_access = Enum::new(self);
        visitor.visit_enum(enum_access)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

enum Ctx<'de> {
    Empty,
    Attr(&'de str),
    Ele(&'de Element),
    Text(&'de Element),
}

struct AttrChild<'a, 'de: 'a> {
    de: &'a XmlDe<'de>,
    attrs: Attrs<'de>,
    children: Children<'de>,
    texts: Texts<'de>,
    current_value: Ctx<'de>,
    text_visited: bool,
}
impl<'a, 'de> AttrChild<'a, 'de> {
    fn new(de: &'a XmlDe<'de>) -> Self {
        let attrs = de.input.attrs();
        let children = de.input.children();
        let texts = de.input.texts();
        Self {
            de,
            attrs,
            children,
            texts,
            current_value: Ctx::Empty,
            text_visited: false,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for AttrChild<'a, 'de> {
    type Error = XmlDeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if !self.text_visited {
            self.text_visited = true;
            let text = self.de.input.text();

            self.current_value = Ctx::Text(self.de.input);
            let mut de = KeyDe::new_text();
            let result = seed.deserialize(&mut de);
            return result.map(Some);
        }

        // attrs
        let option = self.attrs.next();
        if let Some((key, value)) = option {
            self.current_value = Ctx::Attr(value);
            let mut de = KeyDe::new_attr(key);
            let result = seed.deserialize(&mut de);
            return result.map(Some);
        }
        // children
        let child = self.children.next();
        if let Some(child) = child {
            self.current_value = Ctx::Ele(child);
            let mut de = KeyDe::new_ele(child.name());
            let result = seed.deserialize(&mut de);
            return result.map(Some);
        }
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.current_value {
            Ctx::Empty => Err(XmlDeError::Message(
                "you must call next_key_seed first!".into(),
            )),
            Ctx::Attr(s) => {
                let mut de = AttrValueDe::new(s);
                seed.deserialize(&mut de)
            }
            Ctx::Ele(e) => {
                let mut de = XmlDe::from_ele(e);
                seed.deserialize(&mut de)
            }
            Ctx::Text(txt) => {
                let mut de = TextValueDe::new(txt.text());
                seed.deserialize(&mut de)
            }
        }
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a XmlDe<'de>,
}
impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a XmlDe<'de>) -> Self {
        Self { de }
    }
}

impl<'a, 'de> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = XmlDeError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let mut de = KeyDe::new_ele(self.de.input.name());
        let result = seed.deserialize(&mut de)?;
        Ok((result, self))
    }
}

impl<'a, 'de> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = XmlDeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        let mut de = XmlDe::from_ele(self.de.input);
        seed.deserialize(&mut de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::de::from_ele;
    use eyre::Result;
    use minidom::Element;
    use serde::Deserialize;
    #[derive(Debug, Deserialize)]
    struct Foo {
        #[serde(rename = "@attr1")]
        attr1: Option<String>,
        #[serde(rename = "@attr2")]
        attr2: f32,
    }
    #[test]
    fn it_works() -> Result<()> {
        let mut ele = Element::bare("Foo", "");
        ele.set_attr("attr2", "1.23");
        let foo = from_ele::<Foo>(&ele)?;
        print!("{:#?}", foo);
        Ok(())
    }

    #[test]
    fn deserialize_attr_field() -> Result<()> {
        let mut ele = Element::bare("Foo", "");
        ele.set_attr("attr1", "bar");
        ele.set_attr("attr2", "6.25");
        let foo = from_ele::<Foo>(&ele)?;
        assert_eq!(foo.attr1, Some("bar".to_string()));
        assert_eq!(foo.attr2, 6.25);
        print!("{:#?}", foo);
        Ok(())
    }

    #[test]
    fn deserialize_ele_child() -> Result<()> {
        #[derive(Debug, Deserialize)]
        struct FooB {
            ele: Foo,
            ele1: String,
            ele2: f32,
        }

        let mut root = Element::bare("FooB", "");
        let ele = Element::builder("ele", "").attr("attr2", "0").build();
        root.append_child(ele);

        let ele1 = Element::builder("ele1", "").append("hello").build();
        root.append_child(ele1);

        let ele2 = Element::builder("ele2", "").append("10.0").build();

        root.append_child(ele2);

        let st = from_ele::<FooB>(&root)?;
        assert_eq!(st.ele1, "hello".to_string());
        assert_eq!(st.ele2, 10.0);
        print!("{:#?}", st);
        Ok(())
    }

    #[test]
    fn test_enum() -> Result<()> {
        #[derive(Debug, Deserialize)]
        struct MyStruct {
            #[serde(rename = "@attr")]
            attr: String,
            #[serde(rename = "$text")]
            text: String,
        }

        #[derive(Debug, Deserialize)]
        enum MyEnum {
            Variant1,
            Variant2(usize),
            Variant3(MyStruct),
        }
        // unit variant
        let root = Element::builder("Variant1", "").build();
        let st = from_ele::<MyEnum>(&root)?;
        println!("{:#?}", st);

        // newtype variant with primitive types
        let root = Element::builder("Variant2", "").append("123456").build();
        let st = from_ele::<MyEnum>(&root)?;
        println!("{:#?}", st);

        // newtype variant with struct
        let root = Element::builder("Variant3", "")
            .attr("attr", "hello")
            .append("123456")
            .build();
        let st = from_ele::<MyEnum>(&root)?;
        println!("{:#?}", st);

        Ok(())
    }
}
