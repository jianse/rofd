use crate::dom::{
    parse_optional_from_attr, parse_optional_from_ele, parse_optional_from_text,
    parse_optional_vec, parse_required_from_attr, parse_required_from_ele,
    parse_required_from_text, TryFromDom, TryFromDomError,
};
use crate::element::base::{StArray, StId, StLoc, StRefId};
use crate::element::common::{Cap, CellContent, CtColor, CtPattern, Join, Palette};
use crate::element::file::page::CtPageBlock;
use crate::element::file::res::{
    ColorSpace, ColorSpaces, CompositeGraphicUnit, CompositeGraphicUnits, DrawParam, DrawParams,
    Font, Fonts, MultiMedia, MultiMedias, Resource, ResourceXmlFile, Type,
};
use minidom::Element;
use std::str::FromStr;

use super::parse_required_vec;

impl TryFromDom<Element> for ResourceXmlFile {
    fn try_from_dom(dom: Element) -> Result<Self, TryFromDomError> {
        ResourceXmlFile::try_from_dom(&dom)
    }
}

impl TryFromDom<&Element> for ResourceXmlFile {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let base_loc = parse_required_from_attr(dom, "BaseLoc", StLoc::from_str)?;

        let resources = parse_optional_vec(dom, None, Resource::try_from_dom)?;
        Ok(ResourceXmlFile {
            base_loc,
            resources,
        })
    }
}

impl TryFromDom<&Element> for Resource {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let name = dom.name();
        match name {
            "ColorSpaces" => {
                let color_spaces = ColorSpaces::try_from_dom(dom)?;
                Ok(Resource::ColorSpaces(color_spaces))
            }
            "DrawParams" => {
                let draw_params = DrawParams::try_from_dom(dom)?;
                Ok(Resource::DrawParams(draw_params))
            }
            "Fonts" => {
                let fonts = Fonts::try_from_dom(dom)?;
                Ok(Resource::Fonts(fonts))
            }
            "MultiMedias" => {
                let multi_medias = MultiMedias::try_from_dom(dom)?;
                Ok(Resource::MultiMedias(multi_medias))
            }
            "CompositeGraphicUnits" => {
                let composite_graphic_units = CompositeGraphicUnits::try_from_dom(dom)?;
                Ok(Resource::CompositeGraphicUnits(composite_graphic_units))
            }
            _ => Err(TryFromDomError::ElementNameNotExpected(
                "one of\"ColorSpaces, DrawParams, Fonts, MultiMedias, CompositeGraphicUnits\"",
                name.into(),
            )),
        }
    }
}

impl TryFromDom<&Element> for CompositeGraphicUnits {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let composite_graphic_units = dom
            .children()
            .map(CompositeGraphicUnit::try_from_dom)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(CompositeGraphicUnits {
            composite_graphic_units,
        })
    }
}

impl TryFromDom<&Element> for CompositeGraphicUnit {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let id = parse_required_from_attr(dom, "ID", StId::from_str)?;
        let content = parse_required_from_ele(dom, "Content", CtPageBlock::try_from_dom)?;
        Ok(CompositeGraphicUnit { id, content })
    }
}

impl TryFromDom<&Element> for ColorSpaces {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let color_spaces = dom
            .children()
            .map(ColorSpace::try_from_dom)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ColorSpaces { color_spaces })
    }
}

impl TryFromDom<&Element> for ColorSpace {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let id = parse_required_from_attr(dom, "ID", StId::from_str)?;
        let r#type = parse_required_from_attr(dom, "Type", Type::from_str)?;
        let bits_per_component = parse_optional_from_attr(dom, "BitsPerComponent", u8::from_str)?;
        let profile = parse_optional_from_attr(dom, "Profile", StLoc::from_str)?;
        let palette = parse_optional_from_ele(dom, "Palette", Palette::try_from_dom)?;
        Ok(ColorSpace {
            id,
            r#type,
            bits_per_component,
            profile,
            palette,
        })
    }
}

impl TryFromDom<&Element> for Palette {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let cv = parse_required_vec(dom, Some("CV"), |e| {
            let text = e.text();
            StArray::from_str(&text)
        })?;
        Ok(Palette { cv })
    }
}

impl TryFromDom<&Element> for DrawParams {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let draw_params = dom
            .children()
            .map(DrawParam::try_from_dom)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(DrawParams { draw_params })
    }
}

impl TryFromDom<&Element> for DrawParam {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let id = parse_required_from_attr(dom, "ID", StId::from_str)?;
        let relative = parse_optional_from_attr(dom, "Relative", StRefId::from_str)?;
        let line_width = parse_optional_from_attr(dom, "LineWidth", f32::from_str)?;
        let join = parse_optional_from_attr(dom, "Join", Join::from_str)?;
        let cap = parse_optional_from_attr(dom, "Cap", Cap::from_str)?;
        let dash_offset = parse_optional_from_attr(dom, "DashOffset", f32::from_str)?;
        let dash_pattern = parse_optional_from_attr(dom, "DashPattern", StArray::from_str)?;
        let miter_limit = parse_optional_from_attr(dom, "MiterLimit", f32::from_str)?;
        let fill_color = parse_optional_from_ele(dom, "FillColor", CtColor::try_from_dom)?;
        let stroke_color = parse_optional_from_ele(dom, "FillColor", CtColor::try_from_dom)?;

        Ok(DrawParam {
            id,
            relative,
            line_width,
            join,
            cap,
            dash_offset,
            dash_pattern,
            miter_limit,
            fill_color,
            stroke_color,
        })
    }
}

impl TryFromDom<&Element> for CtColor {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let value = parse_optional_from_attr(dom, "Value", StArray::from_str)?;
        let index = parse_optional_from_attr(dom, "Index", usize::from_str)?;
        let color_space = parse_optional_from_attr(dom, "ColorSpace", StRefId::from_str)?;
        let alpha = parse_optional_from_attr(dom, "Alpha", u8::from_str)?;
        let pattern = parse_optional_from_ele(dom, "Pattern", CtPattern::try_from_dom)?;

        // TODO: SHADOWS
        Ok(CtColor {
            value,
            index,
            color_space,
            alpha,
            pattern,
            axial_shd: None,
            radial_shd: None,
            gouraud_shd: None,
            la_gouraud_shd: None,
        })
    }
}

impl TryFromDom<&Element> for CtPattern {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let width = parse_required_from_attr(dom, "Width", f32::from_str)?;
        let height = parse_required_from_attr(dom, "Height", f32::from_str)?;
        let x_step = parse_optional_from_attr(dom, "XStep", f32::from_str)?;
        let y_step = parse_optional_from_attr(dom, "YStep", f32::from_str)?;
        let reflect_method = parse_optional_from_attr(dom, "ReflectMethod", String::from_str)?;
        let relative_to = parse_optional_from_attr(dom, "RelativeTo", String::from_str)?;
        let ctm = parse_optional_from_attr(dom, "CTM", StArray::from_str)?;
        let cell_content = parse_required_vec(dom, Some("CellContent"), CellContent::try_from_dom)?;
        Ok(CtPattern {
            width,
            height,
            x_step,
            y_step,
            reflect_method,
            relative_to,
            ctm,
            cell_content,
        })
    }
}
impl TryFromDom<&Element> for CellContent {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let thumbnail = parse_optional_from_attr(dom, "Thumbnail", StRefId::from_str)?;
        let base = CtPageBlock::try_from_dom(dom)?;
        Ok(CellContent { thumbnail, base })
    }
}

impl TryFromDom<&Element> for Fonts {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let fonts = dom
            .children()
            .map(Font::try_from_dom)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Fonts { fonts })
    }
}

impl TryFromDom<&Element> for Font {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let id = parse_required_from_attr(dom, "ID", StId::from_str)?;
        let font_name = parse_required_from_attr(dom, "FontName", String::from_str)?;
        let family_name = parse_optional_from_attr(dom, "FamilyName", String::from_str)?;
        let charset = parse_optional_from_attr(dom, "Charset", String::from_str)?;
        let italic = parse_optional_from_attr(dom, "Italic", bool::from_str)?;
        let bold = parse_optional_from_attr(dom, "Bold", bool::from_str)?;
        let serif = parse_optional_from_attr(dom, "Serif", bool::from_str)?;
        let fixed_width = parse_optional_from_attr(dom, "FixedWidth", bool::from_str)?;
        let font_file = parse_optional_from_text(dom, "FontFile", StLoc::from_str)?;
        Ok(Font {
            id,
            font_name,
            family_name,
            charset,
            italic,
            bold,
            serif,
            fixed_width,
            font_file,
        })
    }
}

impl TryFromDom<&Element> for MultiMedias {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let multi_medias = dom
            .children()
            .map(MultiMedia::try_from_dom)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(MultiMedias { multi_medias })
    }
}

impl TryFromDom<&Element> for MultiMedia {
    fn try_from_dom(dom: &Element) -> Result<Self, TryFromDomError> {
        let id = parse_required_from_attr(dom, "ID", StId::from_str)?;
        let r#type = parse_required_from_attr(dom, "Type", String::from_str)?;
        let format = parse_optional_from_attr(dom, "Format", String::from_str)?;
        let media_file = parse_required_from_text(dom, "MediaFile", StLoc::from_str)?;
        Ok(MultiMedia {
            id,
            r#type,
            format,
            media_file,
        })
    }
}
#[cfg(test)]
mod tests {
    use crate::dom::TryFromDom;
    use crate::element::file::res::ResourceXmlFile;
    use eyre::Result;
    use minidom::Element;
    use std::fs::File;
    use std::io::{BufReader, Read};

    #[test]
    fn test_try_from_dom_doc_res() -> Result<()> {
        let file = File::open("samples/sample/Doc_0/DocumentRes.xml")?;
        let mut reader = BufReader::new(file);
        let mut data = String::new();
        reader.read_to_string(&mut data)?;
        let root: Element = data.parse()?;
        let st = ResourceXmlFile::try_from_dom(&root)?;
        dbg!(&st);
        Ok(())
    }

    #[test]
    fn test_try_from_dom_pub_res() -> Result<()> {
        let file = File::open("samples/sample/Doc_0/PublicRes.xml")?;
        let mut reader = BufReader::new(file);
        let mut data = String::new();
        reader.read_to_string(&mut data)?;
        let root: Element = data.parse()?;
        let st = ResourceXmlFile::try_from_dom(&root)?;
        dbg!(&st);
        Ok(())
    }
}
