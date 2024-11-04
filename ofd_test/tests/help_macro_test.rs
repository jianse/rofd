use help_macro::MyDerive;
use ofd_misc::{ToElement, ToNode};

#[derive(MyDerive, Debug)]
struct StructA {
    #[dom(rename = "@Attr1")]
    attr1: String,
    #[dom(rename = "$text")]
    a: String,

    ele: Option<String>,
}

#[test]
fn test() {
    let s = StructA {
        attr1: "a".to_owned(),
        a: "hello".to_string(),
        ele: Some("world".to_owned()),
    };
    let element = s.to_element("test", OFD_NS, Some("ofd".into())).unwrap();
    dbg!(&element);
    dbg!(&element.text());
    // element.append_child()

    let mut buf = Vec::new();
    let _ = element.write_to_decl(&mut buf);
    let xml_str = String::from_utf8(buf).unwrap();
    println!("{}", xml_str);
    // Some(?)
}

use ofd_misc::dom::OFD_NS;
