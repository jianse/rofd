use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use skia_safe::{FontStyle, Typeface};
use walkdir::WalkDir;

struct FontMgr {
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
        // log::debug!("init font manager,{}",font_dir.);
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
            log::debug!("try load font \"{}\"", entry.path().display());
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
                        log::debug!(
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
                    log::warn!("warn! load font error. path {}", path.display());
                }
            } else {
                log::warn!("warn! read file error. path {}", path.display());
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
            log::warn!(
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

    fn init_logger() {
        let e = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
        if e.is_err() {
            println!("warn! init logger error");
        }
    }

    #[test]
    fn test_typeface_eq() {
        let font_mgr = skia_safe::FontMgr::new();
        let sys_kai = font_mgr.match_family_style("楷体", skia_safe::FontStyle::default());
        assert!(sys_kai.is_some());
        let sys_kai = sys_kai.unwrap();
        if !std::fs::exists("simkai.ttf").unwrap(){
            log::warn!("font [simkai.ttf] not found]");
            return;
        }
        let bytes = std::fs::read("simkai.ttf").unwrap();

        let cur_kai = font_mgr.new_from_data(&bytes, 0);
        assert!(cur_kai.is_some());
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
