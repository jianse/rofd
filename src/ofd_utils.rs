use std::{fs::create_dir_all, path::PathBuf};

use eyre::Result;

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
#[derive(Debug, Default)]
pub struct DocInfo {
    /// docid
    pub doc_id: Option<String>,

    /// how many page this doc have
    pub page_count: usize,

    /// how many template page this doc have
    pub template_count: usize,
}

pub fn get_info(path: &PathBuf) -> Result<OfdInfo> {
    let mut container = container::from_path(path)?;
    let xml: OfdXmlFile = container.entry()?.content;
    let doc_count = xml.doc_body.len();
    let doc_info = xml
        .doc_body
        .iter()
        .enumerate()
        .map(|(idx, ele)| {
            let doc_id = ele.doc_info.doc_id.clone();

            let xml: DocumentXmlFile = container.document_by_index(idx)?.content;
            let page_count = xml.pages.page.len();

            let template_count = match xml.common_data.template_page {
                Some(v) => v.len(),
                None => 0,
            };
            Ok(DocInfo {
                doc_id,
                page_count,
                template_count,
                ..Default::default()
            })
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

pub fn render_page(
    ofd_path: &PathBuf,
    output_path: &PathBuf,
    doc_index: usize,
    page_index: usize,
    only_template: bool,
) -> Result<()> {
    if !output_path.exists() {
        create_dir_all(output_path)?;
    } else {
        assert!(
            output_path.is_dir(),
            "path {} is not a dir!",
            output_path.display()
        );
    }

    let mut res = container::from_path(&ofd_path)?;
    let xml = res.template_by_index(doc_index, page_index)?.content;
    dbg!(xml);
    // let surface = create_surface();
    todo!()
}
// fn create_dir()
