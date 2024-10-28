use crate::ser::attr::AttrValueSer;
use crate::ser::{generate_id, Output, XmlSer, XmlSerErr};
use minidom::{Element, Node};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};

const SER_TAG: &str = "field_ser";

macro_rules! write_primitive {
    ($method:ident ( $ty:ty )) => {
        fn $method(self, v: $ty) -> Result<Self::Ok, Self::Error> {
            let msg = format!("{} {} {}", SER_TAG, self.uid, stringify!($method));
            dbg!(msg);
            if let Some(name) = &self.name {
                let ele = Element::builder(name, &self.ns)
                    .append(v.to_string())
                    .build();
                self.output = Output::Ele(ele);
                Ok(())
            } else {
                Err(XmlSerErr::Message("no name provided".into()))
            }
        }
    };
}

// 生成的元素的名称由哪里指定，默认外部指定
enum NamedBy {
    // 外部指定，默认
    External,
    // 内部指定
    Internal,
}

// 当前所处的上下文
enum Ctx {
    // 普通上下文，默认
    Normal,
    // 列表上下人
    Seq,
}

pub(crate) struct FieldSer {
    name: Option<String>,
    ns: String,
    output: Output,
    named_by: NamedBy,
    ctx: Ctx,
    uid: usize,
}

impl FieldSer {
    // fn name()->Option<String> {
    //
    // }
}

impl<'s> Serializer for &'s mut FieldSer {
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
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        // DO NOTHING
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
        // let name = match self.named_by {
        //     NamedBy::External => {
        //         self.name.as_deref().unwrap_or(name)
        //     }
        //     NamedBy::Internal => {
        //         name
        //     }
        // };
        self.output = Output::Ele(Element::bare(name, &self.ns));
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.named_by = NamedBy::Internal;
        self.output = Output::Ele(Element::bare(variant, &self.ns));
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
        // maybe we should honor the name of struct
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // self.named_by = NamedBy::Internal;
        let mut ser = FieldSer {
            name: Some(variant.to_string()),
            ns: self.ns.clone(),
            output: Output::Empty,
            named_by: NamedBy::External,
            ctx: Ctx::Normal,
            uid: generate_id(),
        };
        value.serialize(&mut ser)?;
        self.output = ser.output;
        // let out = AttrValueSer::to_string(&value)?;
        // if let Some(v) = out {
        //     let ele = Element::builder(variant, &self.ns).append(v).build();
        //     self.output = Output::Ele(ele);
        // } else {
        //     self.output = Output::Empty
        // }

        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.ctx = Ctx::Seq;
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
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let name = match self.named_by {
            NamedBy::External => self.name.as_ref().unwrap(),
            NamedBy::Internal => name,
        };
        let ele = Element::bare(name, &self.ns);
        self.output = Output::Ele(ele);
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.output = Output::Ele(Element::bare(variant, &self.ns));
        Ok(self)
    }
}

impl<'s> SerializeSeq for &'s mut FieldSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut ser = FieldSer {
            name: self.name.clone(),
            ns: self.ns.clone(),
            output: Output::Empty,
            named_by: NamedBy::External,
            ctx: Ctx::Normal,
            uid: generate_id(),
        };
        value.serialize(&mut ser);
        self.output.push(ser.output);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ctx = Ctx::Normal;
        Ok(())
    }
}

impl<'s> SerializeTuple for &'s mut FieldSer {
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

impl<'s> SerializeTupleStruct for &'s mut FieldSer {
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

impl<'s> SerializeTupleVariant for &'s mut FieldSer {
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

impl<'s> SerializeMap for &'s mut FieldSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
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

impl<'s> SerializeStruct for &'s mut FieldSer {
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
                let v = AttrValueSer::convert_to_string(&value)?;
                // dbg!(&self.output);
                // dbg!(&v);
                let name = s[1..].to_string();
                // dbg!(&name);
                self.output.set_attr(name, v)?;
                // dbg!(&self.output);
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

                // self.output.append_node("")
                // match self.output {  }
            }
            "$value" => {
                todo!()
            }
            _ => {
                let mut ser = FieldSer {
                    name: Some(key.to_string()),
                    ns: self.ns.clone(),
                    output: Output::Empty,
                    named_by: NamedBy::External,
                    ctx: Ctx::Normal,
                    uid: generate_id(),
                };
                value.serialize(&mut ser);
                self.output.append_child(ser.output);
            }
        }
        let msg = "serialize_field end".to_string();
        dbg!(msg);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'s> SerializeStructVariant for &'s mut FieldSer {
    type Ok = ();
    type Error = XmlSerErr;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let uid = self.uid;
        let msg = format!("{SER_TAG} {uid} SerializeStructVariant::serialize_field key = {key}");
        dbg!(msg);
        match key {
            s if s.starts_with("@") => {
                let v = AttrValueSer::convert_to_string(&value)?;
                // dbg!(&self.output);
                // dbg!(&v);
                let name = s[1..].to_string();
                // dbg!(&name);
                self.output.set_attr(name, v)?;
                // dbg!(&self.output);
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

                // self.output.append_node("")
                // match self.output {  }
            }
            "$value" => {
                todo!()
            }
            _ => {
                let mut ser = FieldSer {
                    name: Some(key.to_string()),
                    ns: self.ns.clone(),
                    output: Output::Empty,
                    named_by: NamedBy::External,
                    ctx: Ctx::Normal,
                    uid: generate_id(),
                };
                value.serialize(&mut ser);
                self.output.append_child(ser.output);
            }
        }
        let msg = "serialize_field end".to_string();
        dbg!(msg);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ser::to_string;
    use eyre::Result;

    #[test]
    fn test_primitive() -> Result<()> {
        let mut ser = FieldSer {
            name: Some("Test".to_string()),
            ns: "".to_string(),
            output: Output::Empty,
            named_by: NamedBy::External,
            ctx: Ctx::Normal,
            uid: generate_id(),
        };

        let value = 10;

        value.serialize(&mut ser)?;

        dbg!(ser.output);
        Ok(())
    }
    #[test]
    fn test_vec_primitive() -> Result<()> {
        let mut ser = FieldSer {
            name: Some("Test".to_string()),
            ns: "".to_string(),
            output: Output::Empty,
            named_by: NamedBy::External,
            ctx: Ctx::Normal,
            uid: generate_id(),
        };

        let value = vec![1, 2, 3];

        value.serialize(&mut ser)?;
        // let xml_str = to_string()
        dbg!(ser.output);
        Ok(())
    }

    #[test]
    fn test_struct() -> Result<()> {
        #[derive(Serialize)]
        struct AltName {
            #[serde(rename = "@A1")]
            attr1: String,
            field1: String,
            field2: String,
        }

        let mut ser = FieldSer {
            name: Some("Test".to_string()),
            ns: "".to_string(),
            output: Output::Empty,
            named_by: NamedBy::External,
            ctx: Ctx::Normal,
            uid: generate_id(),
        };

        // a raw field without rename
        let value = AltName {
            attr1: "attr1-value".to_string(),
            field1: "foo".to_string(),
            field2: "bar".to_string(),
        };
        value.serialize(&mut ser)?;
        // let res = ser.
        assert!(matches!(&ser.output, Output::Ele(_)));

        let e = match ser.output {
            Output::Ele(e) => e,
            _ => todo!(),
        };
        let s = to_string(&e)?;
        println!("{}", s);
        // ser.output.
        // dbg!(&ser.output);

        Ok(())
    }

    #[test]
    fn test_newtype_struct() -> Result<()> {
        #[derive(Serialize)]
        struct AltName(String);

        let mut ser = create_ser();
        let value = AltName("foo".to_string());
        value.serialize(&mut ser)?;
        assert!(matches!(&ser.output, Output::Ele(_)));
        let e = ser.output.get_ele().unwrap();
        let s = to_string(&e)?;
        println!("{}", s);
        Ok(())
    }

    #[test]
    fn test_enum() -> Result<()> {
        #[derive(Serialize)]
        struct SubStruct {
            value: f32,
        }
        #[derive(Serialize)]
        enum MyEnum {
            Variant1,
            Variant2(f32),
            VariantImpossible(f32, f32),
            Variant3(SubStruct),
            Variant4 { value: f32 },
        }

        // unit variant
        let mut ser = create_ser();
        let value = MyEnum::Variant1;
        value.serialize(&mut ser)?;
        let ele = ser.output.get_ele().unwrap();
        let s = to_string(&ele)?;
        println!("{}", s);

        // newtype variant
        let mut ser = create_ser();
        let value = MyEnum::Variant2(10.0);
        value.serialize(&mut ser)?;
        let ele = ser.output.get_ele().unwrap();
        let s = to_string(&ele)?;
        println!("{}", s);

        // newtype variant
        let mut ser = create_ser();
        let value = MyEnum::Variant3(SubStruct { value: 0.0 });
        value.serialize(&mut ser)?;
        let ele = ser.output.get_ele().unwrap();
        let s = to_string(&ele)?;
        println!("{}", s);

        // struct_variant
        let mut ser = create_ser();
        let value = MyEnum::Variant4 { value: 0.0 };
        value.serialize(&mut ser)?;
        let ele = ser.output.get_ele().unwrap();
        let s = to_string(&ele)?;
        println!("{}", s);

        Ok(())
    }

    #[test]
    fn test_unit_struct() -> Result<()> {
        #[derive(Serialize)]
        struct UnitStruct;

        let mut ser = create_ser();
        let value = UnitStruct;
        value.serialize(&mut ser)?;
        let ele = ser.output.get_ele().unwrap();
        let s = to_string(&ele)?;
        println!("{}", s);

        Ok(())
    }
    #[test]
    fn test_vec_enum() -> Result<()> {
        #[derive(Serialize)]
        struct SubStruct {
            value: f32,
        }
        #[derive(Serialize)]
        enum MyEnum {
            Variant1,
            Variant2(f32),
            VariantImpossible(f32, f32),
            Variant3(SubStruct),
            Variant4 { value: f32 },
        }

        let mut ser = create_ser();
        let value = vec![
            MyEnum::Variant1,
            MyEnum::Variant2(32.0),
            MyEnum::Variant4 { value: 10.0 },
        ];
        value.serialize(&mut ser)?;
        dbg!(ser.output);
        Ok(())
    }

    fn create_ser() -> FieldSer {
        // ser is created by serialize a filed so we always have a name and named by external.
        FieldSer {
            name: Some("Test".to_string()),
            ns: "".to_string(),
            output: Output::Empty,
            named_by: NamedBy::External,
            ctx: Ctx::Normal,
            uid: generate_id(),
        }
    }
}
