use std::{fs::File, io::BufReader, path::Path};

use eyre::Result;
use quick_xml::{events::Event, Reader};
use serde::Deserialize;

#[test]
fn test_read_from_file() {
    let path = "samples/ano/OFD.xml";
    // let file = File::open(path).unwrap();
    let mut reader = quick_xml::Reader::from_file(path).unwrap();
    let mut buf = Vec::new();
    let mut level = 0;
    let mut start_ele = false;
    let mut last_ev = None;
    loop {
        let ev = reader.read_event_into(&mut buf);

        match ev.as_ref() {
            Ok(Event::Start(e)) => {
                start_ele = true;
                let (name, _) = e.name().decompose();
                println!(
                    "{}{} ",
                    "  ".repeat(level),
                    String::from_utf8_lossy(name.into_inner())
                );

                e.attributes()
                    .into_iter()
                    .filter_map(|a| a.ok())
                    .for_each(|a| {
                        let (name, _) = a.key.decompose();
                        println!(
                            "{}{}=\"{}\" ",
                            "  ".repeat(level),
                            String::from_utf8_lossy(name.into_inner()),
                            String::from_utf8_lossy(a.value.as_ref())
                        );
                    });
                level += 1;
            }
            Ok(Event::End(_)) => {
                start_ele = false;
                level -= 1;

                // println!("{:?}", e);
            }
            Ok(Event::Text(e)) => {
                if start_ele {
                    let text = e.unescape().unwrap();
                    println!("{}text=\"{}\"", "  ".repeat(level), text);
                } else {
                    let text = e.unescape().unwrap();
                    println!("outer text=\"{}\"", text)
                }
            }
            Ok(Event::Eof) => {
                break;
            }
            Err(_) => todo!(),
            e => {
                println!("fallback {:?}", e);
            }
        };
        last_ev = Some(ev.unwrap());
    }
    // dbg!(&buf);
    println!("---\n{}\n", String::from_utf8_lossy(&buf));
}

#[derive(Debug, Deserialize)]
struct Text {
    #[serde(rename = "$value")]
    text: String,
}

#[test]
fn test_de() -> Result<()> {
    let xml = r#"<Text> </Text>"#;
    let txt: Text = quick_xml::de::from_str(xml)?;
    let txt = dbg!(txt);
    assert_eq!(txt.text, " ");
    Ok(())
}

struct PrefetchReader<'a> {
    reader: Reader<BufReader<File>>,
    buf: Vec<u8>,
    event_buf: Vec<Event<'a>>,
}

impl<'a> PrefetchReader<'a> {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let reader = Reader::from_file(path)?;

        Ok(Self {
            reader,
            buf: Vec::new(),
            event_buf: Vec::new(),
        })
    }
    fn prefetch_event(&'a mut self) -> Result<Event<'a>> {
        let ev = self.reader.read_event_into(&mut self.buf)?;
        self.event_buf.push(ev.clone());
        Ok(ev)
    }

    fn read_event(&'a mut self) -> Result<Event<'a>> {
        if self.event_buf.is_empty() {
            Ok(self.reader.read_event_into(&mut self.buf)?)
        } else {
            let event = self.event_buf.remove(0);
            Ok(event)
        }
    }
}
