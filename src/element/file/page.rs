use super::document::CtPageArea;
use crate::element::base::{StArray, StBox, StId, StLoc, StRefId};
use crate::element::common::{Actions, Cap, CtColor, Join};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::TryFromInto;
use strum::EnumString;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct PageXmlFile {
    /// size
    #[serde(rename = "Area")]
    pub area: Option<CtPageArea>,

    #[serde(rename = "Template")]
    pub template: Option<Vec<Template>>,

    #[serde(rename = "PageRes")]
    pub page_res: Option<Vec<StLoc>>,

    #[serde(rename = "Content")]
    pub content: Option<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
    #[serde(rename = "@Type")]
    pub r#type: Option<String>,

    #[serde(rename = "@DrawParam")]
    pub draw_param: Option<StRefId>,

    #[serde(rename = "@ID")]
    pub id: StId,

    #[serde(rename = "$value")]
    pub objects: Option<Vec<CtPageBlock>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "Layer")]
    pub layer: Vec<Layer>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CtPageBlock {
    TextObject(TextObject),
    PathObject(PathObject),
    ImageObject(ImageObject),
    CompositeObject {},
    PageBlock {},
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageObject {
    #[serde(rename = "@ResourceID")]
    pub resource_id: StRefId,

    #[serde(rename = "@Substitution")]
    pub substitution: Option<StRefId>,

    #[serde(rename = "@ImageMask")]
    pub image_mask: Option<StRefId>,

    #[serde(rename = "Border")]
    pub border: Option<Border>,

    // region:common fields

    // common fields on graphic unit
    #[serde(rename = "@Boundary")]
    pub boundary: StBox,
    #[serde(rename = "@Name")]
    pub name: Option<String>,
    #[serde(rename = "@Visible")]
    pub visible: Option<bool>,
    #[serde(rename = "@CTM")]
    pub ctm: Option<StArray<f32>>,
    #[serde(rename = "@DrawParam")]
    pub draw_param: Option<StRefId>,
    #[serde(rename = "@LineWidth")]
    pub line_width: Option<f32>,
    #[serde(rename = "@Cap")]
    pub cap: Option<Cap>,
    #[serde(rename = "@Join")]
    pub join: Option<Join>,
    #[serde(rename = "@MiterLimit")]
    pub miter_limit: Option<f32>,
    #[serde(rename = "@DashOffset")]
    pub dash_offset: Option<f32>,
    #[serde(rename = "@DashPattern")]
    pub dash_pattern: Option<StArray<f32>>,
    #[serde(rename = "@Alpha")]
    pub alpha: Option<u8>,
    #[serde(rename = "Actions")]
    pub actions: Option<Actions>,
    // endregion
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Border {
    /// default 0.353 mm
    #[serde(rename = "@LineWidth")]
    pub line_width: f32,

    /// default 0
    #[serde(rename = "@HorizontalCornerRadius")]
    pub horizontal_corner_radius: Option<f32>,

    /// default 0
    #[serde(rename = "@VerticalCornerRadius")]
    pub vertical_corner_radius: Option<f32>,

    #[serde(rename = "@DashOffset")]
    pub dash_offset: Option<f32>,

    #[serde(rename = "@DashPattern")]
    pub dash_pattern: Option<StArray<f32>>,

    #[serde(rename = "BorderColor")]
    pub border_color: Option<CtColor>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextObject {
    #[serde(rename = "@Font")]
    pub font: StRefId,

    #[serde(rename = "@Size")]
    pub size: f32,

    #[serde(rename = "@Stroke")]
    pub stroke: Option<bool>,

    #[serde(rename = "@Fill")]
    pub fill: Option<bool>,

    #[serde(rename = "@HScale")]
    pub h_scale: Option<f32>,

    #[serde(rename = "@ReadDirection")]
    pub read_direction: Option<u32>,

    #[serde(rename = "@CharDirection")]
    pub char_direction: Option<u32>,

    /// default 400
    #[serde(rename = "@Weight")]
    pub weight: Option<u32>,

    #[serde(rename = "@Italic")]
    pub italic: Option<bool>,

    #[serde(rename = "FillColor")]
    pub fill_color: Option<CtColor>,

    #[serde(rename = "StrokeColor")]
    pub stroke_color: Option<CtColor>,

    #[serde(rename = "$value")]
    #[serde_as(as = "TryFromInto<UnifiedTextValVec>")]
    pub text_vals: Vec<TextVal>,

    // pub text_v
    // region:common fields

    // common fields on graphic unit
    #[serde(rename = "@Boundary")]
    pub boundary: StBox,
    #[serde(rename = "@Name")]
    pub name: Option<String>,
    #[serde(rename = "@Visible")]
    pub visible: Option<bool>,
    #[serde(rename = "@CTM")]
    pub ctm: Option<StArray<f32>>,
    #[serde(rename = "@DrawParam")]
    pub draw_param: Option<StRefId>,
    #[serde(rename = "@LineWidth")]
    pub line_width: Option<f32>,
    #[serde(rename = "@Cap")]
    pub cap: Option<Cap>,
    #[serde(rename = "@Join")]
    pub join: Option<Join>,
    #[serde(rename = "@MiterLimit")]
    pub miter_limit: Option<f32>,
    #[serde(rename = "@DashOffset")]
    pub dash_offset: Option<f32>,
    #[serde(rename = "@DashPattern")]
    pub dash_pattern: Option<StArray<f32>>,
    #[serde(rename = "@Alpha")]
    pub alpha: Option<u8>,
    #[serde(rename = "Actions")]
    pub actions: Option<Actions>,
    // endregion
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UnifiedText {
    CGTransform(CGTransform),
    TextCode(TextCode),
}

#[derive(Error, Debug)]
pub enum TryIntoTextValError {
    #[error("CGTransform more than one")]
    CgtNotValid,
}

#[derive(Debug, Serialize, Deserialize)]
struct UnifiedTextValVec(Vec<UnifiedText>);
impl TryInto<Vec<TextVal>> for UnifiedTextValVec {
    type Error = TryIntoTextValError;

    fn try_into(self) -> Result<Vec<TextVal>, Self::Error> {
        let mut iter = self.0.into_iter();
        let mut cur_c = None;
        let mut res = vec![];
        for v in iter.by_ref() {
            match v {
                UnifiedText::CGTransform(c) => {
                    if cur_c.is_some() {
                        return Err(TryIntoTextValError::CgtNotValid);
                    } else {
                        cur_c = Some(c)
                    }
                }
                UnifiedText::TextCode(t) => res.push(TextVal {
                    cg_transform: cur_c.take(),
                    text_code: t,
                }),
            }
        }
        Ok(res)
    }
}
impl From<Vec<TextVal>> for UnifiedTextValVec {
    // type Error = TryIntoTextValError;
    //
    // fn try_from(value: Vec<TextVal>) -> Result<Self, Self::Error> {
    //     value.iter().map(UnifiedText::try_into).collect();
    //     // todo!()
    // }

    fn from(value: Vec<TextVal>) -> Self {
        let v0 = value
            .into_iter()
            .map(|v| {
                if v.cg_transform.is_some() {
                    vec![
                        UnifiedText::CGTransform(v.cg_transform.unwrap()),
                        UnifiedText::TextCode(v.text_code),
                    ]
                } else {
                    vec![UnifiedText::CGTransform(v.cg_transform.unwrap())]
                }
            })
            .flat_map(|v| v.into_iter())
            .collect::<Vec<UnifiedText>>();
        UnifiedTextValVec(v0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextVal {
    #[serde(rename = "CGTransform")]
    pub cg_transform: Option<CGTransform>,

    #[serde(rename = "TextCode")]
    pub text_code: TextCode,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CGTransform {
    #[serde(rename = "@CodePosition")]
    pub code_position: u32,

    /// default 1
    #[serde(rename = "@CodeCount")]
    pub code_count: Option<u32>,

    /// default 1
    #[serde(rename = "@GlyphCount")]
    pub glyph_count: Option<u32>,

    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[serde(rename = "Glyphs")]
    pub glyphs: StArray<u32>,
}

#[derive(Debug, Serialize, Deserialize, EnumString, Clone)]
pub enum FillRule {
    NoneZero,
    // #[serde]
    EvenOdd,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PathObject {
    /// default true
    #[serde(rename = "@Stroke")]
    pub stroke: Option<bool>,

    /// default false
    #[serde(rename = "@Fill")]
    pub fill: Option<bool>,

    /// default NoneZero
    #[serde(rename = "@Rule")]
    pub rule: Option<FillRule>,

    /// default transparent
    #[serde(rename = "FillColor")]
    pub fill_color: Option<CtColor>,

    /// default black
    #[serde(rename = "StrokeColor")]
    pub stroke_color: Option<CtColor>,

    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[serde(rename = "AbbreviatedData")]
    pub abbreviated_data: StArray<String>,

    // common fields on graphic unit
    #[serde(rename = "@Boundary")]
    pub boundary: StBox,
    #[serde(rename = "@Name")]
    pub name: Option<String>,
    #[serde(rename = "@Visible")]
    pub visible: Option<bool>,
    #[serde(rename = "@CTM")]
    pub ctm: Option<StArray<f32>>,
    #[serde(rename = "@DrawParam")]
    pub draw_param: Option<StRefId>,
    #[serde(rename = "@LineWidth")]
    pub line_width: Option<f32>,

    /// default Butt
    #[serde(rename = "@Cap")]
    pub cap: Option<Cap>,

    /// default Miter
    #[serde(rename = "@Join")]
    pub join: Option<Join>,

    /// default 3.528
    #[serde(rename = "@MiterLimit")]
    pub miter_limit: Option<f32>,
    #[serde(rename = "@DashOffset")]
    pub dash_offset: Option<f32>,
    #[serde(rename = "@DashPattern")]
    pub dash_pattern: Option<StArray<f32>>,
    #[serde(rename = "@Alpha")]
    pub alpha: Option<u8>,
    #[serde(rename = "Actions")]
    pub actions: Option<Actions>,
    // endregion
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TextCode {
    #[serde(rename = "@X")]
    pub x: Option<f32>,
    #[serde(rename = "@Y")]
    pub y: Option<f32>,
    #[serde(rename = "@DeltaX")]
    pub delta_x: Option<StArray<String>>,
    #[serde(rename = "@DeltaY")]
    pub delta_y: Option<StArray<String>>,
    #[serde(rename = "$text", default = "empty_string")]
    pub val: String,
}

fn empty_string() -> String {
    "".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "@TemplateID")]
    pub template_id: StRefId,

    #[serde(rename = "@ZOrder")]
    pub z_order: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use eyre::Result;
    use serde_with::serde_as;

    use super::*;

    #[test]
    fn test_page_file() -> Result<()> {
        let path = "samples/sample/Doc_0/Pages/Page_0/Content.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: PageXmlFile = quick_xml::de::from_reader(reader)?;
        dbg!(xml);
        Ok(())
    }
    #[test]
    fn test_tpl_file() -> Result<()> {
        let path = "samples/sample/Doc_0/Tpls/Tpl_0/Content.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: PageXmlFile = quick_xml::de::from_reader(reader)?;
        dbg!(xml);
        Ok(())
    }

    #[test]
    fn test_text_val() -> Result<()> {
        let path = "samples/ano/Doc_0/Pages/Page_0/Content.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: PageXmlFile = quick_xml::de::from_reader(reader)?;
        if let Some(content) = xml.content {
            for layer in content.layer {
                if let Some(objs) = layer.objects {
                    for obj in objs {
                        match obj {
                            CtPageBlock::TextObject(to) => {
                                // dbg!(&to.text_vals.);
                                for tv in to.text_vals {
                                    if let Some(cgt) = tv.cg_transform {
                                        dbg!(cgt.glyphs);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_text_object() -> Result<()> {
        let xml_str = r#"
        <ofd:Layer DrawParam="4" ID="11">
        <ofd:PathObject ID="12" CTM="0.45 0 0 0.45 0 0" Boundary="57.5 97.8 5 5" LineWidth="0.5">
          <ofd:AbbreviatedData>M 10.07 5.54 B 10.07 3.04 8.04 1 5.53 1 B 3.03 1 1 3.04 1 5.54 B 1 8.04 3.03 10.08 5.53 10.08 B 8.04 10.08 10.07 8.04 10.07 5.54 M 2.3 2.3 L 8.7 8.7 M 2.3 8.7 L 8.7 2.3 </ofd:AbbreviatedData>
        </ofd:PathObject>
        </ofd:Layer>
        "#;
        let xml = quick_xml::de::from_str::<Layer>(&xml_str)?;
        dbg!(xml);
        Ok(())
    }
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Data {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        #[serde(rename = "$value")]
        val: StArray<u64>,
    }
    #[test]
    fn test_sequence_of_str() -> Result<()> {
        let xml_str = r#"<Data>12 13 15</Data>"#;
        let xml = quick_xml::de::from_str::<Data>(&xml_str)?;
        // dbg!(xml);
        // xml =
        assert_eq!(
            xml,
            Data {
                val: vec![12_u64, 13, 15].into()
            }
        );
        Ok(())
    }

    #[derive(Debug, Deserialize)]
    struct AnyXml {
        #[serde(rename = "FillColor")]
        fill_color: Option<CtColor>,

        #[serde(rename = "$value")]
        text_val: Vec<UnifiedText>,

        #[serde(rename = "Actions")]
        actions: Option<Actions>,
    }
    #[test]
    fn test_unified_text() -> Result<()> {
        let xml_str = r#"<Data><ofd:CGTransform CodePosition="0" CodeCount="1" GlyphCount="1">
                    <ofd:Glyphs>1</ofd:Glyphs>
                </ofd:CGTransform>
                <ofd:TextCode X="0" Y="0"> </ofd:TextCode></Data>"#;
        let xml = quick_xml::de::from_str::<AnyXml>(&xml_str)?;
        dbg!(&xml);
        Ok(())
    }

    #[test]
    fn test_parse_text_object() -> Result<()> {
        let xml_str = r#"<ofd:TextObject ID="112" CTM="0.3528 0 0 0.3528 0 0" Boundary="25.4085 242.817 0.8445 0"
                Font="115" Size="15.96">
                <ofd:CGTransform CodePosition="0" CodeCount="1" GlyphCount="1">
                    <ofd:Glyphs>1</ofd:Glyphs>
                </ofd:CGTransform>
                <ofd:TextCode X="0" Y="0"> </ofd:TextCode>
            </ofd:TextObject>"#;
        let xml = quick_xml::de::from_str::<TextObject>(&xml_str)?;
        dbg!(&xml);
        Ok(())
    }
}
