use std::{fs::File, io::BufReader, path::PathBuf};

use eyre::{OptionExt, Result};
use zip::{read::ZipFile, ZipArchive};

use crate::{
    element::{
        base::StId,
        file::{document::DocumentXmlFile, ofd::OfdXmlFile, page::PageXmlFile},
    },
    error::MyError,
};
pub struct Container {
    // container:
    // path:
    zip_archive: ZipArchive<BufReader<File>>,
}
pub struct InnerFile<'a, T> {
    container: &'a mut Container,
    path: PathBuf,
    pub content: T,
}
/// this holds some resource for render
pub struct Resources {
    // public_res: Option<todo!()>,
}

impl<'a> InnerFile<'a, DocumentXmlFile> {
    pub fn get_page(&mut self, page_index: usize) -> Result<InnerFile<PageXmlFile>> {
        let a = &self.content.pages.page;
        let page = a.get(page_index).ok_or_eyre("no such page!")?;
        let abs_path = self.resolve(&page.base_loc);
        let page_xml = self.container.open(&abs_path)?;
        let reader = BufReader::new(page_xml);

        let xml = quick_xml::de::from_reader::<_, PageXmlFile>(reader)?;
        Ok(InnerFile {
            container: self.container,
            path: abs_path,
            content: xml,
        })
    }
    pub fn get_template_by_index(
        &mut self,
        template_index: usize,
    ) -> Result<InnerFile<PageXmlFile>> {
        let templates_option = self.content.common_data.template_page.as_ref();
        let templates = templates_option.ok_or_eyre("message")?;
        let t = templates.get(template_index).ok_or_eyre("message")?;
        let abs_path = self.resolve(&t.base_loc);
        let page_xml = self.container.open(&abs_path)?;
        let reader = BufReader::new(page_xml);

        let xml = quick_xml::de::from_reader::<_, PageXmlFile>(reader)?;
        Ok(InnerFile {
            container: self.container,
            path: abs_path,
            content: xml,
        })
    }
    pub fn get_template_by_id(&mut self, template_id: StId) -> Result<InnerFile<PageXmlFile>> {
        let templates_option = self.content.common_data.template_page.as_ref();
        let templates = templates_option.ok_or_eyre("message")?;
        let t = templates
            .iter()
            .find(|e| e.id == template_id)
            .ok_or_eyre("message")?;
        let abs_path = self.resolve(&t.base_loc);
        let page_xml = self.container.open(&abs_path)?;
        let reader = BufReader::new(page_xml);

        let xml = quick_xml::de::from_reader::<_, PageXmlFile>(reader)?;
        Ok(InnerFile {
            container: self.container,
            path: abs_path,
            content: xml,
        })
    }
}

impl<'a> InnerFile<'a, OfdXmlFile> {
    /// 通过索引获取Document.xml对象
    pub fn get_document(&mut self, doc_index: usize) -> Result<InnerFile<DocumentXmlFile>> {
        let db = &self.content.doc_body;
        let doc_body = db.get(doc_index).ok_or_eyre("no such doc")?;
        let path = doc_body.doc_root.as_ref().ok_or_eyre("no such document!")?;
        let path = self.resolve(path);
        let inner = self.container.open(&path)?;

        let reader = BufReader::new(inner);
        let xml: DocumentXmlFile = quick_xml::de::from_reader(reader)?;
        Ok(InnerFile {
            container: self.container,
            path,
            content: xml,
        })
        // todo!()
    }
}

impl<'a, T> InnerFile<'a, T> {
    fn resolve(&self, other: &PathBuf) -> PathBuf {
        if other.is_absolute() {
            other.clone()
        } else {
            //  = PathBuf::new();
            let folder = self.path.parent();
            let base = match folder {
                Some(p) => p.into(),
                None => PathBuf::new(),
            };
            base.join(other)
        }
    }
}

impl Container {
    pub fn open(&mut self, path: &PathBuf) -> Result<ZipFile> {
        // todo!()
        let path_str = path.to_str().ok_or_eyre("pathBuf to str faild!!")?;
        let file = self
            .zip_archive
            .by_name(path_str)
            .map_err(|e| MyError::OpenZipError(e, path.clone()))?;
        Ok(file)
    }
    pub fn entry(&mut self) -> Result<InnerFile<OfdXmlFile>> {
        let file = self.zip_archive.by_name("OFD.xml")?;
        let reader = BufReader::new(file);

        let xml: OfdXmlFile = quick_xml::de::from_reader(reader)?;
        Ok(InnerFile {
            container: self,
            path: "OFD.xml".into(),
            content: xml,
        })
    }
    pub fn document_by_index(&mut self, doc_index: usize) -> Result<InnerFile<DocumentXmlFile>> {
        let entry = self.entry()?;
        let doc_body = entry
            .content
            .doc_body
            .get(doc_index)
            .ok_or_eyre("no such document!")?;
        let path = doc_body.doc_root.as_ref().ok_or_eyre("no such document!")?;
        let path = entry.resolve(path);
        let inner = self.open(&path)?;

        let reader = BufReader::new(inner);
        let xml: DocumentXmlFile = quick_xml::de::from_reader(reader)?;
        Ok(InnerFile {
            container: self,
            path,
            content: xml,
        })
    }
    pub fn template_by_index(
        &mut self,
        doc_index: usize,
        template_index: usize,
    ) -> Result<InnerFile<PageXmlFile>> {
        let doc = self.document_by_index(doc_index)?;
        let tpls = doc
            .content
            .common_data
            .template_page
            .as_ref()
            .ok_or_eyre("no such template")?;
        let tpl_el = tpls.get(template_index).ok_or_eyre("no such template")?;
        let tpl_path = &tpl_el.base_loc;
        let tpl_path = doc.resolve(tpl_path);
        let inner = self.open(&tpl_path)?;
        let reader = BufReader::new(inner);
        let xml: PageXmlFile = quick_xml::de::from_reader(reader)?;
        Ok(InnerFile {
            container: self,
            path: tpl_path,
            content: xml,
        })
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
