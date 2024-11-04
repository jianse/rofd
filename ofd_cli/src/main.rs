mod error;
mod ofd_utils;

use clap::{command, Parser, Subcommand};
use cli_table::{print_stdout, WithTitle};
use eyre::Result;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// get information from ofd file
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
        #[arg(short, long, default_value_os_t = PathBuf::from("output"))]
        out_path: PathBuf,

        /// doc index
        #[arg(default_value_t = 0)]
        doc_index: usize,

        /// page index
        #[arg(default_value_t = 0)]
        page_index: usize,

        /// only render template page
        #[arg(short, long, default_value_t = false)]
        template: bool,
    },
}

fn init_logger() {
    let e = env_logger::builder()
        // Include info events
        // .filter_level(log::LevelFilter::Info)
        // Ignore errors initializing the logger if tests race to configure it
        .try_init();
    if e.is_err() {
        println!("warn! init logger error");
    }
}

fn main() -> Result<()> {
    init_logger();
    let ops = Cli::parse();
    match ops.command {
        Commands::Info { ofd_file } => {
            let info = ofd_utils::get_info(&ofd_file)?;
            // dbg!(info);
            println!("This ofd has {} document(s).", info.doc_count);
            print_stdout(info.doc_info.with_title())?;
        }
        Commands::Render {
            ofd_file,
            out_path,
            doc_index,
            page_index,
            template,
        } => {
            ofd_utils::render_page(&ofd_file, &out_path, doc_index, page_index, template)?;
        }
    }

    Ok(())
}
