use serde::{Deserialize, Serialize};

use crate::element::{
    base::{StArray, StId, StLoc, StRefId},
    common::{CtColor, Palette},
};

use super::page::CtPageBlock;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceXmlFile {
    #[serde(rename = "@BaseLoc")]
    pub base_loc: StLoc,

    #[serde(rename = "ColorSpaces")]
    pub color_spaces: Option<ColorSpaces>,

    #[serde(rename = "DrawParams")]
    pub draw_params: Option<DrawParams>,

    #[serde(rename = "Fonts")]
    pub fonts: Option<Fonts>,

    #[serde(rename = "MultiMedias")]
    pub multi_medias: Option<MultiMedias>,

    #[serde(rename = "CompositeGraphicUnits")]
    pub composite_graphic_units: Option<CompositeGraphicUnits>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorSpaces {
    #[serde(rename = "ColorSpace")]
    pub color_spaces: Vec<ColorSpace>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
    pub join: Option<String>,

    /// default Butt
    #[serde(rename = "@Cap")]
    pub cap: Option<String>,

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
#[derive(Debug, Serialize, Deserialize)]
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
    pub r#type: String,

    #[serde(rename = "@Format")]
    pub format: Option<String>,

    #[serde(rename = "MediaFile")]
    pub media_file: StLoc,
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
    //TODO: P79
    #[serde(rename = "Content")]
    pub content: CtPageBlock,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use eyre::Result;

    use crate::element::file::res::ResourceXmlFile;

    #[test]
    fn test_pubres_de() -> Result<()> {
        let path = "sample/Doc_0/PublicRes.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: ResourceXmlFile = quick_xml::de::from_reader(reader)?;
        dbg!(xml);
        Ok(())
    }
    #[test]
    fn test_docres_de() -> Result<()> {
        let path = "sample/Doc_0/DocumentRes.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: ResourceXmlFile = quick_xml::de::from_reader(reader)?;
        dbg!(xml);
        Ok(())
    }
}
