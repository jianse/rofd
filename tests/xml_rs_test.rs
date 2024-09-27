use xml_dom::{level2::convert::as_document, parser::read_xml};

#[test]
fn test() {
    // THIS IS NOT WE EXPECTED
    // trailing space has been trimmed
    let xml = r#"<root>1 </root>"#;
    let dom = read_xml(xml).unwrap();
    let doc = as_document(&dom).unwrap();
    let root_node = doc.document_element().unwrap();
    dbg!(root_node);
}
