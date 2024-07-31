use std::{collections::HashMap, path::PathBuf};

use quick_xml::se;
use skia_safe::{FontStyle, Typeface};

struct FontMgr {
    system_font_mgr: skia_safe::FontMgr,
    font_cache: HashMap<String, Typeface>,
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
    pub fn new_with_font_dir(&self, font_dir: &PathBuf) -> Self {
        todo!()
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
    use skia_safe::Typeface;

    #[test]
    fn test_typeface_eq() {
        let font_mgr = skia_safe::FontMgr::new();
        let sys_kai = font_mgr.match_family_style("楷体", skia_safe::FontStyle::default());
        assert!(sys_kai.is_some());
        let sys_kai = sys_kai.unwrap();
        let bytes = std::fs::read("simkai.ttf").unwrap();

        let cur_kai = font_mgr.new_from_data(&bytes, 0);
        assert!(cur_kai.is_some());
        let cur_kai = cur_kai.unwrap();
        cur_kai.family_name();
        assert!(!Typeface::equal(sys_kai, cur_kai));
    }
}
