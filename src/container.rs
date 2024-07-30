use std::{fs::File, io::BufReader, path::PathBuf};

use eyre::{Ok, OptionExt, Result};
use relative_path::RelativePathBuf;
use zip::{read::ZipFile, ZipArchive};

use crate::{
    element::{
        base::StRefId,
        file::{
            document::DocumentXmlFile,
            ofd::OfdXmlFile,
            page::PageXmlFile,
            res::{ColorSpace, DrawParam, ResourceXmlFile},
        },
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
    pub default_cs: Option<StRefId>,
    public_resource: Option<Vec<InnerFile<ResourceXmlFile>>>,
    document_resource: Option<Vec<InnerFile<ResourceXmlFile>>>,
    page_resource: Option<Vec<InnerFile<ResourceXmlFile>>>,
}

struct ResourceIter<'a> {
    res_flat: Vec<&'a InnerFile<ResourceXmlFile>>,
    // res: &'a Resources,
    idx: usize,
}
impl<'a> ResourceIter<'a> {
    fn new(res: &'a Resources) -> Self {
        let res_flat = [
            &res.page_resource,
            &res.document_resource,
            &res.public_resource,
        ]
        .iter()
        .filter_map(|e| e.as_ref())
        .flat_map(|e| e.iter())
        .collect::<Vec<&InnerFile<ResourceXmlFile>>>();
        Self {
            res_flat,
            // res,
            idx: 0,
        }
    }
}
impl<'a> Iterator for ResourceIter<'a> {
    type Item = &'a InnerFile<ResourceXmlFile>;

    fn next(&mut self) -> Option<Self::Item> {
        let res;
        if self.idx < self.res_flat.len() {
            res = Some(self.res_flat[self.idx]);
        } else {
            res = None;
        }
        self.idx += 1;
        res
        // todo!()
    }
}

impl Resources {
    fn iter(&self) -> ResourceIter {
        ResourceIter::new(self)
    }
    pub fn get_color_space_by_id(&self, color_space_id: StRefId) -> Option<&ColorSpace> {
        let cs = self
            .iter()
            .filter_map(|f| f.content.color_spaces.as_ref())
            .flat_map(|css| css.color_spaces.iter())
            .find(|cs| cs.id == color_space_id);
        cs
    }

    pub fn get_draw_param_by_id(&self, draw_param_id: StRefId) -> Option<DrawParam> {
        let dp = self
            .iter()
            .filter_map(|f| f.content.draw_params.as_ref())
            .flat_map(|dps| dps.draw_params.iter())
            .find(|dp| dp.id == draw_param_id);
        dp.cloned()
    }
}

impl<'a, T> InnerFile<T> {
    fn resolve(&self, other: &PathBuf) -> RelativePathBuf {
        let this = self.path.clone();
        let that = RelativePathBuf::from_path(other).unwrap();
        let res = if that.to_string().starts_with("/") {
            that.normalize()
        } else {
            let folder = this.parent();
            let base = match folder {
                Some(p) => p.into(),
                None => RelativePathBuf::new(),
            };
            let np = base.join(that).normalize();
            np
        };
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
    fn _template_by_index(
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
        let doc = self.document_by_index(doc_index)?;
        let tpls = doc
            .content
            .common_data
            .template_page
            .as_ref()
            .ok_or_eyre("no such template")?;
        let tpl_el = tpls
            .iter()
            .find(|i| i.id == template_id)
            .ok_or_eyre("no such template")?;
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
    pub fn resources_for_page(&mut self, doc_index: usize, page_index: usize) -> Result<Resources> {
        let doc = self.document_by_index(doc_index)?;
        let pub_res_locs = &doc.content.common_data.public_res;
        let pub_res = self._load_res(&doc, pub_res_locs)?;
        let doc_res_locs = &doc.content.common_data.document_res;
        let doc_res = self._load_res(&doc, doc_res_locs)?;
        let page = self.page_by_index(doc_index, page_index)?;
        let page_res_locs = &page.content.page_res;
        let page_res = self._load_res(&page, page_res_locs)?;
        Ok(Resources {
            default_cs: doc.content.common_data.default_cs,
            public_resource: pub_res,
            document_resource: doc_res,
            page_resource: page_res,
        })
    }

    fn _load_res<T>(
        &mut self,
        parent: &InnerFile<T>,
        res_list: &Option<Vec<PathBuf>>,
    ) -> Result<Option<Vec<InnerFile<ResourceXmlFile>>>> {
        let res = if let Some(paths) = res_list.as_ref() {
            if paths.is_empty() {
                None
            } else {
                let r = paths
                    .iter()
                    .map(|p| -> Result<InnerFile<ResourceXmlFile>> {
                        let rp = parent.resolve(p);
                        let file = self.open(rp.to_string())?;
                        let reader = BufReader::new(file);

                        let xml: ResourceXmlFile = quick_xml::de::from_reader(reader)?;

                        Ok(InnerFile {
                            path: rp,
                            content: xml,
                        })
                    })
                    .collect::<Result<Vec<InnerFile<ResourceXmlFile>>>>()?;
                Some(r)
            }
        } else {
            None
        };
        Ok(res)
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
    use std::path::PathBuf;

    use relative_path::RelativePathBuf;

    use super::InnerFile;

    #[test]
    fn test_reltive_path() {
        let mut rp = RelativePathBuf::from("/a/value");
        rp.join("/foo/bar");
        rp = rp.normalize();
        dbg!(rp.to_string());
        let rp = rp.relative("/");
        dbg!(rp.to_string());
    }
    #[test]
    fn test_resolve() {
        let l = InnerFile {
            path: RelativePathBuf::from("a/b"),
            content: String::new(),
        };
        let r = l.resolve(&PathBuf::from("value".to_string()));
        assert_eq!(r.to_string(), "a/value")
    }
}
