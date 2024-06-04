mod container;
mod custom_de;
mod element;
mod error;
mod ofd_utils;
use std::path::PathBuf;

use clap::{command, Parser, Subcommand};
use eyre::Result;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    // ofd_file: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// get infomation from ofd file
    Info {
        /// file path
        ofd_file: PathBuf,
    },
    /// render an ofd file or page
    Render {
        /// file path
        // #[arg()]
        ofd_file: PathBuf,
        /// out put path
        #[arg(short,long ,default_value_os_t = PathBuf::from("output") )]
        out_path: PathBuf,

        #[arg(default_value_t = 0)]
        doc_index: usize,

        #[arg(default_value_t = 0)]
        template_index: usize,
    },
}

fn main() -> Result<()> {
    let ops = Cli::parse();
    match ops.command {
        Commands::Info { ofd_file } => {
            let info = ofd_utils::get_info(&ofd_file)?;
            dbg!(info);
        }
        Commands::Render {
            ofd_file,
            out_path,
            doc_index,
            template_index,
        } => {
            let _ = ofd_utils::render_template(&ofd_file, &out_path, doc_index, template_index)?;
        }
    }

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
