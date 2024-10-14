use help_macro::MyDerive;
use rofd::dom::ToElement;

#[derive(MyDerive, Debug)]
struct StructA {
    #[dom(rename = "$value")]
    a: String,
}

#[test]
fn test() {
    let s = StructA {
        a: "hello".to_string(),
    };
    dbg!(s.to_element("test", "", None));
}
