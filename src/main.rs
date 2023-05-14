use std::path::PathBuf;

use astra_formats::MessageBundle;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    names: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Extract {
        #[arg(short, long)]
        output: Option<PathBuf>,
        bundle: PathBuf,
    },
    Import {
        #[arg(short, long)]
        output: Option<PathBuf>,
        script: PathBuf,
        bundle: PathBuf,
    },
}

pub fn main() {
    let args = Args::parse();
    match args.command {
        Command::Extract { bundle, output } => {
            let msbt = MessageBundle::load(&bundle)
                .expect("failed to parse bundle")
                .take_msbt();
            let script = astra_formats::parse_msbt_script(&msbt)
                .expect("failed to extract script from MSBT");
            if let Some(path) = output {
                std::fs::write(path, script).expect("failed to write script");
            } else {
                let mut path = bundle
                    .file_name()
                    .expect("could not identify output file name")
                    .to_string_lossy()
                    .to_string();
                path.push_str(".txt");
                std::fs::write(path, script).expect("failed to write script");
            }
        }
        Command::Import { script, bundle, output } => {
            let script = std::fs::read_to_string(script).expect("failed to read script");
            match astra_formats::pack_astra_script(&script) {
                Ok(msbt) => {
                    let mut msbt_bundle = MessageBundle::load(&bundle).expect("failed to load output bundle");
                    msbt_bundle.replace_msbt(msbt);
                    msbt_bundle.save(output.unwrap_or(bundle)).expect("failed to save output bundle");
                }
                Err(err) => err.report(&script),
            }
        }
    }
}
