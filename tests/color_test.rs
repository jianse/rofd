#[cfg(test)]
mod tests {
    use skia_safe::{Color, Color4f};

    #[test]
    fn test_cmyk_to_rgb() {
        let c = colorsys::Cmyk::new(0.0, 0.0, 90.0, 21.57, Some(1.0));
        let c = dbg!(c);
        let r = c.as_rgb();

        dbg!(r);
    }

    #[test]
    fn test_skia_color() {
        let c = Color::RED;
        dbg!(c);
        let c4f: Color4f = c.into();
        dbg!(c4f);
    }
}
