use eyre::Result;
use minidom::Element;
use ofd_base::file::annotation::AnnotationXmlFile;
use ofd_base::file::document::DocumentXmlFile;
use ofd_base::file::ofd::OfdXmlFile;
use ofd_base::file::page::PageXmlFile;
use ofd_misc::dom::OFD_NS;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use xdom::de::{from_ele, XmlDe};

pub fn init_tracing_subscriber() {
    use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
    let fmt = fmt::layer()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true);
    let filter = filter::LevelFilter::TRACE;
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt)
        .try_init();
}

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
    let s = read_to_string("../samples/000/OFD.xml")?;
    let root: Element = s.parse()?;

    let mut de = XmlDe::from_ele(&root);

    let st = OfdXmlFile::deserialize(&mut de)?;
    println!("{:#?}", st);

    Ok(())
}

#[test]
fn test_de_doc() -> Result<()> {
    let s = read_to_string("../samples/000/Doc_0/Document.xml")?;
    let root: Element = s.parse()?;
    let mut de = XmlDe::from_ele(&root);
    let st = DocumentXmlFile::deserialize(&mut de)?;
    println!("{:#?}", st);
    Ok(())
}

#[test]
fn test_de_page() -> Result<()> {
    let s = read_to_string("../samples/000/Doc_0/Pages/Page_0/Content.xml")?;

    let root: Element = s.parse()?;
    let mut de = XmlDe::from_ele(&root);
    let st = PageXmlFile::deserialize(&mut de)?;
    println!("{:#?}", st);
    Ok(())
}

#[test]
fn test_ano() -> Result<()> {
    init_tracing_subscriber();
    let file = File::open("../samples/002/Doc_0/Pages/Page_1/Annotation.xml")?;
    let reader = BufReader::new(file);
    let root: Element = Element::from_reader(reader)?;

    let st = from_ele::<AnnotationXmlFile>(&root)?;
    dbg!(st);

    Ok(())
}

#[test]
fn test_ano2() -> Result<()> {
    init_tracing_subscriber();
    let file = File::open("../samples/001/Doc_0/Annots/Page_0/Annotation.xml")?;
    let mut reader = BufReader::new(file);
    let buf = reader.fill_buf()?;

    // UTF-8 BOM
    // handle u+FEFF in utf-8 file
    // just skip this three bytes
    if buf.starts_with(&[0xef_u8, 0xbb, 0xbf]) {
        reader.consume(3);
    }
    let root: Element = Element::from_reader_with_prefixes(reader, OFD_NS.to_string())?;

    let st = from_ele::<AnnotationXmlFile>(&root)?;
    dbg!(st);

    Ok(())
}
