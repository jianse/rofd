#[cfg(test)]
mod test_skia {
    use std::{
        fs::File,
        io::{BufReader, Read, Write},
    };

    use skia_safe::{Color, Color4f, FontMgr, FontStyle, Image, ImageInfo, Paint, Path, Typeface};

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
    #[cfg(target_os = "linux")]
    #[ignore = "not yet implemented"]
    fn test_text() -> Result<()> {
        use skia_safe::{Font, Point, TextBlob};

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

        let tf = fm
            .match_family_style("Noto Mono", FontStyle::normal())
            .unwrap();

        let font = Font::new(tf, Some(20.0));
        // let font = dbg!(font);
        let blob = TextBlob::from_pos_text("hello", &pos, &font).unwrap();
        // dbg!(blob.bounds());
        canvas.draw_text_blob(blob, (10, 50), &paint);

        // draw origin point as green
        let paint = Paint::new(Color4f::new(0.0, 1.0, 0.0, 1.0), None);
        canvas.draw_point((10.0, 50.0), &paint);

        let image = surface.image_snapshot();
        save_image(image, "output/test_text.png")
    }

    /// to passing this you need install simkai.ttf into your system
    #[test]
    fn test_kai() {
        let fm = FontMgr::new();

        let mut fss = fm.match_family("楷体");
        dbg!(fss.count());
        assert!(fss.count() > 0);

        let kaiti = fm.match_family_style("楷体", FontStyle::normal());
        assert!(kaiti.is_some());
        let kaiti = kaiti.unwrap();
        dbg!(kaiti.font_style());
    }
    #[test]
    #[ignore = "this should behind a feature"]
    fn fm_test() -> Result<()> {
        let fm = FontMgr::new();
        let file = File::open("simkai.ttf")?;
        let mut reader = BufReader::new(file);
        let mut buf = vec![];
        reader.read_to_end(&mut buf)?;
        let tf = fm.new_from_data(&buf, 0);
        // fm.fr
        // dbg!(tf.unwrap());
        let tf = tf.expect("can not open font file");
        let family_name = tf.family_name();
        let font_style = tf.font_style();
        // assert_eq!(font_style.weight(), 400);
        dbg!(font_style);
        for name in tf.new_family_name_iterator() {
            dbg!(name);
        }
        println!("family_name = {}", family_name);
        // assert_eq!()
        // dbg!(fm);
        Ok(())
    }

    #[test]
    #[ignore = "this should behind a feature"]
    fn test_typeface_load() -> Result<()> {
        let fm = FontMgr::empty();
        let file = File::open("simkai.ttf")?;
        // let mut reader = BufReader::new(file);
        // Typeface::fr
        let tf = Typeface::make_deserialize(file, Some(fm));
        dbg!(tf);
        Ok(())
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_font_mgr() {
        let fm = FontMgr::new();
        let fc = fm.count_families();
        for index in 0..fc {
            let fss = fm.new_style_set(index);
            dbg!(fss);
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
