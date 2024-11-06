//! structs for OFD.xml
//!
//! [OfdXmlFile] main entry

use crate::base::StLoc;
use ::serde::{Deserialize, Serialize};
use chrono::NaiveDate;

/// main entry for an ofd file
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "OFD")]
pub struct OfdXmlFile {
    #[serde(rename = "@Version")]
    pub version: String,
    #[serde(rename = "@DocType")]
    pub doc_type: String,
    #[serde(rename = "DocBody")]
    pub doc_body: Vec<DocBody>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct DocBody {
    #[serde(rename = "DocInfo")]
    pub doc_info: CtDocInfo,

    /// this field is optional by spec,
    /// but it is required by the xsd
    #[serde(rename = "DocRoot")]
    pub doc_root: Option<StLoc>,

    /// this prop type is unsure
    /// **see xsd**
    #[serde(rename = "Versions")]
    pub versions: Option<Versions>,

    #[serde(rename = "Signatures")]
    pub signatures: Option<StLoc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Versions {
    #[serde(rename = "Version")]
    pub version: Vec<Version>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Version {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Index")]
    pub index: i32,
    /// default `false`
    #[serde(rename = "@Current")]
    pub current: Option<bool>,

    #[serde(rename = "BaseLoc")]
    pub base_loc: StLoc,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CtDocInfo {
    #[serde(rename = "DocID")]
    pub doc_id: Option<String>,

    #[serde(rename = "Title")]
    pub title: Option<String>,

    #[serde(rename = "Author")]
    pub author: Option<String>,

    #[serde(rename = "Subject")]
    pub subject: Option<String>,

    #[serde(rename = "Abstract")]
    pub r#abstract: Option<String>,

    #[serde(rename = "CreationDate")]
    pub creation_date: Option<NaiveDate>,

    #[serde(rename = "ModDate")]
    pub mod_date: Option<NaiveDate>,

    #[serde(rename = "DocUsage")]
    pub doc_usage: Option<String>,

    #[serde(rename = "Cover")]
    pub cover: Option<StLoc>,

    #[serde(rename = "Keywords")]
    pub keywords: Option<Keywords>,

    #[serde(rename = "Creator")]
    pub creator: Option<String>,

    #[serde(rename = "CreatorVersion")]
    pub creator_version: Option<String>,

    #[serde(rename = "CustomDatas")]
    pub custom_datas: Option<CustomDatas>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Keywords {
    #[serde(rename = "Keyword")]
    pub keywords: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomDatas {
    #[serde(rename = "CustomData")]
    pub custom_data: Vec<CustomData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomData {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub value: String,
}
#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, Write},
        vec,
    };

    use eyre::Result;

    use super::*;

    #[test]
    fn test_parse_ofd_xml() -> Result<()> {
        let path = "../samples/sample/OFD.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: OfdXmlFile = quick_xml::de::from_reader(reader)?;
        dbg!(xml);
        Ok(())
    }

    #[test]
    fn test_write_ofd_xml() -> Result<()> {
        let out_path = "../output/OFD.xml";
        let mut file = File::create(out_path)?;
        let value = new_ofd();
        let buffer = quick_xml::se::to_string(&value)?;
        write!(file, "{}", buffer)?;
        Ok(())
    }

    fn new_ofd() -> OfdXmlFile {
        OfdXmlFile {
            version: "1.1".into(),
            doc_type: "OFD".into(),
            doc_body: vec![DocBody {
                doc_info: CtDocInfo {
                    doc_id: Some("str".into()),
                    title: None,
                    author: Some("China Tex".into()),
                    subject: None,
                    r#abstract: None,
                    creation_date: NaiveDate::from_ymd_opt(2023, 12, 19),
                    mod_date: None,
                    doc_usage: None,
                    cover: None,
                    keywords: Some(Keywords {
                        keywords: ["a", "b", "c", "d"].iter().map(|k| k.to_string()).collect(),
                    }),
                    creator: None,
                    creator_version: None,
                    custom_datas: None,
                },
                doc_root: Some("Doc_0".into()),
                versions: None,
                signatures: Some("abc".into()),
            }],
        }
    }
}
