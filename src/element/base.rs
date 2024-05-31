use std::path::PathBuf;

// use eyre::Ok;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

pub type StLoc = PathBuf;
pub type StArray<T> = Vec<T>;
pub type StId = u64;
pub type StRefId = StId;
pub struct StPos {
    x: f64,
    y: f64,
}

// #[serde_as(as = "FromInto<(f64, f64, f64, f64)>")]
#[derive(Debug, Serialize, PartialEq, PartialOrd)]
pub struct StBox {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}
impl StBox {
    fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }
}
impl From<(f64, f64, f64, f64)> for StBox {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        StBox {
            x: value.0,
            y: value.1,
            w: value.2,
            h: value.3,
        }
    }
}
impl From<StBox> for (f64, f64, f64, f64) {
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
                formatter.write_str("4 space seprated numbers")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                fn get<E>(parts: &Vec<&str>, index: usize) -> Result<f64, E>
                where
                    E: de::Error,
                {
                    let p_str = parts[index];

                    let res: f64 = p_str.parse().map_err(|_| {
                        de::Error::invalid_value(de::Unexpected::Str(p_str), &"number like")
                    })?;
                    Ok(res)
                }

                // let v = dbg!(v);
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
mod test_de {
    use super::*;

    #[test]
    fn test_vec_de() {
        let r: Vec<f64> = quick_xml::de::from_str("<StBox>a.0 0 160 35.5</StBox>").unwrap();
        assert_eq!(r, vec![0.0, 0.0, 160.0, 35.5])
    }
    #[test]
    fn test_stbox_de() {
        let res = quick_xml::de::from_str::<StBox>("<StBox>0 0 160 35.5</StBox>");
        let res = dbg!(res);
        let r = res.unwrap();
        assert_eq!(r, StBox::new(0.0, 0.0, 160.0, 35.5))
    }
}