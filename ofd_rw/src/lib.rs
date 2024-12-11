mod container;
pub mod error;

pub use container::*;

#[allow(unused)]
const OFD_NS: &str = "http://www.ofdspec.org/2016";

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    // use super::*;
    use eyre::Result;

    #[test]
    fn test_zip() -> Result<()> {
        let f = File::open("../samples/000.ofd")?;
        let reader = BufReader::new(f);
        let zip = zip::ZipArchive::new(reader)?;

        let idx = zip.index_for_name("OFD.xml");
        if idx.is_none() {
            println!("OFD entry point not found!!");
            return Ok(());
        }
        for name in zip.file_names() {
            println!("Filename:{}", name)
        }

        // for i in 0..zip.len() {
        //     let mut file = zip.by_index(i)?;
        //     println!("Filename: {}", file.name());
        //     // std::io::copy(&mut file, &mut std::io::stdout())?;
        // }

        Ok(())
    }
}
