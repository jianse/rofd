use crate::error::Error;
use std::fmt::{Debug, Formatter};
use std::io::{Cursor, Read, Seek};

#[derive(Debug)]
pub struct Tlv {
    pub tag_class: TagClass,
    pub tag: Tag,
    pub length: usize,
    pub value: Value,
}
impl Tlv {
    pub fn item_count(&self) -> usize {
        match &self.value {
            Value::Primitive(_) => 1,
            Value::Nested(v) => v.len(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TagClass {
    Universal,
    Application,
    ContextSpecific,
    Private,
}

#[derive(Debug, PartialEq)]
pub enum Tag {
    Integer,
    BitString,
    OctetString,
    Null,
    ObjectIdentifier,
    UTF8String,
    Sequence,
    Set,
    PrintableString,
    IA5String,
    UTCTime,
    GeneralizedTime,

    Other(u8),
}
pub enum Value {
    Primitive(Vec<u8>),
    Nested(Vec<Tlv>),
}
impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Primitive(v) => {
                if v.len() > 5 {
                    f.write_fmt(format_args!("Primitive({:#x?} ...)", v[0..5].as_ref()))
                } else {
                    f.write_fmt(format_args!("Primitive({:#x?})", v))
                }
            }
            Value::Nested(v) => f.debug_list().entries(v.iter()).finish(),
        }
    }
}

#[derive(Debug)]
pub enum DerFragment {
    Empty,
    Single(Tlv),
    Multiple(Vec<Tlv>),
}

impl DerFragment {
    fn append(&mut self, tlv: Tlv) {
        *self = match std::mem::replace(self, Self::Empty) {
            DerFragment::Empty => DerFragment::Single(tlv),
            DerFragment::Single(org) => DerFragment::Multiple(vec![org, tlv]),
            DerFragment::Multiple(mut vec) => {
                vec.push(tlv);
                DerFragment::Multiple(vec)
            }
        }
    }
}

impl TryInto<Value> for DerFragment {
    type Error = Error;

    fn try_into(self) -> Result<Value, Self::Error> {
        match self {
            DerFragment::Empty => Err(Error::ConvertError),
            DerFragment::Single(tlv) => Ok(Value::Nested(vec![tlv])),
            DerFragment::Multiple(tlvs) => Ok(Value::Nested(tlvs)),
        }
    }
}

pub fn parse_single<D: Read + Seek>(data: &mut D) -> Result<Tlv, Error> {
    let res = parse_recursive(data)?;
    match res {
        DerFragment::Single(v) => Ok(v),
        _ => Err(Error::ConvertError),
    }
}

fn parse_recursive<D: Read + Seek>(data: &mut D) -> Result<DerFragment, Error> {
    let pos = 0_u64;
    parse_recursive_inner(data, pos)
}

fn parse_recursive_inner<D: Read + Seek>(data: &mut D, pos: u64) -> Result<DerFragment, Error> {
    let mut res = DerFragment::Empty;
    let mut tag_buf = [0_u8; 1];
    while data.read_exact(&mut tag_buf).is_ok() {
        let tag_class_u8 = (tag_buf[0] >> 6) & 3;

        let tag_class = match tag_class_u8 {
            0 => TagClass::Universal,
            1 => TagClass::Application,
            2 => TagClass::ContextSpecific,
            3 => TagClass::Private,
            _ => unreachable!(),
        };

        let tag_u8 = tag_buf[0] & 0b00111111;
        let tag = match tag_u8 {
            0x02 => Tag::Integer,
            0x03 => Tag::BitString,
            0x04 => Tag::OctetString,
            0x05 => Tag::Null,
            0x06 => Tag::ObjectIdentifier,
            0x0c => Tag::UTF8String,
            0x30 => Tag::Sequence,
            0x31 => Tag::Set,
            0x13 => Tag::PrintableString,
            0x16 => Tag::IA5String,
            0x17 => Tag::UTCTime,
            0x18 => Tag::GeneralizedTime,
            v => Tag::Other(v),
        };
        let mut len = [0_u8];
        data.read_exact(&mut len)?;
        let long_format = len[0] & 0b1000_0000 > 0;
        let length = if long_format {
            let c = len[0] & 0b0111_1111;
            assert!(c <= 8, "too long");
            let mut len_buf = vec![0_u8; c as usize];
            data.read_exact(&mut len_buf)?;
            // dbg!(&len_buf);
            let mut len = 0_u64;
            for b in len_buf {
                len <<= 8;
                len |= b as u64;
                // dbg!(&len);
            }
            // dbg!(&len);
            len as usize
        } else {
            (len[0] & 0b0111_1111) as usize
        };
        let pos = pos + data.stream_position()?;
        let mut value_buf = vec![0_u8; length];
        data.read_exact(&mut value_buf)?;
        let value = match tag {
            Tag::Sequence | Tag::Set => {
                let r = parse_recursive_inner(&mut Cursor::new(value_buf), pos)?;
                r.try_into()?
            }
            _ => Value::Primitive(value_buf),
        };
        let tlv = Tlv {
            tag_class,
            tag,
            length,
            value,
        };
        res.append(tlv);
    }
    // dbg!(&res);
    Ok(res)
}

// test!(fn )
#[cfg(test)]
mod tests {
    use crate::der::{parse_single, Tag};
    use eyre::Result;
    use std::fs::File;
    use std::io::Cursor;

    #[test]
    fn it_works() -> Result<()> {
        let mut data = Cursor::new([0x30_u8, 0x03, 0x02, 0x01, 0x09]);
        let v = parse_single(&mut data)?;
        dbg!(v);

        let mut data2 = Cursor::new([0x30, 0x06, 0x80, 0x01, 0x09, 0x81, 0x01, 0x09]);
        let v = parse_single(&mut data2)?;
        dbg!(v);
        Ok(())
    }
    #[test]
    fn test_v4_seal() -> Result<()> {
        let mut file = File::open("../samples/SignedValue.dat")?;
        let v = parse_single(&mut file)?;

        assert_eq!(v.tag, Tag::Sequence);
        dbg!(v.item_count());
        assert!(v.item_count() == 4 || v.item_count() == 5);
        // assert!(matches!(v.value,Value::Nested()))
        Ok(())
    }
    #[test]
    fn test_v1_seal() -> Result<()> {
        let mut file = File::open("../samples/UserV1.esl")?;
        let v = parse_single(&mut file)?;
        dbg!(&v);
        assert_eq!(v.tag, Tag::Sequence);
        dbg!(v.item_count());
        assert_eq!(v.item_count(), 2);
        Ok(())
    }
}
