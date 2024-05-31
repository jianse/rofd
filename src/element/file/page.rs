use super::document::CtPageArea;
use crate::ct_layer;
use crate::element::base::{StId, StLoc, StRefId};
use crate::element::common::CtGraphicUnit;
use serde::{Deserialize, Serialize};

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
pub struct Content {
    #[serde(rename = "Layer")]
    layer: Vec<Layer>,
}

ct_layer!(
    #[derive(Debug, Serialize, Deserialize)]
    pub  struct  Layer {
    // ct_layer!();
    // #[serde(flatten)]
    // base: CtLayer,
    // #[serde(rename = "@Type")]
    // r#type: Option<String>,

    // #[serde(rename = "@DrawParam")]
    // draw_param: Option<StRefId>,
    #[serde(rename = "@ID")]
    id: StId,

    #[serde(rename = "$value")]
    objects: Option<Vec<CtPageBlock>>,
});

#[derive(Debug, Serialize, Deserialize)]
pub enum CtPageBlock {
    TextObject {
        #[serde(flatten)]
        base: CtGraphicUnit
    },
    PathObject {},
    ImageObject {},
    CompositeObject {},
    PageBlock {},
}

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
}
