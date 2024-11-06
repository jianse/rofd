use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use skia_safe::{FontStyle, Typeface};
use walkdir::WalkDir;

use tracing::{debug, warn};

pub(super) struct FontMgr {
    system_font_mgr: skia_safe::FontMgr,
    font_cache: HashMap<String, FontStyleSet>,
}

impl FontMgr {
    /// create a new font manager
    /// only contains system fonts
    pub fn new() -> Self {
        Self {
            system_font_mgr: skia_safe::FontMgr::new(),
            font_cache: HashMap::new(),
        }
    }
    pub fn new_with_font_dir(font_dir: &PathBuf) -> Self {
        let sys_fm = skia_safe::FontMgr::new();
        let mut font_cache = HashMap::new();
        debug!("init font manager,{}", font_dir.display());
        for entry in WalkDir::new(font_dir)
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
                            .or_insert(FontStyleSet::new());
                        font_cache
                            .entry(family_name)
                            .and_modify(|e: &mut FontStyleSet| {
                                e.add_font(ff.clone());
                            });
                    }
                } else {
                    warn!("warn! load font error. path {}", path.display());
                }
            } else {
                warn!("warn! read file error. path {}", path.display());
            }
        }

        Self {
            system_font_mgr: sys_fm,
            font_cache,
        }
    }
    pub fn match_family_style(
        &self,
        family_name: impl AsRef<str>,
        style: FontStyle,
    ) -> Option<Typeface> {
        todo!()
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

    pub fn match_style(&mut self, pattern: FontStyle) -> Option<Typeface> {
        self.fonts
            .iter()
            .find(|i| i.font_style().eq(&pattern))
            .cloned()
    }
}
#[cfg(test)]
mod tests {
    use std::env;

    use skia_safe::Typeface;

    use super::FontMgr;
    use tracing::warn;

    fn init_logger() {
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
        init_logger();

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
        init_logger();
        let mut cur_dir = env::current_dir().unwrap();
        cur_dir.push("fonts");
        println!("--{}--", cur_dir.display());
        let fm = FontMgr::new_with_font_dir(&cur_dir);
        let font_family_count = fm.font_cache.len();
        dbg!(font_family_count);
    }
}
