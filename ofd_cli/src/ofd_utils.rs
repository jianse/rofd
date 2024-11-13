use cli_table::Table;
use eyre::{OptionExt, Result};
use interpolator::{format, Formattable};
use ofd_base::file::document::DocumentXmlFile;
use ofd_conv::img::render;
use ofd_rw::{self, Ofd};
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};
use tracing::info;
#[derive(Debug)]
pub struct OfdInfo {
    /// how many doc this ofd contains
    pub doc_count: usize,

    /// infos for each doc
    pub doc_info: Vec<DocInfo>,

    pub item_names: Vec<String>,
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
// pub fn get_name_iter
pub fn get_info(path: &PathBuf) -> Result<OfdInfo> {
    let container = ofd_rw::from_path(path)?;
    let xml = container.entry()?.content;
    let doc_count = xml.doc_body.len();
    let doc_info = xml
        .doc_body
        .to_owned()
        .iter()
        .enumerate()
        .map(|(idx, ele)| {
            let doc_id = ele.doc_info.doc_id.to_owned();

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
        item_names: container.item_names(),
    })
}

fn get_doc_count(container: &Ofd) -> Result<usize> {
    let item = container.entry()?;

    let xml = item.content;
    let doc_count = xml.doc_body.len();
    Ok(doc_count)
}

fn get_page_count(container: &Ofd, doc_index: usize) -> Result<usize> {
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
    path_template: &str,
) -> Result<()> {
    let res = ofd_rw::from_path(ofd_path)?;

    let doc_count = get_doc_count(&res)?;
    assert!(
        doc_index < doc_count,
        "doc index out of range. could be 0 to {}",
        doc_count - 1
    );

    let page_count = get_page_count(&res, doc_index)?;
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
        render::render_template(&res, doc_index, page_index)?
    } else {
        render::render_page(&res, doc_index, page_index)?
    };

    let data = image
        .encode(None, ofd_conv::img::EncodedImageFormat::PNG, 100)
        .ok_or_eyre("message")?;
    write_image(
        &data,
        path_template,
        ofd_path,
        output_path,
        doc_index,
        page_index,
        "png",
    )?;
    Ok(())
}
// fn create_dir()

// render all pages for doc_index
pub(crate) fn render_doc(
    ofd_path: &PathBuf,
    out_dir_path: &Path,
    doc_index: usize,
    path_template: &str,
) -> Result<()> {
    let res = ofd_rw::from_path(ofd_path)?;

    let doc_count = get_doc_count(&res)?;

    assert!(
        doc_index < doc_count,
        "doc index out of range. could be 0 to {}",
        doc_count - 1
    );
    let page_count = get_page_count(&res, doc_index)?;

    let mut render = render::Render::new(res, "楷体")?;
    for pid in 0..page_count {
        info!("rendering doc {} page {}", doc_index, pid);
        let mut i = render.render_page(doc_index, pid)?;
        let img = i.image_snapshot();
        let data = img
            .encode(None, ofd_conv::img::EncodedImageFormat::PNG, 100)
            .ok_or_eyre("can not encode image to png!")?;

        write_image(
            &data,
            path_template,
            ofd_path,
            out_dir_path,
            doc_index,
            pid,
            "png",
        )?;
    }
    Ok(())
}

pub(crate) fn render_ofd(p0: &PathBuf, p1: &Path, path_template: &str) -> Result<()> {
    let res = ofd_rw::from_path(p0)?;

    let doc_count = get_doc_count(&res)?;
    let mut render = render::Render::new(res.clone(), "楷体")?;

    for doc_index in 0..doc_count {
        let page_count = get_page_count(&res, doc_index)?;
        for page_index in 0..page_count {
            let bt = Instant::now();
            let mut sur = render.render_page(doc_index, page_index)?;
            let img = sur.image_snapshot();
            let data = img
                .encode(None, ofd_conv::img::EncodedImageFormat::PNG, 100)
                .ok_or_eyre("can not encode image to png!")?;

            write_image(&data, path_template, p0, p1, doc_index, page_index, "png")?;
            let t = bt.elapsed();
            info!(
                "rendering doc {} page {} took: {} ms",
                doc_index,
                page_index,
                t.as_millis()
            );
        }
    }

    Ok(())
}

fn write_image(
    buf: &[u8],
    path_template: &str,
    in_path: &Path,
    out_path: &Path,
    doc_index: usize,
    page_index: usize,
    ext: &str,
) -> Result<()> {
    let op = {
        let ofd_file_name = in_path.file_name().unwrap().to_str().unwrap();
        let ext = ext.to_lowercase();
        format(
            path_template,
            &[
                ("out_path", Formattable::display(&out_path.display())),
                ("ofd_file_name", Formattable::display(&ofd_file_name)),
                ("doc_index", Formattable::display(&doc_index)),
                ("page_index", Formattable::display(&page_index)),
                ("ext", Formattable::display(&ext)),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>(),
        )?
    };
    let p = PathBuf::from(op);
    if let Some(parent) = p.parent() {
        create_dir_all(parent)?;
    }
    let mut out = File::create(p)?;
    let _om = out.write(buf)?;
    Ok(())
}
