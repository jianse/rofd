mod error;
mod ofd_utils;
mod util;

use clap::{command, Parser, Subcommand};
use cli_table::{print_stdout, WithTitle};
use eyre::Result;
use ofd_sign::gm::GenKeyPairReq;
use std::path::PathBuf;
use std::time::Duration;
use tracing::info;
use tracing_subscriber::EnvFilter;
use util::parse_duration;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
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

        /// output path template
        #[arg(short,long,default_value_t = String::from("{out_path}/{ofd_file_name}/Doc{doc_index}/Page{page_index}.{ext}"))]
        path_template: String,

        /// doc index
        #[arg()]
        doc_index: Option<usize>,

        /// page index
        #[arg()]
        page_index: Option<usize>,

        /// only render template page
        #[arg(short, long, default_value_t = false)]
        template: bool,
    },
    /// certificate commands
    Cert {
        #[command(subcommand)]
        command: CertCommands,
    },
}

#[derive(Subcommand)]
enum CertCommands {
    /// Generate sm2 keypair
    Gen {
        /// A path where the generated keypair stores.
        #[arg(short, long)]
        output: PathBuf,

        /// Public key output path
        #[arg(short, long)]
        pub_output: Option<PathBuf>,
    },
    /// Generate Csr
    Req {
        #[arg(short, long, value_parser= parse_duration)]
        valid_period: Duration,
    },
    /// X509 certificate
    X509 {},
    /// Electronic seal
    ESeal {},
}

fn init_logger(level: u8) {
    let f = match level {
        0 => "info".into(),
        1 => "debug".into(),
        _ => "trace".into(),
    };
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
    let fmt = fmt::layer()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true);
    let filter = EnvFilter::try_from_default_env().unwrap_or(f);
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt)
        .try_init();
}

fn main() -> Result<()> {
    let ops = Cli::parse();
    init_logger(ops.debug);
    match ops.command {
        Commands::Info { ofd_file } => {
            print_ofd_info(&ofd_file)?;
        }
        Commands::Render {
            ofd_file,
            out_path,
            doc_index,
            page_index,
            template,
            path_template,
        } => {
            info!("{}", path_template);
            if let Some(doc) = doc_index {
                if let Some(page) = page_index {
                    ofd_utils::render_page(
                        &ofd_file,
                        &out_path,
                        doc,
                        page,
                        template,
                        &path_template,
                    )?;
                } else {
                    ofd_utils::render_doc(&ofd_file, &out_path, doc, &path_template)?;
                }
            } else {
                ofd_utils::render_ofd(&ofd_file, &out_path, &path_template)?;
            }
        }
        Commands::Cert { command } => match command {
            CertCommands::Gen { output, pub_output } => {
                let mut req = GenKeyPairReq::new();
                req.mkdir(true).sk_path(output);
                if let Some(pk_path) = pub_output {
                    req.extract_pk(true).pk_path(pk_path);
                };

                req.generate()?;
            }
            CertCommands::Req { valid_period } => {
                dbg!(&valid_period);
            }
            CertCommands::X509 { .. } => {}
            CertCommands::ESeal { .. } => {}
        },
    }

    Ok(())
}

fn print_ofd_info(ofd_file: &PathBuf) -> Result<()> {
    let info = ofd_utils::get_info(ofd_file)?;
    // docs
    println!("This ofd has {} document(s).", info.doc_count);
    print_stdout(info.doc_info.with_title())?;

    // items in package
    println!(
        "This ofd has {} item(s) in it's package.",
        info.item_names.len()
    );
    for item in info.item_names.iter() {
        println!("{}", item);
    }
    Ok(())
}
