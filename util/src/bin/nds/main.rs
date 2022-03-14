use clap::{Parser, Subcommand};

use std::{fs, path::PathBuf};

mod header;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Extract {
        nds_path: PathBuf,
        out_path: PathBuf,
    },
    Build {
        root: PathBuf,
        out_path: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Extract { nds_path, out_path } => {
            let content = fs::read(nds_path).expect("failed to read nds file");
            let header = header::NdsHeader::load(&content).expect("failed to parse header");
        }
        _ => todo!(),
    }
}
