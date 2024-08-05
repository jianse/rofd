#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader};

    use eyre::Result;
    use quick_xml::{events::Event, Reader};
    use serde::Deserialize;
    use skia_safe::{Color, Color4f};

    #[test]
    fn test_skia_color() {
        let c = Color::RED;
        dbg!(c);
        let c4f: Color4f = c.into();
        dbg!(c4f);
    }

    #[derive(Debug, Deserialize)]
    struct Text {
        #[serde(rename = "$text")]
        text: String,
    }

    #[test]
    fn test_de() -> Result<()> {
        let xml = r#"<Text> </Text>"#;
        let txt: Text = quick_xml::de::from_str(xml)?;
        dbg!(txt);
        Ok(())
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
