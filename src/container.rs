use std::{fs::File, io::BufReader, path::PathBuf};

use eyre::{Ok, OptionExt, Result};
use zip::{read::ZipFile, ZipArchive};
pub struct Container {
    // container:
    // path:
    zip_archive: ZipArchive<BufReader<File>>,
}
impl Container {
    pub fn open(&mut self, path: &PathBuf) -> Result<ZipFile> {
        // todo!()
        let path_str = path.to_str().ok_or_eyre("pathBuf to str faild!!")?;
        let file = self.zip_archive.by_name(path_str)?;
        Ok(file)
    }
}

pub fn from_path(path: PathBuf) -> Result<Container> {
    // todo!()
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let zip = zip::ZipArchive::new(reader)?;

    zip.index_for_name("OFD.xml")
        .ok_or_eyre("OFD entry point not found!!")?;
    Ok(Container { zip_archive: zip })
}
