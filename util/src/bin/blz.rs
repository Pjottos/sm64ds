use util::blz;

use clap::{Parser, Subcommand};

use std::{
    fs,
    io::{self, prelude::*},
    path::PathBuf,
};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
    #[clap(short)]
    in_path: Option<PathBuf>,
    #[clap(short)]
    out_path: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    Extract { offset: usize },
}

fn main() {
    let args = Args::parse();

    let input = if let Some(in_path) = args.in_path {
        fs::read(in_path).expect("failed to read input file")
    } else {
        let stdin = io::stdin();
        let mut buf = vec![];
        stdin
            .lock()
            .read_to_end(&mut buf)
            .expect("failed to read input from stdin");

        buf
    };

    let output = match args.command {
        Command::Extract { offset } => blz::extract(input, offset),
    };

    if let Some(out_path) = args.out_path {
        fs::write(out_path, output).expect("failed to write output file");
    } else {
        let stdout = io::stdout();
        stdout
            .lock()
            .write_all(&output)
            .expect("failed to write output to stdout");
    }
}
