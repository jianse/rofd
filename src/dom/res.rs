use crate::dom::{
    parse_optional_from_attr, parse_optional_from_ele, parse_optional_from_text,
    parse_required_from_attr, parse_required_from_ele, parse_required_from_text, TryFromDom,
    TryFromDomError,
};
use crate::element::base::{StArray, StId, StLoc, StRefId};
use crate::element::common::{Cap, CtColor, Join, Palette};
use crate::element::file::page::CtPageBlock;
use crate::element::file::res::{
    ColorSpace, ColorSpaces, CompositeGraphicUnit, CompositeGraphicUnits, DrawParam, DrawParams,
    Font, Fonts, MultiMedia, MultiMedias, ResourceXmlFile, Type,
};
use minidom::Element;
use std::str::FromStr;

impl TryFromDom<&Element> for ResourceXmlFile {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let base_loc = parse_required_from_attr(dom, "BaseLoc", StLoc::from_str)?;

        let color_spaces = parse_optional_from_ele(dom, "ColorSpaces", ColorSpaces::try_from_dom)?;

        let draw_params = parse_optional_from_ele(dom, "DrawParams", DrawParams::try_from_dom)?;

        let fonts = parse_optional_from_ele(dom, "Fonts", Fonts::try_from_dom)?;

        let multi_medias = parse_optional_from_ele(dom, "MultiMedias", MultiMedias::try_from_dom)?;

        let composite_graphic_units = parse_optional_from_ele(
            dom,
            "CompositeGraphicUnits",
            CompositeGraphicUnits::try_from_dom,
        )?;
        Ok(ResourceXmlFile {
            base_loc,
            color_spaces,
            draw_params,
            fonts,
            multi_medias,
            composite_graphic_units,
        })
    }
}

impl TryFromDom<&Element> for CompositeGraphicUnits {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
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
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let id = parse_required_from_attr(dom, "ID", StId::from_str)?;
        let content = parse_required_from_ele(dom, "Content", CtPageBlock::try_from_dom)?;
        Ok(CompositeGraphicUnit { id, content })
    }
}

impl TryFromDom<&Element> for ColorSpaces {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let color_spaces = dom
            .children()
            .map(ColorSpace::try_from_dom)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ColorSpaces { color_spaces })
    }
}

impl TryFromDom<&Element> for ColorSpace {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
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
    type Error = TryFromDomError;

    fn try_from_dom(_dom: &Element) -> Result<Self, Self::Error> {
        // TODO: follow struct definitions
        Ok(Palette {})
    }
}

impl TryFromDom<&Element> for DrawParams {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let draw_params = dom
            .children()
            .map(DrawParam::try_from_dom)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(DrawParams { draw_params })
    }
}

impl TryFromDom<&Element> for DrawParam {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
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
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let value = parse_optional_from_attr(dom, "Value", StArray::from_str)?;
        let index = parse_optional_from_attr(dom, "Index", usize::from_str)?;
        let color_space = parse_optional_from_attr(dom, "ColorSpace", StRefId::from_str)?;
        let alpha = parse_optional_from_attr(dom, "Alpha", u8::from_str)?;
        Ok(CtColor {
            value,
            index,
            color_space,
            alpha,
        })
    }
}

impl TryFromDom<&Element> for Fonts {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let fonts = dom
            .children()
            .map(Font::try_from_dom)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Fonts { fonts })
    }
}

impl TryFromDom<&Element> for Font {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
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
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
        let multi_medias = dom
            .children()
            .map(MultiMedia::try_from_dom)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(MultiMedias { multi_medias })
    }
}

impl TryFromDom<&Element> for MultiMedia {
    type Error = TryFromDomError;

    fn try_from_dom(dom: &Element) -> Result<Self, Self::Error> {
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
