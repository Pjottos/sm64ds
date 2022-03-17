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
        Command::Extract { nds_path, mut out_path } => {
            let rom = fs::read(nds_path).expect("failed to read nds file");
            let header = header::NdsHeader::load(&rom).expect("failed to parse header");
            println!("{:x?}", header);

            let arm9 = checked_rom_range(&rom, header.arm9_rom_offset, header.arm9_size, "arm9");
            let arm7 = checked_rom_range(&rom, header.arm7_rom_offset, header.arm7_size, "arm7");
            let arm9_overlay = checked_rom_range(&rom, header.arm9_overlay_offset, header.arm9_overlay_size, "arm9 overlay");
            let arm7_overlay = checked_rom_range(&rom, header.arm7_overlay_offset, header.arm7_overlay_size, "arm7 overlay");

            out_path.push("bin");
            fs::create_dir_all(&out_path);
            write_output(&mut out_path, "arm9.bin", arm9);
            write_output(&mut out_path, "arm7.bin", arm7);
            write_output(&mut out_path, "arm9_overlay.bin", arm9_overlay);
            write_output(&mut out_path, "arm7_overlay.bin", arm7_overlay);
            out_path.pop();
        }
        _ => todo!(),
    }
}

fn checked_rom_range<'a>(rom: &'a [u8], offset: u32, size: u32, range_name: &str) -> &'a [u8] {
    let start = offset as usize;
    let end = start + size as usize;
    if start > end || end > rom.len() {
        panic!("invalid {range_name} offset and/or size");
    }
    &rom[start..end]
}

fn write_output(path: &mut PathBuf, file_name: &str, content: &[u8]) {
    path.push(file_name);
    fs::write(&path, content).expect("failed to write output");
    path.pop();
}
