use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::base::{StLoc, StRefId};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionXmlFile {
    pub extensions: Option<Vec<Extension>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extension {
    pub app_name: String,
    pub company: Option<String>,
    pub app_version: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub ref_id: StRefId,
    pub props: Vec<Prop>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Prop {
    Property {
        name: String,
        r#type: Option<String>,
        value: String,
    },
    Data(
        // TODO: ANY_TYPE
    ),
    ExtendData(StLoc),
}
