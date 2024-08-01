#[cfg(test)]
mod tests {
    use skia_safe::{Color, Color4f};

    #[test]
    fn test_skia_color() {
        let c = Color::RED;
        dbg!(c);
        let c4f: Color4f = c.into();
        dbg!(c4f);
    }
}
