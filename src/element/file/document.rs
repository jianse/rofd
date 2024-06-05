use serde::{Deserialize, Serialize};

use crate::element::base::{StBox, StId, StLoc, StRefId};

/// Document.xml 文件
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentXmlFile {
    #[serde(rename = "CommonData")]
    pub common_data: CommonData,

    #[serde(rename = "Pages")]
    pub pages: Pages,

    // TODO missing some props
    #[serde(rename = "Annotations")]
    pub annotations: Option<StLoc>,

    #[serde(rename = "Attachments")]
    pub attachments: Option<StLoc>,

    #[serde(rename = "CustomTags")]
    pub custom_tags: Option<StLoc>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonData {
    #[serde(rename = "MaxUnitID")]
    max_unit_id: StId,

    #[serde(rename = "PageArea")]
    page_area: CtPageArea,

    #[serde(rename = "PublicRes")]
    public_res: Option<Vec<StLoc>>,

    #[serde(rename = "DocumentRes")]
    document_res: Option<Vec<StLoc>>,

    #[serde(rename = "TemplatePage")]
    pub template_page: Option<Vec<TemplatePage>>,

    /// 可选属性 默认值为sRGB
    #[serde(rename = "DefaultCS")]
    default_cs: Option<StRefId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplatePage {
    #[serde(rename = "@ID")]
    pub id: StId,

    #[serde(rename = "@Name")]
    name: Option<String>,

    #[serde(rename = "@ZOrder")]
    z_order: Option<String>,

    #[serde(rename = "@BaseLoc")]
    pub base_loc: StLoc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pages {
    #[serde(rename = "Page")]
    pub page: Vec<Page>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    #[serde(rename = "@ID")]
    id: StId,
    #[serde(rename = "@BaseLoc")]
    pub base_loc: StLoc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CtPageArea {
    #[serde(rename = "PhysicalBox")]
    pub physical_box: StBox,

    #[serde(rename = "ApplicationBox")]
    pub application_box: Option<StBox>,

    #[serde(rename = "ContentBox")]
    pub content_box: Option<StBox>,

    #[serde(rename = "BleedBox")]
    pub bleed_box: Option<StBox>,
}

#[cfg(test)]
mod tests {
    use eyre::Result;
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn test_de() -> Result<()> {
        let path = "sample/Doc_0/Document.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: DocumentXmlFile = quick_xml::de::from_reader(reader)?;
        dbg!(xml);
        Ok(())
    }
}
