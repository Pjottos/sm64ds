use crate::{
    file_table::{FileName, FileTable},
    overlay::OverlayTable,
};
use overlay::Overlay;
use util::blz;

use clap::{Parser, Subcommand};

use std::{fs, path::PathBuf};

mod file_table;
mod header;
mod overlay;

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
        Command::Extract {
            nds_path,
            mut out_path,
        } => {
            let rom = fs::read(nds_path).expect("failed to read nds file");
            let header = header::NdsHeader::load(&rom).expect("failed to parse header");
            // println!("header dump: {:x?}", header);

            let arm9 = checked_rom_range(&rom, header.arm9_rom_offset, header.arm9_size, "arm9");
            let compressed_end = header.arm9_rom_offset + header.arm9_size;
            let mut extract_buf = rom.clone();
            let extracted_range = blz::extract(&mut extract_buf, compressed_end as usize);
            let arm9_offset = header.arm9_rom_offset as usize;
            let arm9_extracted: Vec<_> = rom
                .iter()
                .copied()
                .skip(arm9_offset)
                .take(extracted_range.start - arm9_offset)
                .chain((&extract_buf[extracted_range]).iter().copied())
                .collect();

            let arm7 = checked_rom_range(&rom, header.arm7_rom_offset, header.arm7_size, "arm7");

            out_path.push("bin");
            fs::create_dir_all(&out_path).expect("failed to create bin dir");
            write_output(&mut out_path, "arm9.bin", arm9);
            write_output(&mut out_path, "arm9_extracted.bin", &arm9_extracted);
            write_output(&mut out_path, "arm7.bin", arm7);
            out_path.pop();

            let file_table = FileTable::load(&rom, &header).expect("failed to load file table");

            let overlay_table = OverlayTable::load(&rom, &header, &file_table)
                .expect("failed to load overlay table");
            // println!("overlay dump: {:#x?}", overlay_table);

            write_overlays(
                &mut out_path,
                "arm9_overlays",
                &file_table,
                overlay_table.arm9_overlays(),
            );
            write_overlays(
                &mut out_path,
                "arm7_overlays",
                &file_table,
                overlay_table.arm7_overlays(),
            );

            let mut dir_stack = vec![file_table.root().iter()];
            out_path.push("fs");
            fs::create_dir_all(&out_path).expect("failed to create fs dir");
            'dirs: while let Some(mut entries) = dir_stack.pop() {
                for entry in &mut entries {
                    match entry {
                        file_table::Entry::Directory(id) => {
                            let dir = file_table.dir(id);
                            dir_stack.push(entries);
                            dir_stack.push(dir.iter());
                            out_path.push(dir.name());
                            fs::create_dir_all(&out_path).expect("failed to create fs subdir");

                            continue 'dirs;
                        }
                        file_table::Entry::File(id) => {
                            let file = file_table.file(id);
                            let name = match file.name() {
                                FileName::Name(name) => name.as_str(),
                                FileName::Overlay(_) => unreachable!(),
                            };
                            write_output(&mut out_path, name, file.data())
                        }
                    }
                }

                out_path.pop();
            }
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
    // println!("Writing: {}", path.as_os_str().to_string_lossy());
    fs::write(&path, content).expect("failed to write output");
    path.pop();
}

fn write_overlays(
    path: &mut PathBuf,
    dir_name: &str,
    file_table: &FileTable,
    overlays: &[Overlay],
) {
    path.push(dir_name);
    fs::create_dir_all(&path).expect("failed to create overlay dir");

    for overlay in overlays {
        let file = file_table.file(overlay.file_id());
        let name = match file.name() {
            FileName::Overlay(num) => format!("overlay{}.bin", num),
            FileName::Name(_) => unreachable!(),
        };
        write_output(path, &name, file.data());
    }

    path.pop();
}
