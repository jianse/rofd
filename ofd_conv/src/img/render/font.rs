use skia_safe::{FontMgr, FontStyle, Typeface};
use std::io::{Read, Seek};
use std::path::Path;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};
use walkdir::WalkDir;

use crate::error::MyError;
use eyre::{eyre, Result};
use ofd_base::{StId, StRefId};
use ofd_rw::Ofd;
use tracing::{debug, warn};

pub(super) struct LocalDirFontMgr {
    dir: PathBuf,
    system_font_mgr: FontMgr,
    font_cache: HashMap<String, FontStyleSet>,
}

impl LocalDirFontMgr {
    pub fn form_path(font_dir: impl AsRef<Path>) -> Self {
        // let dir = font_dir.as_ref();
        let font_dir = font_dir.as_ref().to_path_buf();
        let sys_fm = FontMgr::new();
        let mut font_cache = HashMap::new();
        debug!("init font manager,{}", font_dir.display());
        for entry in WalkDir::new(&font_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .map(|f| f.to_ascii_lowercase().ends_with(".ttf"))
                    .unwrap_or(false)
                    && e.file_type().is_file()
            })
        {
            debug!("try load font \"{}\"", entry.path().display());
            let path = entry.path();
            let bytes = fs::read(path);
            if let Ok(bytes) = bytes {
                let ff = sys_fm.new_from_data(bytes.as_ref(), 0);
                if let Some(ff) = ff {
                    let family_names = ff
                        .new_family_name_iterator()
                        .map(|l| l.string)
                        .collect::<HashSet<String>>();
                    for family_name in family_names {
                        debug!(
                            "font family name \"{}\", font_style {:?}",
                            family_name,
                            ff.font_style()
                        );
                        font_cache
                            .entry(family_name.clone())
                            .or_insert(FontStyleSet::new())
                            .add_font(ff.clone());
                    }
                } else {
                    warn!("warn! load font error. path {}", path.display());
                }
            } else {
                warn!("warn! read file error. path {}", path.display());
            }
        }

        Self {
            dir: font_dir,
            system_font_mgr: sys_fm,
            font_cache,
        }
    }
    pub fn match_family_style(
        &self,
        family_name: impl AsRef<str>,
        style: FontStyle,
    ) -> Option<Typeface> {
        self.font_cache
            .get(family_name.as_ref())
            .map(|font_style_set| font_style_set.match_style(style))?
    }
}

struct FontStyleSet {
    // font_style : FontStyle,
    fonts: Vec<Typeface>,
}

impl FontStyleSet {
    fn new() -> Self {
        Self { fonts: vec![] }
    }
    fn add_font(&mut self, font: Typeface) {
        let dup = self.match_style(font.font_style());
        if dup.is_some() {
            warn!(
                "warn! font dup. family_name={},style={:?}",
                font.family_name(),
                font.font_style()
            );
            return;
        }
        self.fonts.push(font);
    }
    pub fn count(&self) -> usize {
        self.fonts.len()
    }

    pub fn style(&mut self, index: usize) -> (FontStyle, Option<String>) {
        assert!(index < self.fonts.len());
        let font_style = self.fonts[index].font_style();
        (font_style, None)
    }

    pub fn new_typeface(&mut self, index: usize) -> Option<Typeface> {
        self.fonts.get(index).cloned()
    }

    pub fn match_style(&self, pattern: FontStyle) -> Option<Typeface> {
        self.fonts
            .iter()
            .find(|i| i.font_style().eq(&pattern))
            .cloned()
    }
}

/// struct handle ofd embedded fonts
struct EmbeddedFontMgr<I> {
    ofd: Ofd<I>,
    system_font_mgr: FontMgr,
    font_cache: HashMap<String, Typeface>,
}

impl<I: Read + Seek> EmbeddedFontMgr<I> {
    pub(crate) fn load_embed_font(
        &mut self,
        path: impl AsRef<str> + Into<String>,
    ) -> Result<Typeface> {
        let key = path.as_ref().to_string();
        if let Some(tf) = self.font_cache.get(&key).cloned() {
            Ok(tf)
        } else {
            let bytes = self.ofd.bytes(path)?;
            let tf = self.system_font_mgr.new_from_data(&bytes, 0);
            let tf = tf.ok_or_else(|| eyre!("failed to load embedded font"))?;
            self.font_cache.insert(key.to_string(), tf.clone());
            Ok(tf)
        }
    }
    pub fn from_ofd(ofd: Ofd<I>) -> Self {
        let system_font_mgr = FontMgr::new();
        let font_cache = HashMap::new();

        // ofd.res

        Self {
            ofd,
            system_font_mgr,
            font_cache,
        }
    }

    /// load embed font from document.xml
    /// including public_res and document_res
    pub(super) fn load_doc(&mut self, doc_index: usize) {
        todo!()
    }

    pub(super) fn load_page(&mut self, doc_index: usize, page_index: usize) {
        todo!()
    }

    pub fn match_resource_id(&self, resource_id: StId) -> Option<Typeface> {
        todo!()
    }

    // pub fn
}

pub struct AggFontMgr<I> {
    ofd: Ofd<I>,
    embedded_font_mgr: EmbeddedFontMgr<I>,
    local_dir_font_mgr: Option<LocalDirFontMgr>,
    system_font_mgr: FontMgr,
    fallback_font_name: String,
    fallback: Typeface,
}

impl<I> AggFontMgr<I> {
    pub(crate) fn match_family_style(
        &self,
        family_name: &String,
        style: FontStyle,
    ) -> Option<Typeface> {
        self.local_dir_font_mgr
            .as_ref()
            .and_then(|lfm| lfm.match_family_style(family_name, style))
            .or_else(|| self.system_font_mgr.match_family_style(family_name, style))
    }
}

impl<I: Read + Seek> AggFontMgr<I> {
    pub(crate) fn fallback_typeface(&self) -> Typeface {
        self.fallback.clone()
    }

    pub(super) fn load_embed_font(
        &mut self,
        path: impl AsRef<str> + Into<String>,
    ) -> Result<Typeface> {
        self.embedded_font_mgr.load_embed_font(path)
    }
    pub(crate) fn typeface_by_resource_id(&self, resource_id: StRefId) -> Typeface {
        // self.ofd.resources_for_page()
        todo!()
    }
    pub(super) fn load_page(&self, doc_index: usize, page_index: usize) {
        // let resources = self.ofd.resources_for_page(doc_index, page_index)?;
        // resources.iter().find()
        // resources.
        // todo!()
    }
}

impl<I: Read + Seek> AggFontMgr<I> {
    pub(super) fn builder(
        ofd: Ofd<I>,
        fallback_font_name: impl AsRef<str>,
    ) -> AggFontMgrBuilder<I> {
        AggFontMgrBuilder::new(ofd, fallback_font_name)
    }
}

pub(super) struct AggFontMgrBuilder<I> {
    ofd: Ofd<I>,
    font_dir: Option<PathBuf>,
    fallback_font_name: String,
}

impl<I: Read + Seek> AggFontMgrBuilder<I> {
    pub fn new(ofd: Ofd<I>, fallback_font_name: impl AsRef<str>) -> Self {
        Self {
            ofd,
            fallback_font_name: fallback_font_name.as_ref().to_owned(),
            font_dir: None,
        }
    }

    pub fn font_dir(mut self, font_dir: impl AsRef<Path>) -> Self {
        let font_dir = font_dir.as_ref().to_path_buf();
        self.font_dir = Some(font_dir);
        self
    }

    pub fn build(mut self) -> Result<AggFontMgr<I>> {
        let system_fm = FontMgr::new();
        let local_dir_fm = self.font_dir.take().map(LocalDirFontMgr::form_path);

        // fallback font
        let tf = if let Some(lfm) = &local_dir_fm {
            lfm.match_family_style(&self.fallback_font_name, FontStyle::normal())
        } else {
            None
        }
        .or_else(|| system_fm.match_family_style(&self.fallback_font_name, FontStyle::normal()))
        .ok_or(MyError::NoFallbackFontSet)?;

        Ok(AggFontMgr {
            ofd: self.ofd.clone(),
            embedded_font_mgr: EmbeddedFontMgr::from_ofd(self.ofd),
            local_dir_font_mgr: local_dir_fm,
            system_font_mgr: system_fm,
            fallback_font_name: self.fallback_font_name,
            fallback: tf,
        })
    }
}

#[cfg(test)]
mod tests {
    use skia_safe::Typeface;

    use super::LocalDirFontMgr;
    use tracing::warn;

    fn init_test_logger() {
        use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
        let fmt = fmt::layer()
            .with_ansi(true)
            .with_file(true)
            .with_line_number(true);
        let filter = filter::LevelFilter::TRACE;
        let _ = tracing_subscriber::registry()
            .with(filter)
            .with(fmt)
            .try_init();
    }

    #[test]
    fn test_typeface_eq() {
        init_test_logger();

        let font_mgr = skia_safe::FontMgr::new();
        let sys_kai = font_mgr.match_family_style("楷体", skia_safe::FontStyle::default());
        // assert!(sys_kai.is_some());
        if sys_kai.is_none() {
            warn!("we are running on system that not have some default fonts");
            return;
        }
        let sys_kai = sys_kai.unwrap();
        if !std::fs::exists("simkai.ttf").unwrap() {
            warn!("font [simkai.ttf] not found]");
            return;
        }
        let bytes = std::fs::read("simkai.ttf").unwrap();

        let cur_kai = font_mgr.new_from_data(&bytes, 0);

        let cur_kai = cur_kai.unwrap();
        cur_kai.family_name();
        assert!(!Typeface::equal(sys_kai, cur_kai));
    }

    #[test]
    fn test_new_with_font_dir() {
        init_test_logger();
        let fm = LocalDirFontMgr::form_path("../fonts");
        let font_family_count = fm.font_cache.len();
        dbg!(font_family_count);
    }
}
