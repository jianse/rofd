#[cfg(test)]
mod test_skia {
    use std::{fs::File, io::Write};

    use skia_safe::{
        Color, Color4f, Font, FontMgr, Image, ImageInfo, Paint, Path, Point, TextBlob, Typeface,
    };

    use super::*;
    use eyre::{OptionExt, Result};

    #[test]
    fn test_triangle() -> Result<()> {
        let ii = ImageInfo::new_s32((300, 300), skia_safe::AlphaType::Opaque);
        let mut surface = skia_safe::surfaces::raster(&ii, None, None).ok_or_eyre("message")?;
        let canvas = surface.canvas();
        let mut path = Path::new();
        path.move_to((10, 10))
            .line_to((10, 60))
            .line_to((60, 35))
            .close();
        let color: Color4f = Color::from_rgb(255, 0, 0).into();

        let paint = Paint::new(color, None);
        canvas.draw_path(&path, &paint);
        // canvas.
        let img = surface.image_snapshot();
        let data = img
            .encode(None, skia_safe::EncodedImageFormat::PNG, 100)
            .ok_or_eyre("message")?;
        // data.
        let mut out = File::create("output/res.png")?;
        out.write(&data)?;

        Ok(())
    }

    #[test]
    fn test_resize() -> Result<()> {
        let ii = ImageInfo::new_s32((300, 300), skia_safe::AlphaType::Opaque);
        let mut surface = skia_safe::surfaces::raster(&ii, None, None).ok_or_eyre("message")?;
        let canvas = surface.canvas();
        canvas.save();
        canvas.scale((2.0, 2.0));

        let mut path = Path::new();
        path.move_to((10, 10))
            .line_to((10, 60))
            .line_to((60, 35))
            .close();
        let color: Color4f = Color::from_rgb(255, 0, 0).into();

        let paint = Paint::new(color, None);
        canvas.draw_path(&path, &paint);
        canvas.restore();
        let img = surface.image_snapshot();
        save_image(img, "output/res2.png")?;
        Ok(())
    }

    #[test]
    fn test_text() -> Result<()> {
        let ii = ImageInfo::new_s32((300, 300), skia_safe::AlphaType::Unpremul);
        let mut surface = skia_safe::surfaces::raster(&ii, None, None).ok_or_eyre("message")?;
        let canvas = surface.canvas();

        let mut paint = Paint::new(Color4f::new(1.0, 0.0, 0.0, 1.0), None);
        paint.set_style(skia_safe::PaintStyle::StrokeAndFill);
        let pos: Vec<Point> = vec![
            (0.0, 0.0).into(),
            (20.0, 0.0).into(),
            (40, 0).into(),
            (60, 0).into(),
            (120, 0).into(),
        ];
        // let typeface = Typeface::from_data();
        let fm = FontMgr::new();
        let fc = fm.count_families();
        for index in 0..fc {
            let fss = fm.new_style_set(index);
            // dbg!(fss.);
            let family_name = fm.family_name(index);
            dbg!(family_name);
        }
        let mut ff = fm.match_family("Noto Mono");
        let ff = ff.new_typeface(0).unwrap();
        // return Ok(());
        // let ff = fm.new_from_data("Noto Mono".as_bytes(),None).unwrap();

        let font = Font::new(ff, Some(20.0));
        // let font = dbg!(font);
        let blob = TextBlob::from_pos_text("hello", &pos, &font).unwrap();
        dbg!(blob.bounds());
        canvas.draw_text_blob(blob, (10, 50), &paint);

        let image = surface.image_snapshot();
        save_image(image, "output/test_text.png")
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_font_mgr() {
        let fm = FontMgr::new();
        let fc = fm.count_families();
        for index in 0..fc {
            let fss = fm.new_style_set(index);
            // dbg!(fss.);
            let family_name = fm.family_name(index);
            dbg!(family_name);
        }
        let mut fss = fm.match_family("楷体");
        for index in 0..fss.count() {
            let ff = fss.new_typeface(index).unwrap();
            let family_name = ff.new_family_name_iterator().for_each(|f| {
                dbg!(f);
            });
            dbg!(family_name, ff);
        }
    }
    #[test]
    #[cfg(target_os = "windows")]
    fn test_kaiti() {
        let fm = FontMgr::new();
        let mut fss = fm.match_family("KaiTi");
        for index in 0..fss.count() {
            let ff = fss.new_typeface(index).unwrap();
            ff.new_family_name_iterator().for_each(|f| {
                dbg!(f);
            });
            dbg!(ff);
        }
    }

    fn save_image(image: Image, path: &str) -> Result<()> {
        // write out
        let data = image
            .encode(None, skia_safe::EncodedImageFormat::PNG, 100)
            .ok_or_eyre("message")?;
        // data.
        let mut out = File::create(path)?;
        out.write(&data)?;

        Ok(())
    }
}
