use std::{fs::File, io::BufReader, path::PathBuf};

use eyre::{Ok, OptionExt, Result};
use relative_path::RelativePathBuf;
use zip::{read::ZipFile, ZipArchive};

use crate::{
    element::{
        base::{StId, StRefId},
        file::{document::DocumentXmlFile, ofd::OfdXmlFile, page::PageXmlFile},
    },
    error::MyError,
};
pub struct Container {
    // container:
    // path:
    zip_archive: ZipArchive<BufReader<File>>,
}
pub struct InnerFile<T> {
    // container: &'a mut Container,
    path: RelativePathBuf,
    pub content: T,
}
/// this holds some resource for render
pub struct Resources {
    // public_res: Option<todo!()>,
}

impl<'a, T> InnerFile<T> {
    fn resolve(&self, other: &PathBuf) -> RelativePathBuf {
        
        let this = self.path.clone();
        let this = dbg!(this);
        let that = RelativePathBuf::from_path(other).unwrap();
        let that = dbg!(that);
        // let res;
        let res = if that.starts_with("/") {
            that.normalize()
        } else {
            //  = PathBuf::new();
            let folder = this.parent();
            let base = match folder {
                Some(p) => p.into(),
                None => RelativePathBuf::new(),
            };
            let np = base.join(that).normalize();
            np
        };
        let res = dbg!(res);

        res
    }
}

impl Container {
    pub fn open(&mut self, path: String) -> Result<ZipFile> {
        let file = self
            .zip_archive
            .by_name(path.as_ref())
            .map_err(|e| MyError::OpenZipError(e, path))?;
        Ok(file)
    }
    pub fn entry(&mut self) -> Result<InnerFile<OfdXmlFile>> {
        let file = self.zip_archive.by_name("OFD.xml")?;
        let reader = BufReader::new(file);

        let xml: OfdXmlFile = quick_xml::de::from_reader(reader)?;
        Ok(InnerFile {
            // container: self,
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
        let inner = self.open(path.to_string())?;

        let reader = BufReader::new(inner);
        let xml: DocumentXmlFile = quick_xml::de::from_reader(reader)?;
        Ok(InnerFile {
            // container: self,
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
        let inner = self.open(tpl_path.to_string())?;
        let reader = BufReader::new(inner);
        let xml: PageXmlFile = quick_xml::de::from_reader(reader)?;
        // let cont = &*self;
        Ok(InnerFile {
            // container: self,
            path: tpl_path,
            content: xml,
        })
    }
    pub fn template_by_id(
        &mut self,
        doc_index: usize,
        template_id: StRefId,
    ) -> Result<InnerFile<PageXmlFile>> {
        todo!()
    }
    pub fn page_by_index(
        &mut self,
        doc_index: usize,
        page_index: usize,
    ) -> Result<InnerFile<PageXmlFile>> {
        let doc = self.document_by_index(doc_index)?;
        let tpls = &doc.content.pages.page;
        let tpl_el = tpls.get(page_index).ok_or_eyre("no such template")?;
        let tpl_path = &tpl_el.base_loc;
        let tpl_path = doc.resolve(tpl_path);
        let inner = self.open(tpl_path.to_string())?;
        let reader = BufReader::new(inner);
        let xml: PageXmlFile = quick_xml::de::from_reader(reader)?;
        // let cont = &*self;
        Ok(InnerFile {
            // container: self,
            path: tpl_path,
            content: xml,
        })
    }
    pub fn templates_for_page(
        &mut self,
        doc_index: usize,
        page_index: usize,
    ) -> Result<Vec<InnerFile<PageXmlFile>>> {
        let page = self.page_by_index(doc_index, page_index)?;
        let xml = &page.content;
        if let Some(tpls) = xml.template.as_ref() {
            let mut res = vec![];
            for tpl in tpls {
                let template_id = tpl.template_id;
                let t = self.template_by_id(doc_index, template_id)?;
                res.push(t);
            }
            Ok(res)
        } else {
            Ok(vec![])
        }

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

#[cfg(test)]
mod tests {
    use relative_path::RelativePathBuf;

    #[test]
    fn test_reltive_path() {
        let mut rp = RelativePathBuf::from("/a/value");
        rp.push("../");
        rp = rp.normalize();
        dbg!(rp.to_string());
    }
}
