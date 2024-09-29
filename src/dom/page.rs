use crate::dom::{
    parse_optional_from_attr, parse_optional_from_ele, parse_optional_vec,
    parse_required_from_attr, parse_required_from_text, parse_required_vec, TryFromDom,
    TryFromDomError,
};
use crate::element::base::{StArray, StBox, StId, StLoc, StRefId};
use crate::element::common::{Actions, Cap, CtColor, Join};
use crate::element::file::document::CtPageArea;
use crate::element::file::page::{Content, CtPageBlock, ImageObject, Layer, PageXmlFile, PathObject, Template, TextCode, TextObject};
use minidom::Element;
use std::str::FromStr;

impl TryFromDom<&Element> for PageXmlFile {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let area = parse_optional_from_ele(dom, "Area", CtPageArea::try_from_dom)?;
        let template = parse_optional_vec(dom, Some("Template"), Template::try_from_dom)?;
        let page_res = parse_optional_vec(dom, Some("PageRes"), StLoc::try_from_dom)?;
        let content = parse_optional_from_ele(dom, "Content", Content::try_from_dom)?;
        Ok(PageXmlFile {
            area,
            template,
            page_res,
            content,
        })
    }
}

impl TryFromDom<&Element> for Template {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let template_id = parse_required_from_attr(dom, "TemplateID", StRefId::from_str)?;
        let z_order = parse_optional_from_attr(dom, "zOrder", String::from_str)?;
        Ok(Template {
            template_id,
            z_order,
        })
    }
}

impl TryFromDom<&Element> for Content {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let layer = parse_required_vec(dom, Some("Layer"), Layer::try_from_dom)?;
        Ok(Content { layer })
    }
}

impl TryFromDom<&Element> for Layer {
    type Error = TryFromDomError;
    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let r#type = parse_optional_from_attr(dom, "Type", String::from_str)?;
        let draw_param = parse_optional_from_attr(dom, "DrawParam", StRefId::from_str)?;
        let id = parse_required_from_attr(dom, "ID", StId::from_str)?;
        let objects = parse_optional_vec(dom, None, CtPageBlock::try_from_dom)?;
        Ok(Layer {
            r#type,
            draw_param,
            id,
            objects,
        })
    }
}

macro_rules! parse_graphic_unit {
    ($dom:ident,&mut $obj:ident) => {
        $obj.boundary = parse_required_from_attr($dom, "Boundary", StBox::from_str)?;
        $obj.name = parse_optional_from_attr($dom, "Name", String::from_str)?;
        $obj.visible = parse_optional_from_attr($dom, "Visible", bool::from_str)?;
        $obj.ctm = parse_optional_from_attr($dom, "CTM", StArray::from_str)?;
        $obj.draw_param = parse_optional_from_attr($dom, "DrawParam", StRefId::from_str)?;
        $obj.line_width = parse_optional_from_attr($dom, "LineWidth", f32::from_str)?;
        $obj.cap = parse_optional_from_attr($dom, "Cap", Cap::from_str)?;
        $obj.join = parse_optional_from_attr($dom, "Join", Join::from_str)?;
        $obj.miter_limit = parse_optional_from_attr($dom, "MiterLimit", f32::from_str)?;
        $obj.dash_offset = parse_optional_from_attr($dom, "DashOffset", f32::from_str)?;
        $obj.dash_pattern = parse_optional_from_attr($dom, "DashPattern", StArray::from_str)?;
        $obj.alpha = parse_optional_from_attr($dom, "Alpha", u8::from_str)?;
        $obj.actions = parse_optional_from_ele($dom, "Actions", Actions::try_from_dom)?;
    };
}

impl TryFromDom<&Element> for CtPageBlock {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let name = dom.name();
        match name {
            "TextObject" => {
                let font = parse_required_from_attr(dom, "Font", StRefId::from_str)?;
                let size = parse_required_from_attr(dom, "Size", f32::from_str)?;
                let stroke = parse_optional_from_attr(dom, "Stoke", bool::from_str)?;
                let fill = parse_optional_from_attr(dom, "Fill", bool::from_str)?;
                let h_scale = parse_optional_from_attr(dom, "HScale", f32::from_str)?;
                let read_direction = parse_optional_from_attr(dom, "ReadDirection", u32::from_str)?;
                let char_direction = parse_optional_from_attr(dom, "CharDirection", u32::from_str)?;
                let weight = parse_optional_from_attr(dom, "Weight", u32::from_str)?;
                let italic = parse_optional_from_attr(dom, "Italic", bool::from_str)?;
                let fill_color = parse_optional_from_ele(dom, "FillColor", CtColor::try_from_dom)?;
                let stroke_color = parse_optional_from_ele(dom, "StrokeColor", CtColor::try_from_dom)?;
                let text_codes =
                    parse_required_vec(dom, Some("TextCode"), TextCode::try_from_dom)?;
                let mut to = TextObject {
                    font,
                    size,
                    stroke,
                    fill,
                    h_scale,
                    read_direction,
                    char_direction,
                    weight,
                    italic,
                    fill_color,
                    stroke_color,
                    text_codes,
                    // following fields are common graphic unit fields
                    boundary: StBox::zero(),
                    name: None,
                    visible: None,
                    ctm: None,
                    draw_param: None,
                    line_width: None,
                    cap: None,
                    join: None,
                    miter_limit: None,
                    dash_offset: None,
                    dash_pattern: None,
                    alpha: None,
                    actions: None,
                };
                parse_graphic_unit!(dom, &mut to);
                Ok(CtPageBlock::TextObject(to))
            }
            "PathObject" => {
                let abbreviated_data =
                    parse_required_from_text(dom, "AbbreviatedData", StArray::from_str)?;
                let mut po = PathObject {
                    stroke: None,
                    fill: None,
                    rule: None,
                    fill_color: None,
                    stroke_color: None,
                    abbreviated_data,
                    // following fields are common graphic unit fields
                    boundary: StBox::zero(),
                    name: None,
                    visible: None,
                    ctm: None,
                    draw_param: None,
                    line_width: None,
                    cap: None,
                    join: None,
                    miter_limit: None,
                    dash_offset: None,
                    dash_pattern: None,
                    alpha: None,
                    actions: None,
                };
                parse_graphic_unit!(dom, &mut po);
                Ok(CtPageBlock::PathObject(po))
            }
            "ImageObject" => {
                let mut io = ImageObject {
                    resource_id: 0,
                    substitution: None,
                    image_mask: None,
                    border: None,
                    boundary: StBox::zero(),
                    name: None,
                    visible: None,
                    ctm: None,
                    draw_param: None,
                    line_width: None,
                    cap: None,
                    join: None,
                    miter_limit: None,
                    dash_offset: None,
                    dash_pattern: None,
                    alpha: None,
                    actions: None,
                };
                parse_graphic_unit!(dom, &mut io);
                Ok(CtPageBlock::ImageObject(io))
            }
            "CompositeObject" => {
                todo!()
            }
            "PageBlock" => {
                todo!()
            }
            _ => Err(TryFromDomError::ElementNameNotExpected(
                "one of \"TextObject, PathObject, ImageObject, CompositeObject, PageBlock\"",
                name.into(),
            )),
        }
    }
}

impl TryFromDom<&Element> for Actions {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFromDom<&Element> for TextCode {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let x = parse_optional_from_attr(dom, "X", f32::from_str)?;
        let y = parse_optional_from_attr(dom, "Y", f32::from_str)?;
        let delta_x = parse_optional_from_attr(dom, "DeltaX", StArray::from_str)?;
        let delta_y = parse_optional_from_attr(dom, "DeltaY", StArray::from_str)?;
        let val = dom.text();
        Ok(TextCode {
            x,
            y,
            delta_x,
            delta_y,
            val,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::dom::TryFromDom;
    use crate::element::file::page::{PageXmlFile, TextCode};
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

    #[test]
    fn test_text_code_empty() -> Result<()> {
        let data = r#"<?xml version="1.0" encoding="UTF-8"?>
        <TextCode xmlns="http://www.ofdspec.org/2016"> </TextCode>
        "#;
        let data = dbg!(data);
        let root: Element = data.parse()?;
        let st = TextCode::try_from_dom(&root)?;
        dbg!(&st);
        Ok(())
    }
}
