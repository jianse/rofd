use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::base::StLoc;

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionXmlFile {
    #[serde(rename = "@ID")]
    pub id: String,

    #[serde(rename = "@Version")]
    pub version: Option<String>,

    #[serde(rename = "@Name")]
    pub name: Option<String>,

    #[serde(rename = "@CreationDate")]
    pub creation_date: Option<NaiveDate>,

    #[serde(rename = "FileList")]
    pub file_list: FileList,

    #[serde(rename = "DocRoot")]
    pub doc_root: StLoc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileList {
    #[serde(rename = "File")]
    pub files: Vec<File>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "$value")]
    pub path: StLoc,
}
#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
    use eyre::Result;

    /// this test resources file is inferred from xsd
    /// maybe not correct yet
    #[test]
    fn test_works() -> Result<()> {
        let file = std::fs::File::open("../samples/Version.xml")?;
        let reader = BufReader::new(file);
        let xml = quick_xml::de::from_reader::<_, VersionXmlFile>(reader)?;
        dbg!(xml);
        Ok(())
    }
}
