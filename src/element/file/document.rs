use crate::element::common::CtDest;
use crate::element::{
    base::{StBox, StId, StLoc, StRefId},
    common::Actions,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;

/// Document.xml 文件
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentXmlFile {
    #[serde(rename = "CommonData")]
    pub common_data: CommonData,

    #[serde(rename = "Pages")]
    pub pages: Pages,

    #[serde(rename = "Outlines")]
    pub outlines: Option<Outlines>,

    #[serde(rename = "Permissions")]
    pub permissions: Option<CtPermission>,

    #[serde(rename = "Actions")]
    pub actions: Option<Actions>,

    #[serde(rename = "VPreferences")]
    pub v_preferences: Option<CtVPreferences>,

    #[serde(rename = "Bookmarks")]
    pub bookmarks: Option<Bookmarks>,

    #[serde(rename = "Annotations")]
    pub annotations: Option<StLoc>,

    #[serde(rename = "CustomTags")]
    pub custom_tags: Option<StLoc>,

    #[serde(rename = "Attachments")]
    pub attachments: Option<StLoc>,

    #[serde(rename = "Extensions")]
    pub extensions: Option<StLoc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmarks {
    #[serde(rename = "Bookmark")]
    pub bookmarks: Vec<CtBookmark>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CtBookmark {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Dest")]
    pub dest: CtDest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CtVPreferences {
    /// default None
    #[serde(rename = "PageMode")]
    pub page_mode: Option<String>,

    /// default OneColumn
    #[serde(rename = "PageLayout")]
    pub page_layout: Option<String>,

    /// default DocTitle
    #[serde(rename = "TabDisplay")]
    pub tab_display: Option<String>,

    /// default false
    #[serde(rename = "HideToolbar")]
    pub hide_toolbar: Option<bool>,

    /// default false
    #[serde(rename = "HideMenubar")]
    pub hide_menubar: Option<bool>,

    /// default false
    #[serde(rename = "HideWindowUI")]
    pub hide_window_ui: Option<bool>,

    /// choice
    #[serde(rename = "ZoomMode")]
    pub zoom_mode: Option<String>,

    /// zoom ratio
    #[serde(rename = "Zoom")]
    pub zoom: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CtPermission {
    #[serde(rename = "Edit")]
    pub edit: Option<bool>,
    #[serde(rename = "Annot")]
    pub annot: Option<bool>,
    #[serde(rename = "Export")]
    pub export: Option<bool>,
    #[serde(rename = "Signature")]
    pub signature: Option<bool>,
    #[serde(rename = "Watermark")]
    pub watermark: Option<bool>,
    #[serde(rename = "PrintScreen")]
    pub print_screen: Option<bool>,

    #[serde(rename = "Print")]
    pub print: Option<Print>,
    #[serde(rename = "ValidPeriod")]
    pub valid_period: Option<ValidPeriod>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidPeriod {
    #[serde(rename = "@StartDate")]
    pub start_date: Option<NaiveDateTime>,
    #[serde(rename = "@EndDate")]
    pub end_date: Option<NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Print {
    #[serde(rename = "@Printable")]
    pub printable: bool,
    /// default -1
    #[serde(rename = "@Copies")]
    pub copies: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Outlines {
    #[serde(rename = "OutlineElem")]
    pub outline_elems: Vec<CtOutlineElem>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CtOutlineElem {
    #[serde(rename = "@Title")]
    pub title: String,
    #[serde(rename = "@Count")]
    pub count: Option<u32>,
    #[serde(rename = "@Expanded")]
    pub expanded: Option<bool>,
    #[serde(rename = "Actions")]
    pub actions: Option<Actions>,
    #[serde(rename = "OutlineElem")]
    pub outline_elems: Option<Vec<CtOutlineElem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonData {
    #[serde(rename = "MaxUnitID")]
    pub max_unit_id: StId,

    #[serde(rename = "PageArea")]
    pub page_area: CtPageArea,

    #[serde(rename = "PublicRes")]
    pub public_res: Option<Vec<StLoc>>,

    #[serde(rename = "DocumentRes")]
    pub document_res: Option<Vec<StLoc>>,

    #[serde(rename = "TemplatePage")]
    pub template_page: Option<Vec<TemplatePage>>,

    /// 可选属性 默认值为sRGB
    #[serde(rename = "DefaultCS")]
    pub default_cs: Option<StRefId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplatePage {
    #[serde(rename = "@ID")]
    pub id: StId,

    #[serde(rename = "@Name")]
    pub name: Option<String>,

    #[serde(rename = "@ZOrder")]
    pub z_order: Option<String>,

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
    pub id: StId,
    #[serde(rename = "@BaseLoc")]
    pub base_loc: StLoc,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CtPageArea {
    #[serde_as(as = "DisplayFromStr")]
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
        let path = "samples/sample/Doc_0/Document.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: DocumentXmlFile = quick_xml::de::from_reader(reader)?;
        dbg!(xml);
        Ok(())
    }
}
