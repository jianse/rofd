use crate::dom::{
    parse_optional_from_attr, parse_required_from_attr, TryFromDom, TryFromDomError, OFD_NS,
};
use base::file::document::{CommonData, CtPageArea, DocumentXmlFile, Page, Pages, TemplatePage};
use base::{StBox, StId, StLoc, StRefId};
use minidom::Element;
use std::str::FromStr;

impl TryFromDom<Element> for DocumentXmlFile {
    fn try_from_dom(dom: Element) -> Result<Self, TryFromDomError> {
        DocumentXmlFile::try_from_dom(&dom)
    }
}
impl TryFromDom<&Element> for DocumentXmlFile {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        if dom.name() != "Document" {
            return Err(TryFromDomError::ElementNameNotExpected(
                "Document",
                dom.name().to_owned(),
            ));
        }
        let common_data_el = dom
            .get_child("CommonData", OFD_NS)
            .ok_or(TryFromDomError::NoSuchAttribute("CommonData"))?;
        let common_data = CommonData::try_from_dom(common_data_el)?;

        let pages = dom
            .get_child("Pages", OFD_NS)
            .map(Pages::try_from_dom)
            .transpose()?
            .ok_or(TryFromDomError::NoSuchAttribute("Pages"))?;

        #[inline(always)]
        fn parse_optional_st_loc_from_ele(
            dom: &Element,
            el_name: &str,
        ) -> Result<Option<StLoc>, TryFromDomError> {
            let r = dom
                .get_child(el_name, OFD_NS)
                .map(Element::text)
                .map(|s| StLoc::from_str(s.as_str()))
                .transpose()
                .map_err(|e| TryFromDomError::Warp(Box::new(e)))?;
            Ok(r)
        }

        let annotations = parse_optional_st_loc_from_ele(dom, "Annotations")?;
        let attachments = parse_optional_st_loc_from_ele(dom, "Attachments")?;
        let custom_tags = parse_optional_st_loc_from_ele(dom, "CustomTags")?;

        // TODO: parse missing fields
        Ok(DocumentXmlFile {
            common_data,
            pages,
            outlines: None,
            permissions: None,
            actions: None,
            v_preferences: None,
            bookmarks: None,
            annotations,
            attachments,
            custom_tags,
            extensions: None,
        })
    }
}

impl TryFromDom<&Element> for CommonData {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let max_unit_id = dom
            .get_child("MaxUnitID", OFD_NS)
            .ok_or(TryFromDomError::NoSuchAttribute("MaxUnitID"))
            .map(Element::text)?
            .parse::<StId>()
            .map_err(|e| TryFromDomError::Warp(Box::new(e)))?;

        let page_area_el = dom
            .get_child("PageArea", OFD_NS)
            .ok_or(TryFromDomError::NoSuchAttribute("PageArea"))?;
        let page_area = CtPageArea::try_from_dom(page_area_el)?;

        #[inline(always)]
        fn parse_optional_st_loc_list(
            dom: &Element,
            el_name: &str,
        ) -> Result<Option<Vec<StLoc>>, TryFromDomError> {
            let loc_vec = dom
                .children()
                .filter(|c| c.name() == el_name)
                .map(Element::text)
                .map(|s| StLoc::from_str(s.as_str()))
                .collect::<Result<Vec<StLoc>, _>>()
                .map_err(|e| TryFromDomError::Warp(Box::new(e)))?;
            let opt_vec = if loc_vec.is_empty() {
                None
            } else {
                Some(loc_vec)
            };
            Ok(opt_vec)
        }
        let public_res = parse_optional_st_loc_list(dom, "PublicRes")?;
        let document_res = parse_optional_st_loc_list(dom, "DocumentRes")?;

        let template_page_vec = dom
            .children()
            .filter(|c| c.name() == "TemplatePage")
            .map(TemplatePage::try_from_dom)
            .collect::<Result<Vec<_>, _>>()?;
        let template_page = if !template_page_vec.is_empty() {
            Some(template_page_vec)
        } else {
            None
        };

        let default_cs = dom
            .get_child("DefaultCS", OFD_NS)
            .map(Element::text)
            .map(|s| StRefId::from_str(s.as_str()))
            .transpose()
            .map_err(|e| TryFromDomError::Warp(Box::new(e)))?;

        Ok(CommonData {
            max_unit_id,
            page_area,
            public_res,
            document_res,
            template_page,
            default_cs,
        })
    }
}

impl TryFromDom<&Element> for CtPageArea {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        #[inline(always)]
        fn parse_optional_from_ele(
            dom: &Element,
            el_name: &str,
        ) -> Result<Option<StBox>, TryFromDomError> {
            let r = dom
                .get_child(el_name, OFD_NS)
                .map(Element::text)
                .map(|s| StBox::from_str(s.as_str()))
                .transpose()
                .map_err(|e| TryFromDomError::Warp(Box::new(e)))?;
            Ok(r)
        }
        #[inline(always)]
        fn parse_required_from_ele(
            dom: &Element,
            el_name: &'static str,
        ) -> Result<StBox, TryFromDomError> {
            let r = parse_optional_from_ele(dom, el_name)?
                .ok_or(TryFromDomError::NoSuchAttribute(el_name))?;
            Ok(r)
        }

        let physical_box = parse_required_from_ele(dom, "PhysicalBox")?;
        let application_box = parse_optional_from_ele(dom, "ApplicationBox")?;
        let content_box = parse_optional_from_ele(dom, "ContentBox")?;
        let bleed_box = parse_optional_from_ele(dom, "BleedBox")?;
        Ok(CtPageArea {
            physical_box,
            application_box,
            content_box,
            bleed_box,
        })
    }
}

impl TryFromDom<&Element> for Pages {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let page = dom
            .children()
            .map(Page::try_from_dom)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Pages { page })
    }
}
impl TryFromDom<&Element> for Page {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let id = parse_required_from_attr(dom, "ID", StId::from_str)?;
        let base_loc = parse_required_from_attr(dom, "BaseLoc", StLoc::from_str)?;
        Ok(Page { id, base_loc })
    }
}

impl TryFromDom<&Element> for TemplatePage {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let id = parse_required_from_attr(dom, "ID", StId::from_str)?;

        let name = parse_optional_from_attr(dom, "Name", String::from_str)?;

        let z_order = parse_optional_from_attr(dom, "ZOrder", String::from_str)?;

        let base_loc = parse_required_from_attr(dom, "BaseLoc", StLoc::from_str)?;

        Ok(TemplatePage {
            id,
            name,
            z_order,
            base_loc,
        })
    }
}
#[cfg(test)]
mod tests {
    use crate::dom::TryFromDom;
    use base::file::document::DocumentXmlFile;
    use eyre::Result;
    use minidom::Element;
    use std::fs::File;
    use std::io::{BufReader, Read};

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
}
