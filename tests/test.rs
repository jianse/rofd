use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Outter {
    #[serde(flatten)]
    base: Inner,

    #[serde(rename = "@ID")]
    id: String,
}
#[derive(Debug, Deserialize)]
struct Inner {
    #[serde(rename = "@Type")]
    r#type: Option<String>,
}

fn main() {
    let res = quick_xml::de::from_str::<Outter>(
        r#"<Outter Type="AAA" DrawParam="4" ID="123">content</TestStruct>"#,
    );
    let _ = dbg!(res);
}
#[cfg(test)]
mod test_super {
    use help_macro::extends;

    use super::*;

    struct Foo {
        foo: f64,
    }
    #[extends(Foo)]
    #[derive(Debug)]
    struct Sub {
        aa: u64,
    }

    #[test]
    fn test_() {
        let sub = Sub { aa: 64 };
        let sub = dbg!(sub);
        assert_eq!(sub.aa, 64)
    }
}
