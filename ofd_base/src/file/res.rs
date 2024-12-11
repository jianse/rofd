use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::{
    base::{StArray, StId, StLoc, StRefId},
    common::{Cap, CtColor, Join, Palette},
};

use super::page::VtGraphicUnit;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceXmlFile {
    #[serde(rename = "@BaseLoc")]
    pub base_loc: StLoc,

    #[serde(rename = "$value")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Resource {
    ColorSpaces(ColorSpaces),
    DrawParams(DrawParams),
    Fonts(Fonts),
    MultiMedias(MultiMedias),
    CompositeGraphicUnits(CompositeGraphicUnits),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ColorSpaces {
    #[serde(rename = "ColorSpace")]
    pub color_spaces: Vec<ColorSpace>,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
#[allow(clippy::upper_case_acronyms)]
pub enum Type {
    RGB,
    GRAY,
    CMYK,
}
impl Type {
    pub fn channel_count(&self) -> usize {
        match self {
            Type::RGB => 3,
            Type::GRAY => 1,
            Type::CMYK => 4,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ColorSpace {
    #[serde(rename = "@ID")]
    pub id: StId,

    #[serde(rename = "@Type")]
    pub r#type: Type,

    /// default 8
    #[serde(rename = "@BitsPerComponent")]
    pub bits_per_component: Option<u8>,

    #[serde(rename = "@Profile")]
    pub profile: Option<StLoc>,

    #[serde(rename = "Palette")]
    pub palette: Option<Palette>,
}

pub static SRGB: ColorSpace = ColorSpace {
    id: 0,
    r#type: Type::RGB,
    bits_per_component: Some(8),
    profile: None,
    palette: None,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct DrawParams {
    #[serde(rename = "DrawParam")]
    pub draw_params: Vec<DrawParam>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DrawParam {
    #[serde(rename = "@ID")]
    pub id: StId,

    #[serde(rename = "@Relative")]
    pub relative: Option<StRefId>,

    /// default 0.353
    #[serde(rename = "@LineWidth")]
    pub line_width: Option<f32>,

    /// default Miter
    #[serde(rename = "@Join")]
    pub join: Option<Join>,

    /// default Butt
    #[serde(rename = "@Cap")]
    pub cap: Option<Cap>,

    /// default 0
    #[serde(rename = "@DashOffset")]
    pub dash_offset: Option<f32>,

    #[serde(rename = "@DashPattern")]
    pub dash_pattern: Option<StArray<f32>>,

    /// default 4.234
    #[serde(rename = "@MiterLimit")]
    pub miter_limit: Option<f32>,

    #[serde(rename = "FillColor")]
    pub fill_color: Option<CtColor>,

    #[serde(rename = "StrokeColor")]
    pub stroke_color: Option<CtColor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fonts {
    #[serde(rename = "Font")]
    pub fonts: Vec<Font>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Font {
    #[serde(rename = "@ID")]
    pub id: StId,

    #[serde(rename = "@FontName")]
    pub font_name: String,

    #[serde(rename = "@FamilyName")]
    pub family_name: Option<String>,

    /// default unicode
    #[serde(rename = "@Charset")]
    pub charset: Option<String>,

    /// default false
    #[serde(rename = "@Italic")]
    pub italic: Option<bool>,

    /// default false
    #[serde(rename = "@Bold")]
    pub bold: Option<bool>,

    /// default false
    #[serde(rename = "@Serif")]
    pub serif: Option<bool>,

    /// default false
    #[serde(rename = "@FixedWidth")]
    pub fixed_width: Option<bool>,

    #[serde(rename = "FontFile")]
    pub font_file: Option<StLoc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiMedias {
    #[serde(rename = "MultiMedia")]
    pub multi_medias: Vec<MultiMedia>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiMedia {
    #[serde(rename = "@ID")]
    pub id: StId,

    #[serde(rename = "@Type")]
    pub r#type: MultiMediaType,

    #[serde(rename = "@Format")]
    pub format: Option<String>,

    #[serde(rename = "MediaFile")]
    pub media_file: StLoc,
}

// #[strum]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, EnumString)]
pub enum MultiMediaType {
    Video,
    Audio,
    Image,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeGraphicUnits {
    #[serde(rename = "CompositeGraphicUnit")]
    pub composite_graphic_units: Vec<CompositeGraphicUnit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeGraphicUnit {
    #[serde(rename = "@ID")]
    pub id: StId,

    /// this prop is using for minidom
    pub base: CtVectorG,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CtVectorG {
    #[serde(rename = "@Width")]
    pub width: f32,
    #[serde(rename = "@Height")]
    pub height: f32,
    #[serde(rename = "Thumbnail")]
    pub thumbnail: Option<StRefId>,
    #[serde(rename = "Substitution")]
    pub substitution: Option<StRefId>,
    #[serde(rename = "Content")]
    pub content: VtGraphicUnit,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use eyre::Result;

    use crate::file::res::ResourceXmlFile;

    #[test]
    fn test_pub_res_de() -> Result<()> {
        let path = "../samples/000/Doc_0/PublicRes.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: ResourceXmlFile = quick_xml::de::from_reader(reader)?;
        dbg!(xml);
        Ok(())
    }
    #[test]
    fn test_doc_res_de() -> Result<()> {
        let path = "../samples/000/Doc_0/DocumentRes.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: ResourceXmlFile = quick_xml::de::from_reader(reader)?;
        dbg!(xml);
        Ok(())
    }
}
