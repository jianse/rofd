use crate::dom::{parse_optional_from_ele, TryFromDom, TryFromDomError};
use crate::element::file::document::CtPageArea;
use crate::element::file::page::PageXmlFile;
use minidom::Element;

impl TryFromDom<&Element> for PageXmlFile {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let area = parse_optional_from_ele(dom, "Area", CtPageArea::try_from_dom)?;
        Ok(PageXmlFile {
            area,
            template: None,
            page_res: None,
            content: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::dom::TryFromDom;
    use crate::element::file::page::PageXmlFile;
    use eyre::Result;
    use minidom::Element;
    use std::fs::File;
    use std::io::{BufReader, Read};

    #[test]
    fn test_try_from_dom() -> Result<()> {
        let file = File::open("samples/sample/Doc_0/Pages/Page_0/Content.xml")?;
        let mut reader = BufReader::new(file);
        let mut data = String::new();
        let _ = reader.read_to_string(&mut data);
        let root: Element = data.parse()?;
        let st = PageXmlFile::try_from_dom(&root)?;
        dbg!(&st);
        Ok(())
    }
}
