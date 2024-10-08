use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use cli_table::Table;
use eyre::{OptionExt, Result};

use crate::{
    container::{self, Container},
    element::file::{document::DocumentXmlFile, ofd::OfdXmlFile},
    render,
};
#[derive(Debug)]
pub struct OfdInfo {
    /// how many doc this ofd contains
    pub doc_count: usize,

    /// infos for each doc
    pub doc_info: Vec<DocInfo>,
}

fn fmt_doc_id(doc_id: &Option<String>) -> String {
    match doc_id {
        Some(v) => v.to_string(),
        None => "".to_string(),
    }
}
#[derive(Debug, Default, Table)]
pub struct DocInfo {
    /// doc id
    #[table(title = "doc_id", display_fn = "fmt_doc_id")]
    pub doc_id: Option<String>,

    /// how many page this doc have
    #[table(title = "page_count")]
    pub page_count: usize,

    /// how many template page this doc have
    #[table(title = "template_count")]
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
            })
        })
        .collect::<Result<Vec<DocInfo>>>()?;
    // let db = xml.doc_body.first().ok_or_eyre("empty?")?;
    Ok(OfdInfo {
        doc_count,
        doc_info,
    })
}

fn get_doc_count(container: &mut Container) -> Result<usize> {
    let xml: OfdXmlFile = container.entry()?.content;
    let doc_count = xml.doc_body.len();
    Ok(doc_count)
}

fn get_page_count(container: &mut Container, doc_index: usize) -> Result<usize> {
    let xml: DocumentXmlFile = container.document_by_index(doc_index)?.content;
    let page_count = xml.pages.page.len();
    Ok(page_count)
}

pub fn render_page(
    ofd_path: &PathBuf,
    output_path: &PathBuf,
    doc_index: usize,
    page_index: usize,
    only_template: bool,
) -> Result<()> {
    let mut res = container::from_path(ofd_path)?;

    let page_count = get_doc_count(&mut res)?;
    assert!(
        doc_index < page_count,
        "doc index out of range. could be 0 to {}",
        page_count - 1
    );

    let page_count = get_page_count(&mut res, doc_index)?;
    assert!(
        page_index < page_count,
        "page index out of range. could be 0 to {}",
        page_count - 1
    );

    if !output_path.exists() {
        create_dir_all(output_path)?;
    } else {
        assert!(
            output_path.is_dir(),
            "path {} is not a dir!",
            output_path.display()
        );
    }

    let image = if only_template {
        render::render_template(&mut res, doc_index, page_index)?
    } else {
        render::render_page(&mut res, doc_index, page_index)?
    };

    let data = image
        .encode(None, skia_safe::EncodedImageFormat::PNG, 100)
        .ok_or_eyre("message")?;
    let mut op = PathBuf::from(output_path);
    op.push(format!("page_{doc_index}_{page_index}.png"));
    let mut out = File::create(op)?;
    let _om = out.write(&data)?;
    Ok(())
}
// fn create_dir()
