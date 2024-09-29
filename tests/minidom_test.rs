use eyre::Result;
use minidom::Element;
use rofd::dom::TryFromDom;
use rofd::element::file::document::DocumentXmlFile;
use rofd::element::file::ofd::OfdXmlFile;
use std::fs::File;
use std::io::{BufReader, Read};

#[test]
fn test() -> Result<()> {
    let file = File::open("samples/ano/OFD.xml")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    let _ = reader.read_to_string(&mut data);
    let root: Element = data.parse()?;
    dbg!(&root);

    for child in root.children() {
        dbg!(child.text());
    }
    Ok(())
}

#[test]
fn test_try_from_dom_ofd() -> Result<()> {
    let file = File::open("samples/sample/OFD.xml")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    let _ = reader.read_to_string(&mut data);
    let root: minidom::Element = data.parse()?;
    let st = OfdXmlFile::try_from_dom(&root)?;
    // dbg!(&root);
    dbg!(&st);
    Ok(())
}

#[test]
fn test_try_from_dom_doc() -> Result<()> {
    let file = File::open("samples/sample/Doc_0/Document.xml")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    let _ = reader.read_to_string(&mut data);
    let root: Element = data.parse()?;
    let st = DocumentXmlFile::try_from_dom(&root)?;
    // dbg!(&root);
    dbg!(&st);
    Ok(())
}
