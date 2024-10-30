#![allow(dead_code, unused)]
mod key;
mod value;

use crate::de::key::KeyDe;
use crate::de::value::AttrValueDe;
use minidom::element::Attrs;
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
}

impl serde::de::Error for XmlDeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Message(msg.to_string())
    }
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

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v = self.input.text();
        let num = v.parse::<f64>()?;
        visitor.visit_f64(num)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
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
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
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
}

struct AttrChild<'a, 'de: 'a> {
    de: &'a XmlDe<'de>,
    attrs: Attrs<'de>,
    children: Children<'de>,
    current_value: Ctx<'de>,
}
impl<'a, 'de> AttrChild<'a, 'de> {
    fn new(de: &'a XmlDe<'de>) -> Self {
        let attrs = de.input.attrs();
        let children = de.input.children();
        Self {
            de,
            attrs,
            children,
            current_value: Ctx::Empty,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for AttrChild<'a, 'de> {
    type Error = XmlDeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
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
        todo!()
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
        enum MyEnum {
            Variant1,
        }
        let root = Element::builder("Variant1", "").build();
        let st = from_ele::<MyEnum>(&root)?;
        print!("{:#?}", st);
        Ok(())
    }
}
