use std::{fmt::Display, path::PathBuf, str::FromStr};

// use eyre::Ok;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use serde_with::serde_as;

pub type StLoc = PathBuf;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct StArray<T: FromStr + Display>(pub Vec<T>);

impl<T: FromStr + Display> FromStr for StArray<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let parts = s.split(" ");
        let data = parts
            .into_iter()
            .map(T::from_str)
            .collect::<Result<Vec<T>, <T as FromStr>::Err>>();
        match data {
            Ok(data) => Ok(Self { 0: data }),
            Err(e) => Err(e),
        }
        // todo!()
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
        Self { 0: value }
    }
}

pub type StId = u64;
pub type StRefId = StId;

#[allow(dead_code)]
pub struct StPos {
    x: f32,
    y: f32,
}

// #[serde_as(as = "FromInto<(f32, f32, f32, f32)>")]
#[derive(Debug, Serialize, PartialEq, PartialOrd, Clone, Copy)]
pub struct StBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl StBox {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
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
                fn get<E>(parts: &Vec<&str>, index: usize) -> Result<f32, E>
                where
                    E: de::Error,
                {
                    let p_str = parts[index];

                    let res: f32 = p_str.parse().map_err(|_| {
                        de::Error::invalid_value(de::Unexpected::Str(p_str), &"number like")
                    })?;
                    Ok(res)
                }

                let parts: Vec<&str> = v.split(" ").collect();
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
}
