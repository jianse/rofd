use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::base::StLoc;

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentsXmlFile {
    #[serde(rename = "Attachment")]
    pub attachments: Option<Vec<Attachment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename = "@ID")]
    pub id: String,

    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "@Format")]
    pub format: Option<String>,

    #[serde(rename = "@CreationDate")]
    pub creation_date: Option<NaiveDateTime>,

    #[serde(rename = "@ModDate")]
    pub mod_date: Option<NaiveDateTime>,

    #[serde(rename = "@Size")]
    pub size: Option<f32>,

    /// default `true`
    #[serde(rename = "@Visible")]
    pub visible: Option<bool>,

    /// default `none`
    #[serde(rename = "@Usage")]
    pub usage: Option<String>,

    #[serde(rename = "FileLoc")]
    pub file_loc: StLoc,
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;
    use eyre::Result;

    #[test]
    fn test_works() -> Result<()> {
        let file = File::open("../samples/000/Doc_0/Attachs/Attachments.xml")?;
        let reader = std::io::BufReader::new(file);
        let xml = quick_xml::de::from_reader::<_, AttachmentsXmlFile>(reader)?;
        dbg!(xml);
        Ok(())
    }
}
