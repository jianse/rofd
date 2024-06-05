use ::serde::{Deserialize, Serialize};
use chrono::NaiveDate;
// use serde::{Deserialize, Serialize};

use crate::element::base::StLoc;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "OFD")]
pub struct OfdXmlFile {
    #[serde(rename = "@Version")]
    pub version: String,
    #[serde(rename = "@DocType")]
    pub doc_type: String,
    #[serde(rename = "DocBody")]
    pub doc_body: Vec<DocBody>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocBody {
    #[serde(rename = "DocInfo")]
    pub doc_info: CtDocInfo,

    #[serde(rename = "DocRoot")]
    pub doc_root: Option<StLoc>,

    // this prop type is unsure
    #[serde(rename = "Versions")]
    pub versions: Option<String>,
    #[serde(rename = "Signatures")]
    pub signatures: Option<StLoc>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CtDocInfo {
    // TODO missing some prop
    #[serde(rename = "DocID")]
    pub doc_id: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Author")]
    pub author: Option<String>,
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<NaiveDate>,
    #[serde(rename = "CustomDatas")]
    pub custom_datas: Option<CustomDatas>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomDatas {
    #[serde(rename = "CustomData")]
    pub custom_data: Vec<CustomData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomData {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "$value")]
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
        let path = "sample/OFD.xml";
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let xml: OfdXmlFile = quick_xml::de::from_reader(reader)?;
        dbg!(xml);
        Ok(())
    }

    #[test]
    fn test_write_ofd_xml() -> Result<()> {
        let out_path = "output/OFD.xml";
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
                    creation_date: NaiveDate::from_ymd_opt(2023, 12, 19),
                    custom_datas: None,
                },
                doc_root: Some("Doc_0".into()),
                versions: None,
                signatures: Some("abc".into()),
            }],
        }
    }
}
