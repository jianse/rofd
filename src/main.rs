mod container;
mod element;
mod custom_de;
mod ofd_utils;
use std::path::PathBuf;

use clap::{command, Parser};
use eyre::Result;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    ofd_file: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    // #[command(subcommand)]
    // command: Option<Commands>,
}

fn main() -> Result<()> {
    let ops = Cli::parse();
    let info = ofd_utils::get_info(&ops.ofd_file)?;
    dbg!(info);
    Ok(())
}
#[cfg(test)]
mod test_zip {
    use std::{fs::File, io::BufReader};

    // use super::*;
    use eyre::Result;

    #[test]
    fn test_zip() -> Result<()> {
        let f = File::open("test.ofd")?;
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
