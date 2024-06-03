use std::{fs::File, io::BufReader, path::PathBuf};

use eyre::{Ok, OptionExt, Result};
use zip::{read::ZipFile, ZipArchive};

use crate::element::file::{document::DocumentXmlFile, ofd::OfdXmlFile, page::PageXmlFile};
pub struct Container {
    // container:
    // path:
    zip_archive: ZipArchive<BufReader<File>>,
}
impl Container {
    pub fn open(&mut self, path: &PathBuf) -> Result<ZipFile> {
        // todo!()
        let path_str = path.to_str().ok_or_eyre("pathBuf to str faild!!")?;
        let file = self.zip_archive.by_name(path_str)?;
        Ok(file)
    }
    pub fn entry(&mut self) -> Result<OfdXmlFile> {
        let file = self.zip_archive.by_name("OFD.xml")?;
        let reader = BufReader::new(file);

        let xml: OfdXmlFile = quick_xml::de::from_reader(reader)?;
        Ok(xml)
    }
    pub fn document_by_index(&mut self, doc_index: usize) -> Result<DocumentXmlFile> {
        let entry = self.entry()?;
        let doc_body = entry
            .doc_body
            .get(doc_index)
            .ok_or_eyre("no such document!")?;
        let path = doc_body.doc_root.as_ref().ok_or_eyre("no such document!")?;
        let inner = self.open(path)?;

        let reader = BufReader::new(inner);
        let xml: DocumentXmlFile = quick_xml::de::from_reader(reader)?;
        Ok(xml)
    }
    pub fn template_by_index(
        &mut self,
        doc_index: usize,
        template_index: usize,
    ) -> Result<PageXmlFile> {
        let doc = self.document_by_index(doc_index)?;
        let tpls = doc
            .common_data
            .template_page
            .as_ref()
            .ok_or_eyre("no such template")?;
        let tpl_el = tpls.get(template_index).ok_or_eyre("no such template")?;
        let tpl_path = &tpl_el.base_loc;
        let inner = self.open(tpl_path)?;
        let reader = BufReader::new(inner);
        let xml: PageXmlFile = quick_xml::de::from_reader(reader)?;
        Ok(xml)
        // todo!()
    }
}

pub fn from_path(path: &PathBuf) -> Result<Container> {
    // todo!()
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let zip = zip::ZipArchive::new(reader)?;

    zip.index_for_name("OFD.xml")
        .ok_or_eyre("OFD entry point not found!!")?;
    Ok(Container { zip_archive: zip })
}
