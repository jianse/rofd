use serde::{Deserialize, Serialize};

use crate::base::{StBox, StLoc, StRefId};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignaturesXmlFile {
    #[serde(rename = "MaxSignId")]
    pub max_sign_id: Option<String>,

    #[serde(rename = "Signature")]
    pub signature: Option<Vec<Signature>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature {
    #[serde(rename = "@ID")]
    pub id: String,

    /// default `Seal`
    #[serde(rename = "@Type")]
    pub r#type: Option<String>,

    #[serde(rename = "@BaseLoc")]
    pub base_loc: StLoc,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignatureXmlFile {
    #[serde(rename = "SignedInfo")]
    pub signed_info: SignedInfo,

    #[serde(rename = "SignedValue")]
    pub signed_value: StLoc,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignedInfo {
    #[serde(rename = "Provider")]
    pub provider: Provider,
    #[serde(rename = "SignatureMethod")]
    pub signature_method: Option<String>,
    #[serde(rename = "SignatureDateTime")]
    pub signature_date_time: Option<String>,

    #[serde(rename = "References")]
    pub references: References,

    #[serde(rename = "StampAnnot")]
    pub stamp_annot: Option<Vec<StampAnnot>>,

    #[serde(rename = "Seal")]
    pub seal: Option<Seal>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Seal {
    #[serde(rename = "BaseLoc")]
    pub base_loc: StLoc,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StampAnnot {
    #[serde(rename = "@ID")]
    pub id: String,

    #[serde(rename = "@PageRef")]
    pub page_ref: StRefId,

    #[serde(rename = "@Boundary")]
    pub boundary: StBox,

    #[serde(rename = "@Clip")]
    pub clip: Option<StBox>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct References {
    /// default `MD5`
    #[serde(rename = "@CheckMethod")]
    pub check_method: Option<String>,
    #[serde(rename = "Reference")]
    pub references: Vec<Reference>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reference {
    #[serde(rename = "@FileRef")]
    pub file_ref: StLoc,
    #[serde(rename = "CheckValue")]
    pub check_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    #[serde(rename = "@ProviderName")]
    pub provider_name: String,

    #[serde(rename = "@Version")]
    pub version: Option<String>,

    #[serde(rename = "@Company")]
    pub company: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;
    use eyre::Result;
    #[test]
    fn test_sigs() -> Result<()> {
        let file = File::open("../samples/sample/Doc_0/Signs/Signatures.xml")?;
        let reader = BufReader::new(file);
        let xml = quick_xml::de::from_reader::<_, SignaturesXmlFile>(reader)?;
        dbg!(xml);
        Ok(())
    }
    #[test]
    fn test_sig() -> Result<()> {
        let file = File::open("../samples/sample/Doc_0/Signs/Sign_0/Signature.xml")?;
        let reader = BufReader::new(file);
        let xml = quick_xml::de::from_reader::<_, SignatureXmlFile>(reader)?;
        dbg!(xml);
        Ok(())
    }
}
