use crate::dom::{
    parse_required_from_attr, parse_required_vec, TryFromDom, TryFromDomError, OFD_NS,
};
use crate::element::base::StLoc;
use crate::element::file::ofd::{CtDocInfo, CustomData, CustomDatas, DocBody, OfdXmlFile};
use chrono::NaiveDate;
use minidom::Element;
use std::path::PathBuf;
use std::str::FromStr;
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
        let doc_info_ele = dom
            .get_child("DocInfo", OFD_NS)
            .ok_or(TryFromDomError::NoSuchAttribute("DocInfo"))?;
        let doc_info = CtDocInfo::try_from_dom(doc_info_ele)?;

        let doc_root = dom
            .get_child("DocRoot", OFD_NS)
            .map(StLoc::try_from_dom)
            .transpose()?;

        // TODO: The versions field is not implemented at this time

        let signatures = dom
            .get_child("Signatures", OFD_NS)
            .map(StLoc::try_from_dom)
            .transpose()?;

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
        let doc_id = dom.get_child("DocId", OFD_NS).map(Element::text);
        let title = dom.get_child("Title", OFD_NS).map(Element::text);
        let author = dom.get_child("Author", OFD_NS).map(Element::text);

        let creation_date = dom
            .get_child("CreationDate", OFD_NS)
            .map(Element::text)
            .map(|s| NaiveDate::from_str(s.as_str()))
            .transpose()
            .map_err(|e| TryFromDomError::Warp(Box::new(e)))?;
        let custom_datas = dom
            .get_child("CustomDatas", OFD_NS)
            .map(CustomDatas::try_from_dom)
            .transpose()?;
        Ok(CtDocInfo {
            doc_id,
            title,
            author,
            creation_date,
            custom_datas,
        })
    }
}
impl TryFromDom<&Element> for CustomDatas {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let custom_data_result = dom
            .children()
            .map(CustomData::try_from_dom)
            .collect::<eyre::Result<_, _>>();
        let custom_data: Vec<CustomData> = custom_data_result?;
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
        let name = dom
            .attr("Name")
            .ok_or(TryFromDomError::NoSuchAttribute("Name"))?;
        let value = dom.text();
        Ok(CustomData {
            name: name.to_string(),
            value,
        })
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
        Ok(())
    }
}
