use eyre::Result;
use minidom::Element;
use ofd_base::file::document::DocumentXmlFile;
use ofd_base::file::ofd::OfdXmlFile;
use ofd_base::file::page::PageXmlFile;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use xdom::de::XmlDe;

fn read_to_string<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut s = String::new();
    let _ = file.read_to_string(&mut s)?;
    Ok(s)
}

#[test]
fn test_de_ofd() -> Result<()> {
    let s = read_to_string("samples/sample/OFD.xml")?;
    let root: Element = s.parse()?;

    let mut de = XmlDe::from_ele(&root);

    let st = OfdXmlFile::deserialize(&mut de)?;
    println!("{:#?}", st);

    Ok(())
}

#[test]
fn test_de_doc() -> Result<()> {
    let s = read_to_string("samples/sample/Doc_0/Document.xml")?;
    let root: Element = s.parse()?;
    let mut de = XmlDe::from_ele(&root);
    let st = DocumentXmlFile::deserialize(&mut de)?;
    println!("{:#?}", st);
    Ok(())
}

#[test]
fn test_de_page() -> Result<()> {
    let s = read_to_string("samples/sample/Doc_0/Pages/Page_0/Content.xml")?;

    let root: Element = s.parse()?;
    let mut de = XmlDe::from_ele(&root);
    let st = PageXmlFile::deserialize(&mut de)?;
    println!("{:#?}", st);
    Ok(())
}
