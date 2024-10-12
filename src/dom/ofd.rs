use crate::element::base::StLoc;
use crate::element::file::ofd::{CtDocInfo, CustomData, CustomDatas, DocBody, OfdXmlFile};
use chrono::NaiveDate;
use minidom::Element;
use std::path::PathBuf;
use std::str::FromStr;

use super::{
    parse_optional_from_ele, parse_optional_from_text, parse_required_from_attr,
    parse_required_from_ele, parse_required_vec, TryFromDom, TryFromDomError,
};

impl TryFromDom<Element> for OfdXmlFile {
    fn try_from_dom(dom: Element) -> Result<Self, TryFromDomError> {
        let version = parse_required_from_attr(&dom, "Version", String::from_str)?;
        let doc_type = parse_required_from_attr(&dom, "DocType", String::from_str)?;
        let doc_body = parse_required_vec(&dom, None, DocBody::try_from_dom)?;
        Ok(OfdXmlFile {
            version,
            doc_type,
            doc_body,
        })
    }
}
impl TryFromDom<&Element> for OfdXmlFile {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let name = dom.name();
        if name != "OFD" {
            return Err(TryFromDomError::ElementNameNotExpected(
                "OFD",
                name.to_string(),
            ));
        }
        let version = parse_required_from_attr(dom, "Version", String::from_str)?;
        let doc_type = parse_required_from_attr(dom, "DocType", String::from_str)?;
        let doc_body = parse_required_vec(dom, None, DocBody::try_from_dom)?;
        let res = OfdXmlFile {
            version,
            doc_type,
            doc_body,
        };
        Ok(res)
    }
}

impl TryFromDom<&Element> for DocBody {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError>
    where
        Self: Sized,
    {
        if dom.name() != "DocBody" {
            return Err(TryFromDomError::ElementNameNotExpected(
                "DocBody",
                dom.name().to_string(),
            ));
        }

        let doc_info = parse_required_from_ele(dom, "DocInfo", CtDocInfo::try_from_dom)?;
        let doc_root = parse_optional_from_text(dom, "DocRoot", StLoc::from_str)?;

        // TODO: The versions field is not implemented at this time

        let signatures = parse_optional_from_ele(dom, "Signatures", StLoc::try_from_dom)?;

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
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError>
    where
        Self: Sized,
    {
        let doc_id = parse_optional_from_text(dom, "DocID", String::from_str)?;
        let title = parse_optional_from_text(dom, "Title", String::from_str)?;
        let author = parse_optional_from_text(dom, "Author", String::from_str)?;
        let creation_date = parse_optional_from_text(dom, "CreationDate", NaiveDate::from_str)?;
        let custom_datas = parse_optional_from_ele(dom, "CustomDatas", CustomDatas::try_from_dom)?;

        // TODO: parse missing fields
        Ok(CtDocInfo {
            doc_id,
            title,
            author,
            subject: None,
            r#abstract: None,
            creation_date,
            mod_date: None,
            doc_usage: None,
            cover: None,
            keywords: None,
            creator: None,
            creator_version: None,
            custom_datas,
        })
    }
}
impl TryFromDom<&Element> for CustomDatas {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let custom_data = parse_required_vec(dom, None, CustomData::try_from_dom)?;
        Ok(CustomDatas { custom_data })
    }
}

impl TryFromDom<&Element> for CustomData {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        if dom.name() != "CustomData" {
            return Err(TryFromDomError::ElementNameNotExpected(
                "CustomData",
                dom.name().to_string(),
            ));
        }
        let name = parse_required_from_attr(dom, "Name", String::from_str)?;
        let value = dom.text();
        Ok(CustomData { name, value })
    }
}

/// impl StLoc from an element
impl TryFromDom<&Element> for StLoc {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let p = dom.text();
        Ok(PathBuf::from(p))
    }
}

#[cfg(test)]
mod tests {
    use crate::dom::TryFromDom;
    use crate::element::file::ofd::OfdXmlFile;
    use minidom::Element;
    use std::fs::File;
    use std::io::{BufReader, Read};

    #[test]
    fn test_try_from_dom_ofd() -> eyre::Result<()> {
        let file = File::open("samples/sample/OFD.xml")?;
        let mut reader = BufReader::new(file);
        let mut data = String::new();
        let _ = reader.read_to_string(&mut data);
        let root: Element = data.parse()?;
        let st = OfdXmlFile::try_from_dom(root)?;
        // dbg!(&root);
        dbg!(&st);
        assert_eq!(
            st.doc_body[0].doc_info.doc_id,
            Some(String::from("44107dc257034d38898838015df3e3ed"))
        );
        Ok(())
    }
}
