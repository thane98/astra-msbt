use std::path::PathBuf;

use astra_formats::MessageBundle;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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
            let script = MessageBundle::load(&bundle)
                .expect("failed to parse bundle")
                .take_script()
                .expect("failed to parse script from MSBT");
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
            let mut msbt_bundle = MessageBundle::load(&bundle).expect("failed to load output bundle");
            msbt_bundle.replace_script(&script).expect("failed to serialize script");
            msbt_bundle.save(output.unwrap_or(bundle)).expect("failed to save output bundle");
        }
    }
}
