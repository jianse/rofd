use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::element::base::{StBox, StId, StLoc, StRefId};

use super::page::VtGraphicUnit;

#[derive(Debug, Deserialize, Serialize)]
pub struct AnnotationsXmlFile {
    #[serde(rename = "Page")]
    pub page: Option<Vec<Page>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    #[serde(rename = "@PageID")]
    pub page_id: StRefId,

    #[serde(rename = "FileLoc")]
    pub file_loc: StLoc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnotationXmlFile {
    #[serde(rename = "Annot")]
    pub annot: Vec<Annot>,
}

fn empty_string() -> String {
    "".to_string()
}
fn zero_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Annot {
    #[serde(rename = "@ID")]
    pub id: StId,

    #[serde(rename = "@Type")]
    pub r#type: String,

    #[serde(rename = "@Creator", default = "empty_string")]
    pub creator: String,

    #[serde(rename = "@LastModDate", default = "zero_date")]
    pub last_mod_date: NaiveDate,

    /// default `true`
    #[serde(rename = "@Visible")]
    pub visible: Option<bool>,

    #[serde(rename = "@Subtype")]
    pub subtype: Option<String>,

    /// default `true`
    #[serde(rename = "@Print")]
    pub print: Option<bool>,

    /// default `false`
    #[serde(rename = "@NoZoom")]
    pub no_zoom: Option<bool>,

    /// default `false`
    #[serde(rename = "@NoRotate")]
    pub no_rotate: Option<bool>,

    /// default `true`
    #[serde(rename = "@ReadOnly")]
    pub read_only: Option<bool>,

    #[serde(rename = "Remark")]
    pub remark: Option<String>,

    #[serde(rename = "Parameters")]
    pub parameters: Option<Parameters>,

    #[serde(rename = "Appearance")]
    pub appearance: Appearance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Appearance {
    #[serde(rename = "@Boundary")]
    pub boundary: StBox,
    #[serde(rename = "$value")]
    pub objects: Option<Vec<VtGraphicUnit>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameters {
    #[serde(rename = "Parameter")]
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "$value")]
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;
    use std::{fs::File, io::BufReader};

    #[test]
    fn annotations_xml_file_works() -> Result<()> {
        let file = File::open("samples/sample/Doc_0/Annots/Annotations.xml")?;
        let reader = BufReader::new(file);
        let xml = quick_xml::de::from_reader::<_, AnnotationsXmlFile>(reader)?;
        dbg!(xml);
        Ok(())
    }

    #[test]
    fn annotation_xml_file_works() -> Result<()> {
        let file = File::open("samples/sample/Doc_0/Annots/Page_0/Annotation.xml")?;
        let reader = BufReader::new(file);
        let xml = quick_xml::de::from_reader::<_, AnnotationXmlFile>(reader)?;
        dbg!(xml);
        Ok(())
    }
}
