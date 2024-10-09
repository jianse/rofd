#[cfg(test)]
mod tests {
    use quick_xml::{events::Event, Reader};
    use skia_safe::{Color, Color4f};

    #[test]
    fn test_skia_color() {
        let c = Color::RED;
        dbg!(c);
        let c4f: Color4f = c.into();
        dbg!(c4f);
    }

    #[test]
    fn test_de_from_reader() {
        let xml = r#"<Text> &lt;</Text>"#;
        // let r = BufReader::new(xml);
        let mut reader = Reader::from_reader(xml.as_bytes());
        // reader.
        // let mut buf = Vec::new();
        loop {
            match reader.read_event().unwrap() {
                Event::Eof => break,
                Event::Start(e) => {
                    println!("START\t{:?}", e.name());
                }
                Event::End(e) => {
                    println!("END  \t{:?}", e.name());
                }
                Event::Text(e) => {
                    println!("TEXT \t{:?}", e);
                }
                _ => {}
            }
        }
    }
}
