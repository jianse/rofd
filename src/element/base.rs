use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use std::{fmt::Display, path::PathBuf, str::FromStr};
use thiserror::Error;

pub type StLoc = PathBuf;

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct StArray<T: FromStr + Display>(pub Vec<T>);

impl<T: FromStr + Display> Deref for StArray<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: FromStr + Display> FromStr for StArray<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let parts = s.split(' ');
        let data = parts
            .filter(|p| !p.is_empty())
            .map(T::from_str)
            .collect::<Result<Vec<T>, <T as FromStr>::Err>>();
        match data {
            Ok(data) => Ok(Self(data)),
            Err(e) => Err(e),
        }
    }
}
impl<T: FromStr + Display> Display for StArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self
            .0
            .iter()
            .map(|i| format!("{}", i))
            .collect::<Vec<String>>();
        write!(f, "{}", r.join(" "))
    }
}

impl<T: FromStr + Display> From<Vec<T>> for StArray<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

pub type StId = u64;
pub type StRefId = StId;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct StPos {
    x: f32,
    y: f32,
}

// #[serde_as(as = "FromInto<(f32, f32, f32, f32)>")]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct StBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Error, Debug)]
pub enum ParseStBoxError {
    #[error("Element parts must be 4")]
    ElementFormat,
}
impl FromStr for StBox {
    type Err = ParseStBoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let parts = s.split(' ').collect::<Vec<&str>>();
        if parts.len() != 4 {
            return Err(ParseStBoxError::ElementFormat);
        }
        let res = parts
            .iter()
            .map(|s| f32::from_str(s))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseStBoxError::ElementFormat)?;
        Ok(StBox::new(res[0], res[1], res[2], res[3]))
    }
}

impl Display for StBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.x, self.y, self.w, self.h)
    }
}

impl StBox {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
    pub fn get_size(&self) -> (f32, f32) {
        (self.w, self.h)
    }
    pub fn get_tl(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
impl From<(f32, f32, f32, f32)> for StBox {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        StBox {
            x: value.0,
            y: value.1,
            w: value.2,
            h: value.3,
        }
    }
}
impl From<StBox> for (f32, f32, f32, f32) {
    fn from(value: StBox) -> Self {
        (value.x, value.y, value.w, value.h)
    }
}
impl<'de> Deserialize<'de> for StBox {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StBoxVisitor;

        impl<'de> Visitor<'de> for StBoxVisitor {
            type Value = StBox;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("4 space separated numbers")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                fn get<E>(parts: &[&str], index: usize) -> Result<f32, E>
                where
                    E: de::Error,
                {
                    let p_str = parts[index];

                    let res: f32 = p_str.parse().map_err(|_| {
                        de::Error::invalid_value(de::Unexpected::Str(p_str), &"number like")
                    })?;
                    Ok(res)
                }

                let parts: Vec<&str> = v.split(' ').collect();
                if parts.len() != 4 {
                    Err(de::Error::invalid_length(parts.len(), &"4"))
                } else {
                    let x = get(&parts, 0)?;
                    let y = get(&parts, 1)?;
                    let w = get(&parts, 2)?;
                    let h = get(&parts, 3)?;
                    Ok(StBox::new(x, y, w, h))
                }
            }
        }
        deserializer.deserialize_string(StBoxVisitor)
    }
}

impl serde::Serialize for StBox {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{}", self).as_str())
    }
}

impl<'de, T: FromStr + Display> Deserialize<'de> for StArray<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StArrayVisitor<C: FromStr + Display> {
            marker: PhantomData<fn() -> StArray<C>>,
        }
        impl<'de, C: FromStr + Display> Visitor<'de> for StArrayVisitor<C> {
            type Value = StArray<C>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("space separated numbers")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let parts = v
                    .split(' ')
                    .map(|s| s.parse::<C>())
                    .collect::<Result<_, _>>()
                    .map_err(|_| {
                        de::Error::invalid_value(de::Unexpected::Str(v), &"something can parse")
                    })?;
                Ok(StArray(parts))
            }
        }
        deserializer.deserialize_string(StArrayVisitor {
            marker: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stbox_de() {
        let res = quick_xml::de::from_str::<StBox>("<StBox>0 0 160 35.5</StBox>");
        let res = dbg!(res);
        let r = res.unwrap();
        assert_eq!(r, StBox::new(0.0, 0.0, 160.0, 35.5))
    }
    use eyre::Result;
    use serde_with::serde_as;
    use serde_with::DisplayFromStr;
    #[serde_as]
    #[derive(Debug, Deserialize)]
    struct E {
        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "$value")]
        val: StArray<u32>,
    }
    #[test]
    fn test_st_array_de() -> Result<()> {
        let xml = r#"<e>2 3 4</e>"#;
        let res = quick_xml::de::from_str::<E>(xml)?;
        // dbg!(res);
        assert_eq!(res.val, StArray::from(vec![2, 3, 4]));
        Ok(())
    }

    #[serde_as]
    #[derive(Debug, Deserialize)]
    struct StrList {
        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "$value")]
        val: StArray<String>,
    }
    #[test]
    fn test_st_array_blank_de() -> Result<()> {
        let xml = r#"<e>g 4 1.5875  3.175 g 2 1.5875  3.175 g 2 1.5875 -19.05</e>"#;
        let res = quick_xml::de::from_str::<StrList>(xml)?;
        // dbg!(res);
        assert_eq!(
            res.val.0,
            vec![
                "g", "4", "1.5875", "3.175", "g", "2", "1.5875", "3.175", "g", "2", "1.5875",
                "-19.05"
            ]
        );
        Ok(())
    }
}
