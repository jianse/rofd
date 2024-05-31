use std::{io::BufReader, path::PathBuf, str::FromStr};

use eyre::{OptionExt, Result};

use crate::{
    container,
    element::file::{document::DocumentXmlFile, ofd::OfdXmlFile},
};
#[derive(Debug)]
pub struct OfdInfo {
    /// how many doc this ofd contains
    pub doc_count: usize,

    pub doc_info: Vec<DocInfo>,
}
#[derive(Debug)]
pub struct DocInfo {
    /// docid
    pub doc_id: Option<String>,
    
    /// how many page this doc have
    pub page_count: usize,
}

pub fn get_info(path: PathBuf) -> Result<OfdInfo> {
    let mut res = container::from_path(path)?;
    let entry_path = PathBuf::from_str(r"OFD.xml")?;
    let ofd_entry = res.open(&entry_path)?;
    let reader = BufReader::new(ofd_entry);

    let xml: OfdXmlFile = quick_xml::de::from_reader(reader)?;
    let doc_count = xml.doc_body.len();
    let doc_info = xml
        .doc_body
        .iter()
        .map(|ele| {
            let doc_id = ele.doc_info.doc_id.clone();
            let dr = ele
                .doc_root
                .as_ref()
                .ok_or_eyre("unable to locate document root")?;
            // dbg!(dr);
            // dbg!(xml);
            let document_xml = res.open(dr)?;
            let reader = BufReader::new(document_xml);

            let xml: DocumentXmlFile = quick_xml::de::from_reader(reader)?;
            let page_count = xml.pages.page.len();
            Ok(DocInfo { doc_id, page_count })
        })
        .filter_map(
            |e: Result<DocInfo>| {
                if e.is_ok() {
                    Some(e.unwrap())
                } else {
                    None
                }
            },
        )
        .collect();
    // let db = xml.doc_body.first().ok_or_eyre("empty?")?;
    Ok(OfdInfo {
        doc_count,
        doc_info,
    })
}
