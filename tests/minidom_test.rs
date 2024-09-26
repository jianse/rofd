use eyre::Result;
use minidom::Element;
use rofd::element::base::StLoc;
use rofd::element::file::ofd::{CtDocInfo, DocBody, OfdXmlFile};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::str::FromStr;
use chrono::NaiveDate;
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum TryFromDomError {
    #[error("common error")]
    Common,
    #[error("no attribute named \"{0}\"")]
    NoSuchAttribute(&'static str),
}

const OFD_NS: &str = "http://www.ofdspec.org/2016";

trait TryFromDom<T>: Sized {
    type Error: std::error::Error;
    fn try_from_dom(dom: T) -> std::result::Result<Self, Self::Error>;
}

impl TryFromDom<&minidom::Element> for OfdXmlFile
{
    type Error = TryFromDomError;

    fn try_from_dom(dom: &minidom::Element) -> std::result::Result<Self, Self::Error> {
        let name = dom.name();
        if name != "OFD" {
            return Err(TryFromDomError::Common);
        }
        let version = dom.attr("Version").ok_or(TryFromDomError::NoSuchAttribute("Version"))?;
        let doc_type = dom.attr("DocType").ok_or(TryFromDomError::NoSuchAttribute("DocType"))?;
        let doc_body_result = dom.children().map(DocBody::try_from_dom).collect::<Result<_, _>>();
        let doc_body: Vec<DocBody> = doc_body_result?;
        let res = OfdXmlFile {
            version: version.to_string(),
            doc_type: doc_type.to_string(),
            doc_body,
        };
        Ok(res)
    }
}

impl TryFromDom<&minidom::Element> for DocBody {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> std::result::Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if dom.name() != "DocBody" {
            // TODO: more proper error
            return Err(TryFromDomError::NoSuchAttribute("DocBody"));
        }
        let doc_info_ele = dom.get_child("DocInfo", OFD_NS)
            .ok_or(TryFromDomError::NoSuchAttribute("DocInfo"))?;
        let doc_info = CtDocInfo::try_from_dom(&doc_info_ele)?;

        let doc_root = dom.get_child("DocRoot", OFD_NS)
            .map(StLoc::try_from_dom).transpose()?;

        // TODO: The versions field is not implemented at this time

        let signatures = dom.get_child("Signatures", OFD_NS)
            .map(StLoc::try_from_dom).transpose()?;

        let res = DocBody {
            doc_info,
            doc_root,
            versions: None,
            signatures,
        };
        Ok(res)
    }
}

impl TryFromDom<&Element> for CtDocInfo {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> std::result::Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let doc_id = dom.get_child("DocId", OFD_NS).map(Element::text);

        // let doc_id = doc_id_el;
        let title = dom.get_child("Title", OFD_NS).map(Element::text);
        let author = dom.get_child("Author", OFD_NS).map(Element::text);

        let creation_date = dom.get_child("CreationDate", OFD_NS)
            .map(Element::text)
            .map(|s| { NaiveDate::from_str(s.as_str()) })
            .transpose()
            .map_err(|e| TryFromDomError::Common)?;

        Ok(CtDocInfo {
            doc_id,
            title,
            author,
            creation_date,
            custom_datas: None,
        })
    }
}


impl TryFromDom<&Element> for StLoc {
    type Error = TryFromDomError;
    fn try_from_dom(dom: &Element) -> std::result::Result<Self, Self::Error> {
        let p = dom.text();
        Ok(PathBuf::from(p))
    }
}
#[test]
fn test_try_from_dom() -> Result<()> {
    let file = File::open("samples/sample/ofd.xml")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    let _ = reader.read_to_string(&mut data);
    let root: minidom::Element = data.parse()?;
    let st = OfdXmlFile::try_from_dom(&root)?;
    // dbg!(&root);
    dbg!(&st);
    Ok(())
}