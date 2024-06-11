use std::io::BufRead;

use eyre::Result;
use quick_xml::{events::Event, NsReader};

#[allow(dead_code)]
fn parse<R: BufRead>(mut reader: NsReader<R>) -> Result<()> {
    // reader.trim_text(true);
    let conf = reader.config_mut();
    conf.trim_text(true);
    // let mut reader = NsReader::from_file("sample/OFD.xml")?;
    // let mut reader = NsReader::from_str(xml.as_str());
    let mut buf = vec![];
    loop {
        // if !reader.decoder()
        match reader.read_event_into(&mut buf).unwrap() {
            Event::Decl(decl) => {
                dbg!(decl);
            }
            Event::Start(e) => {
                let (ns, local) = reader.resolve_element(e.name());
                dbg!(ns);
                dbg!(local);
            }
            Event::Text(text) => {
                dbg!(text);
            }
            Event::End(e) => {
                dbg!(e);
                break;
            }
            _ => {
                //ignore
            }
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_file() -> Result<()> {
        let reader = NsReader::from_file("sample/OFD.xml")?;
        parse(reader)?;
        Ok(())
    }
    #[test]
    fn test_parse_str() -> Result<()> {
        let reader = NsReader::from_str("<hello>world!</hello>");
        parse(reader)?;
        Ok(())
    }
}
