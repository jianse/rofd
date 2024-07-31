use super::document::CtPageArea;
use crate::element::base::{StArray, StBox, StId, StLoc, StRefId};
use crate::element::common::{Actions, Cap, CtColor, Join};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

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
    r#type: Option<String>,

    #[serde(rename = "@DrawParam")]
    pub draw_param: Option<StRefId>,

    #[serde(rename = "@ID")]
    id: StId,

    #[serde(rename = "$value")]
    pub objects: Option<Vec<CtPageBlock>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "Layer")]
    pub layer: Vec<Layer>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub enum CtPageBlock {
    TextObject(TextObject),
    PathObject(PathObject),
    ImageObject {},
    CompositeObject {},
    PageBlock {},
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct TextObject {
    #[serde(rename = "@Font")]
    pub font: StRefId,

    #[serde(rename = "@Size")]
    pub size: f32,

    #[serde(rename = "@Stroke")]
    pub stroke: Option<bool>,

    pub fill: Option<bool>,

    pub h_scale: Option<f32>,

    pub read_direction: Option<u32>,

    pub char_direction: Option<u32>,

    pub weight: Option<u32>,

    pub italic: Option<bool>,

    #[serde(rename = "FillColor")]
    pub fill_color: Option<CtColor>,

    #[serde(rename = "StrokeColor")]
    pub stroke_color: Option<CtColor>,

    #[serde(rename = "TextCode")]
    pub text_codes: Vec<TextCode>,

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
    pub cap: Option<String>,
    #[serde(rename = "@Join")]
    pub join: Option<String>,
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
pub enum FillRule {
    NoneZero,
    // #[serde]
    EvenOdd,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct TextCode {
    #[serde(rename = "@X")]
    pub x: Option<f32>,
    #[serde(rename = "@Y")]
    pub y: Option<f32>,
    #[serde(rename = "@DeltaX")]
    pub delta_x: Option<StArray<String>>,
    #[serde(rename = "@DeltaY")]
    pub delta_y: Option<StArray<String>>,
    #[serde(rename = "$value")]
    pub val: String,
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
        let path = "sample/Doc_0/Pages/Page_0/Content.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: PageXmlFile = quick_xml::de::from_reader(reader)?;
        dbg!(xml);
        Ok(())
    }
    #[test]
    fn test_tpl_file() -> Result<()> {
        let path = "sample/Doc_0/Tpls/Tpl_0/Content.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: PageXmlFile = quick_xml::de::from_reader(reader)?;
        dbg!(xml);
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
}
