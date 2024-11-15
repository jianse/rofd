use crate::error::{Error, Result};
use minidom::Element;
use ofd_base::file::annotation::{AnnotationXmlFile, AnnotationsXmlFile};
use ofd_base::file::res::Resource;
use ofd_base::{
    file::{
        document::DocumentXmlFile,
        ofd::OfdXmlFile,
        page::PageXmlFile,
        res::{ColorSpace, DrawParam, Font, ResourceXmlFile},
    },
    StRefId,
};
use relative_path::{PathExt, RelativePathBuf};
use serde::de::DeserializeOwned;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{BufRead, Read};
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;
use std::{fs::File, io::BufReader, path::PathBuf};
use zip::{read::ZipFile, ZipArchive};

#[derive(Eq, Hash, PartialEq)]
struct CacheKey {
    path: String,
    tid: TypeId,
}

// pub type  Ofd =  Rc<RefCell<RawOfd>>;

struct RawOfd {
    zip_archive: ZipArchive<BufReader<File>>,
    cache: HashMap<CacheKey, Box<dyn Any>>,
}

#[derive(Clone)]
pub struct Ofd(Rc<RefCell<RawOfd>>);

impl Ofd {
    fn from_raw(raw_ofd: RawOfd) -> Self {
        Self(Rc::new(RefCell::new(raw_ofd)))
    }

    pub fn entry(&self) -> Result<OfdItem<OfdXmlFile>> {
        self.0.borrow_mut().entry()
    }

    pub fn document_by_index(&self, doc_index: usize) -> Result<OfdItem<DocumentXmlFile>> {
        self.0.borrow_mut().document_by_index(doc_index)
    }

    pub fn page_by_index(
        &self,
        doc_index: usize,
        page_index: usize,
    ) -> Result<OfdItem<PageXmlFile>> {
        self.0.borrow_mut().page_by_index(doc_index, page_index)
    }

    pub fn templates_for_page(
        &self,
        doc_index: usize,
        page_index: usize,
    ) -> Result<Vec<OfdItem<PageXmlFile>>> {
        self.0
            .borrow_mut()
            .templates_for_page(doc_index, page_index)
    }

    pub fn resources_for_page(&self, doc_index: usize, page_index: usize) -> Result<Resources> {
        self.0
            .borrow_mut()
            .resources_for_page(doc_index, page_index)
    }

    pub fn annotations_for_page(
        &self,
        doc_index: usize,
        page_index: usize,
    ) -> Result<Vec<OfdItem<AnnotationXmlFile>>> {
        self.0
            .borrow_mut()
            .annotations_for_page(doc_index, page_index)
    }

    pub fn item_names(&self) -> Vec<String> {
        self.0
            .borrow_mut()
            .item_names()
            .map(str::to_owned)
            .collect::<Vec<_>>()
    }

    pub fn bytes(&self, path: impl AsRef<str> + Into<String>) -> Result<Vec<u8>> {
        self.0.borrow_mut().bytes(path)
    }
}

impl RawOfd {
    const OFD_ENTRY: &'static str = "OFD.xml";

    fn read_item<T, R>(reader: R) -> Result<T>
    where
        T: DeserializeOwned,
        R: BufRead,
    {
        let mut reader = BufReader::new(reader);
        let buf = reader.fill_buf()?;

        // UTF-8 BOM
        // handle u+FEFF in utf-8 file
        // just skip this three bytes
        if buf.starts_with(&[0xef_u8, 0xbb, 0xbf]) {
            reader.consume(3);
        }
        let root = Element::from_reader(reader)?;
        let res: T = xdom::de::from_ele(&root)?;
        Ok(res)
    }

    fn open<P>(&mut self, path: P) -> Result<ZipFile>
    where
        P: AsRef<str> + Into<String>,
    {
        let file = self
            .zip_archive
            .by_name(path.as_ref())
            .map_err(|e| Error::OpenZipError(e, path.into()))?;
        Ok(file)
    }

    /// get an item from cache
    fn get_cache<T: 'static, S>(&mut self, path: S) -> Option<&T>
    where
        S: Into<String>,
    {
        let tid = TypeId::of::<T>();
        let path = path.into();
        let key = CacheKey { path, tid };
        self.cache.get(&key).and_then(|x| x.downcast_ref::<T>())
    }

    /// getting from cache or parse xml from file
    fn cache_or<R, P>(&mut self, path: P) -> Result<R>
    where
        R: Clone + 'static + DeserializeOwned,
        P: Into<String>,
    {
        let p = path.into();
        if let Some(cache) = self.get_cache::<R, _>(p.clone()) {
            Ok(cache.clone())
        } else {
            let inner = self.open(&p)?;
            let reader = BufReader::new(inner);
            let xml: R = RawOfd::read_item(reader)?;
            self.set_cache(p, xml.clone());
            Ok(xml)
        }
    }

    /// set an item into cache
    fn set_cache<T: 'static, S>(&mut self, path: S, value: T)
    where
        S: Into<String>,
    {
        let tid = TypeId::of::<T>();
        let p = path.into();
        let key = CacheKey { path: p, tid };
        self.cache.insert(key, Box::new(value));
    }

    /// get entry file of ofd
    pub fn entry(&mut self) -> Result<OfdItem<OfdXmlFile>> {
        let xml = self.cache_or(RawOfd::OFD_ENTRY)?;

        Ok(OfdItem {
            path: RawOfd::OFD_ENTRY.into(),
            content: xml,
        })
    }

    /// get a reader
    pub fn _reader<P>(&mut self, path: P) -> Result<BufReader<ZipFile>>
    where
        P: AsRef<str> + Into<String>,
    {
        let file = self.zip_archive.by_name(path.as_ref())?;
        Ok(BufReader::new(file))
    }

    /// get as bytes
    pub fn bytes<P>(&mut self, path: P) -> Result<Vec<u8>>
    where
        P: AsRef<str> + Into<String>,
    {
        let mut file = self.zip_archive.by_name(path.as_ref())?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn document_by_index(&mut self, doc_index: usize) -> Result<OfdItem<DocumentXmlFile>> {
        // get path
        let entry = self.entry()?;
        let doc_body = entry
            .content
            .doc_body
            .get(doc_index)
            .ok_or(Error::NoSuchDocument)?;
        let path = doc_body.doc_root.as_ref().ok_or(Error::NoSuchDocument)?;
        let path = entry.resolve(path);

        // cache or read
        let xml = self.cache_or(path.clone())?;

        Ok(OfdItem { path, content: xml })
    }

    #[deprecated]
    fn _template_by_index(
        &mut self,
        doc_index: usize,
        template_index: usize,
    ) -> Result<OfdItem<PageXmlFile>> {
        let doc = self.document_by_index(doc_index)?;
        let tpls = doc
            .content
            .common_data
            .template_page
            .as_ref()
            .ok_or(Error::NoSuchTemplate)?;
        let tpl_el = tpls.get(template_index).ok_or(Error::NoSuchTemplate)?;
        let tpl_path = &tpl_el.base_loc;
        let tpl_path = doc.resolve(tpl_path);
        let inner = self.open(tpl_path.to_string())?;
        let reader = BufReader::new(inner);
        let xml: PageXmlFile = RawOfd::read_item(reader)?;
        // let cont = &*self;
        Ok(OfdItem {
            // container: self,
            path: tpl_path,
            content: xml,
        })
    }

    /// get template by id
    pub fn template_by_id(
        &mut self,
        doc_index: usize,
        template_id: StRefId,
    ) -> Result<OfdItem<PageXmlFile>> {
        // path
        let doc = self.document_by_index(doc_index)?;
        let template_refs = doc
            .content
            .common_data
            .template_page
            .as_ref()
            .ok_or(Error::NoSuchTemplate)?;
        let tpl_el = template_refs
            .iter()
            .find(|i| i.id == template_id)
            .ok_or(Error::NoSuchTemplate)?;
        let tpl_path = &tpl_el.base_loc;
        let path = doc.resolve(tpl_path);

        let xml = self.cache_or(path.clone())?;

        Ok(OfdItem { path, content: xml })
    }
    pub fn page_by_index(
        &mut self,
        doc_index: usize,
        page_index: usize,
    ) -> Result<OfdItem<PageXmlFile>> {
        let doc = self.document_by_index(doc_index)?;
        // get path
        let pages = &doc.content.pages.page;
        let tpl_el = pages.get(page_index).ok_or(Error::NoSuchTemplate)?;
        let tpl_path = &tpl_el.base_loc;
        let path = doc.resolve(tpl_path);

        // cache or read
        let xml = self.cache_or(path.clone())?;

        Ok(OfdItem {
            // container: self,
            path,
            content: xml,
        })
    }

    /// get templates for a page
    pub fn templates_for_page(
        &mut self,
        doc_index: usize,
        page_index: usize,
    ) -> Result<Vec<OfdItem<PageXmlFile>>> {
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
        parent: &OfdItem<T>,
        res_list: &Option<Vec<PathBuf>>,
    ) -> Result<Option<Vec<OfdItem<ResourceXmlFile>>>> {
        let res = if let Some(paths) = res_list.as_ref() {
            if paths.is_empty() {
                None
            } else {
                let r = paths
                    .iter()
                    .map(|p| -> Result<OfdItem<ResourceXmlFile>> {
                        let rp = parent.resolve(p);
                        let file = self.open(rp.to_string())?;
                        let reader = BufReader::new(file);

                        let xml: ResourceXmlFile = RawOfd::read_item(reader)?;

                        Ok(OfdItem {
                            path: rp,
                            content: xml,
                        })
                    })
                    .collect::<Result<Vec<OfdItem<ResourceXmlFile>>>>()?;
                Some(r)
            }
        } else {
            None
        };
        Ok(res)
    }

    pub fn item_names(&self) -> impl Iterator<Item = &str> {
        self.zip_archive.file_names()
    }

    /// load resources for doc
    pub fn _resources_for_doc(&mut self) -> Result<OfdItem<ResourceXmlFile>> {
        let entry = self.entry()?;
        let _xml = entry.content;
        // xml.doc_body
        todo!()
    }

    pub fn annotations_for_page(
        &mut self,
        doc_index: usize,
        page_index: usize,
    ) -> Result<Vec<OfdItem<AnnotationXmlFile>>> {
        let doc = self.document_by_index(doc_index)?;

        let vec = &doc.pages.page;
        assert!(page_index < vec.len(), "page_index out of range");
        let page_id = vec[page_index].id;
        if let Some(loc) = &doc.annotations {
            let path = doc.resolve(loc);
            let file = self.open(path.to_string())?;
            let reader = BufReader::new(file);
            let xml: AnnotationsXmlFile = RawOfd::read_item(reader)?;
            if let Some(pages) = &xml.page {
                let anno_vec = pages
                    .iter()
                    .filter(|p| p.page_id == page_id)
                    .map(|f| {
                        let p = inner_resolve(&path, &f.file_loc);
                        let file = self.open(p.to_string())?;

                        let reader = BufReader::new(file);
                        let xml: AnnotationXmlFile = RawOfd::read_item(reader)?;
                        Ok(OfdItem {
                            path: p,
                            content: xml,
                        })
                    })
                    .collect::<Result<Vec<_>>>();
                anno_vec
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
        }
    }
}

#[derive(Debug)]
pub struct OfdItem<T> {
    // container: &'a mut Container,
    path: RelativePathBuf,
    pub content: T,
}
impl<T> Deref for OfdItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
/// this holds some resource for render
pub struct Resources {
    pub default_cs: Option<StRefId>,
    public_resource: Option<Vec<OfdItem<ResourceXmlFile>>>,
    document_resource: Option<Vec<OfdItem<ResourceXmlFile>>>,
    page_resource: Option<Vec<OfdItem<ResourceXmlFile>>>,
}

pub struct ResourceIter<'a> {
    res_flat: Vec<&'a OfdItem<ResourceXmlFile>>,
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
        .collect::<Vec<&OfdItem<ResourceXmlFile>>>();
        Self {
            res_flat,
            // res,
            idx: 0,
        }
    }
}
impl<'a> Iterator for ResourceIter<'a> {
    type Item = &'a OfdItem<ResourceXmlFile>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = if self.idx < self.res_flat.len() {
            Some(self.res_flat[self.idx])
        } else {
            None
        };
        self.idx += 1;
        res
    }
}

impl Resources {
    pub fn iter(&self) -> ResourceIter {
        ResourceIter::new(self)
    }
    pub fn get_color_space_by_id(&self, color_space_id: StRefId) -> Option<&ColorSpace> {
        let cs = self
            .iter()
            .filter_map(|f| f.content.resources.as_ref())
            .flat_map(|r| r.iter())
            .filter_map(|r| match r {
                Resource::ColorSpaces(css) => Some(css),
                _ => None,
            })
            .flat_map(|css| css.color_spaces.iter())
            .find(|cs| cs.id == color_space_id);
        cs
    }

    pub fn get_draw_param_by_id(&self, draw_param_id: StRefId) -> Option<DrawParam> {
        let dp = self
            .iter()
            .filter_map(|f| f.content.resources.as_ref())
            .flat_map(|r| r.iter())
            .filter_map(|r| match r {
                Resource::DrawParams(dp) => Some(dp),
                _ => None,
            })
            .flat_map(|dps| dps.draw_params.iter())
            .find(|dp| dp.id == draw_param_id);
        dp.cloned()
    }
    pub fn get_font_by_id(&self, font_id: StRefId) -> Option<(&OfdItem<ResourceXmlFile>, &Font)> {
        let font = self
            .iter()
            .filter_map(|f| f.content.resources.as_ref().map(|r| (f, r)))
            .flat_map(|(f, v)| v.iter().map(move |v| (f, v)))
            .filter_map(|(f, r)| match r {
                Resource::Fonts(fts) => Some((f, fts)),
                _ => None,
            })
            .flat_map(|(fi, f)| f.fonts.iter().map(move |f| (fi, f)))
            .find(|(_, f)| f.id == font_id);
        font
    }
    pub fn _get_image_by_id(&self, _image_id: StRefId) -> Option<String> {
        todo!()
    }
}

impl<T> OfdItem<T> {
    pub fn resolve(&self, other: &PathBuf) -> RelativePathBuf {
        let this = self.path.clone();
        inner_resolve(&this, other)
    }
}
fn inner_resolve(this: &RelativePathBuf, other: &PathBuf) -> RelativePathBuf {
    if other.starts_with("/") {
        return other.relative_to("/").unwrap();
    }
    let that = RelativePathBuf::from_path(other).unwrap();
    let res = if that.to_string().starts_with('/') {
        that.normalize()
    } else {
        let folder = this.parent();
        let base = match folder {
            Some(p) => p.into(),
            None => RelativePathBuf::new(),
        };
        base.join(that).normalize()
    };
    res
}
pub fn from_path(path: impl AsRef<Path>) -> Result<Ofd> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let zip = ZipArchive::new(reader)?;

    let _ = zip
        .index_for_name("OFD.xml")
        .ok_or(Error::OfdEntryNotFound)?;
    Ok(Ofd::from_raw(RawOfd {
        zip_archive: zip,
        cache: HashMap::new(),
    }))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use relative_path::RelativePathBuf;

    use super::OfdItem;

    #[test]
    fn test_relative_path() {
        let mut rp = RelativePathBuf::from("/a/value");
        rp.join("/foo/bar");
        rp = rp.normalize();
        dbg!(rp.to_string());
        let rp = rp.relative("/");
        dbg!(rp.to_string());
    }
    #[test]
    fn test_resolve() {
        let l = OfdItem {
            path: RelativePathBuf::from("a/b"),
            content: String::new(),
        };
        let r = l.resolve(&PathBuf::from("value".to_string()));
        assert_eq!(r.to_string(), "a/value")
    }
}
