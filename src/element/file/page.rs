use super::document::CtPageArea;
use crate::element::base::{StArray, StBox, StId, StLoc, StRefId};
use crate::element::common::{Actions, CtColor};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Debug, Serialize, Deserialize)]
pub struct PageXmlFile {
    /// size
    #[serde(rename = "Area")]
    area: Option<CtPageArea>,

    #[serde(rename = "Template")]
    template: Option<Vec<Template>>,

    #[serde(rename = "PageRes")]
    page_res: Option<Vec<StLoc>>,

    #[serde(rename = "Content")]
    content: Option<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
    #[serde(rename = "@Type")]
    r#type: Option<String>,

    #[serde(rename = "@DrawParam")]
    draw_param: Option<StRefId>,

    #[serde(rename = "@ID")]
    id: StId,

    #[serde(rename = "$value")]
    objects: Option<Vec<CtPageBlock>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "Layer")]
    layer: Vec<Layer>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub enum CtPageBlock {
    TextObject {
        #[serde(rename = "@Font")]
        font: StRefId,

        #[serde(rename = "@Size")]
        size: f64,

        #[serde(rename = "@Stroke")]
        stroke: Option<bool>,

        fill: Option<bool>,

        h_scale: Option<f64>,

        read_direction: Option<u32>,

        char_direction: Option<u32>,

        weight: Option<u32>,

        italic: Option<bool>,

        #[serde(rename = "FillColor")]
        fill_color: Option<CtColor>,

        #[serde(rename = "StrokeColor")]
        stroke_color: Option<CtColor>,

        #[serde(rename = "TextCode")]
        text_codes: Vec<TextCode>,

        // #[serde(rename = "@FontName")]
        // font_name: String,

        // #[serde(rename = "@FamilyName")]
        // family_name: Option<String>,

        // #[serde(rename = "@Charset")]
        // charset: Option<String>,

        // #[serde(rename = "@Italic")]
        // italic: Option<bool>,

        // #[serde(rename = "@Bold")]
        // bold: Option<bool>,

        // #[serde(rename = "@Serif")]
        // serif: Option<bool>,

        // #[serde(rename = "@FixedWidth")]
        // fixed_width: Option<bool>,

        // #[serde(rename = "FontFile")]
        // font_file: Option<StLoc>,

        // region:common fields

        // common fileds on graphic unit
        #[serde(rename = "@Boundary")]
        boundary: StBox,
        #[serde(rename = "@Name")]
        name: Option<String>,
        #[serde(rename = "@Visible")]
        visible: Option<bool>,
        #[serde(rename = "@CTM")]
        ctm: Option<StArray<f64>>,
        #[serde(rename = "@DrawParam")]
        draw_param: Option<StRefId>,
        #[serde(rename = "@LineWidth")]
        line_width: Option<f64>,
        #[serde(rename = "@Cap")]
        cap: Option<String>,
        #[serde(rename = "@Join")]
        join: Option<String>,
        #[serde(rename = "@MiterLimit")]
        miter_limit: Option<f64>,
        #[serde(rename = "@DashOffset")]
        dash_offset: Option<f64>,
        #[serde(rename = "@DashPattern")]
        dash_pattern: Option<StArray<f64>>,
        #[serde(rename = "@Alapha")]
        alapha: Option<u8>,
        #[serde(rename = "Actions")]
        actions: Option<Actions>,
        // endregion
    },

    PathObject {
        #[serde(rename = "@Stroke")]
        stroke: Option<bool>,
        #[serde(rename = "@Fill")]
        fill: Option<bool>,
        #[serde(rename = "@Rule")]
        rule: Option<String>,
        #[serde(rename = "FillColor")]
        fill_color: Option<CtColor>,

        #[serde(rename = "StrokeColor")]
        stroke_color: Option<CtColor>,

        #[serde_as(as = "serde_with::DisplayFromStr")]
        #[serde(rename = "AbbreviatedData")]
        abbreviated_data: StArray<String>,

        // common fileds on graphic unit
        #[serde(rename = "@Boundary")]
        boundary: StBox,
        #[serde(rename = "@Name")]
        name: Option<String>,
        #[serde(rename = "@Visible")]
        visible: Option<bool>,
        #[serde(rename = "@CTM")]
        ctm: Option<StArray<f64>>,
        #[serde(rename = "@DrawParam")]
        draw_param: Option<StRefId>,
        #[serde(rename = "@LineWidth")]
        line_width: Option<f64>,
        #[serde(rename = "@Cap")]
        cap: Option<String>,
        #[serde(rename = "@Join")]
        join: Option<String>,
        #[serde(rename = "@MiterLimit")]
        miter_limit: Option<f64>,
        #[serde(rename = "@DashOffset")]
        dash_offset: Option<f64>,
        #[serde(rename = "@DashPattern")]
        dash_pattern: Option<StArray<f64>>,
        #[serde(rename = "@Alapha")]
        alapha: Option<u8>,
        #[serde(rename = "Actions")]
        actions: Option<Actions>,
        // endregion
    },
    ImageObject {},
    CompositeObject {},
    PageBlock {},
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TextCode {
    #[serde(rename = "@X")]
    x: Option<f64>,
    #[serde(rename = "@Y")]
    y: Option<f64>,
    #[serde(rename = "@DeltaX")]
    delta_x: Option<StArray<String>>,
    #[serde(rename = "@DeltaY")]
    delta_y: Option<StArray<String>>,
    #[serde(rename = "$value")]
    val: String,
}

// ct_graphic_unit!(
//     #[derive(Debug, Deserialize, Serialize)]
//     pub struct TextObject {}
// );

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "@TemplateID")]
    template_id: StRefId,

    #[serde(rename = "@ZOrder")]
    z_order: Option<String>,
}

#[cfg(test)]
mod test_page_file {
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
