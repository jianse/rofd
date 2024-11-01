use serde::{Deserialize, Serialize};

use crate::base::StLoc;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomTagsXmlFile {
    #[serde(rename = "CustomTag")]
    pub custom_tags: Option<Vec<CustomTag>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomTag {
    #[serde(rename = "@TypeID")]
    pub type_id: String,

    /// this attr appears in the xsd definition
    /// but not in the actual spec doc
    /// so make it optional
    #[serde(rename = "@NameSpace")]
    pub namespace: Option<String>,

    #[serde(rename = "SchemaLoc")]
    pub schema_loc: Option<StLoc>,

    #[serde(rename = "FileLoc")]
    pub file_loc: StLoc,
}
