#[cfg(test)]
mod test_skia {
    use std::{fs::File, io::Write};

    use skia_safe::{Color, Color4f, Image, ImageInfo, Paint, Path};

    use super::*;
    use eyre::{OptionExt, Result};

    #[test]
    fn test_triangle() -> Result<()> {
        let ii = ImageInfo::new_s32((300, 300), skia_safe::AlphaType::Opaque);
        let mut surface = skia_safe::surfaces::raster(&ii, None, None).ok_or_eyre("message")?;
        // let canvas = Canvas::f
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
        // let canvas = Canvas::f
        let canvas = surface.canvas();
        canvas.save();
        canvas.scale((2.0,2.0));

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
