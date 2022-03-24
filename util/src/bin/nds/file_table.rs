use crate::header::NdsHeader;

use ascii::AsciiString;
use byteorder::{LittleEndian, ReadBytesExt};

use std::io::{self, prelude::*};

#[derive(Debug, Clone)]
struct File {
    name: AsciiString,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Directory {
    name: AsciiString,
    first_file_idx: u16,
    files: Vec<File>,
    directories: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct FileTable {
    directories: Vec<(u16, Directory)>,
}

impl FileTable {
    pub fn load(rom: &[u8], header: &NdsHeader) -> Result<Self, io::Error> {
        let mut meta_start = rom;
        meta_start.consume(header.file_name_table_offset as usize);

        let mut meta_entries = meta_start;

        let mut root_meta_entry = meta_entries;
        root_meta_entry.consume(4);
        let root_file_idx = root_meta_entry.read_u16::<LittleEndian>()?;
        let total_dir_count = root_meta_entry.read_u16::<LittleEndian>()? as usize;
        // Directory ids are in range 0xF000..=0xFFFF
        if total_dir_count > 0x1000 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid total directory count",
            ));
        }

        let mut alloc_start = rom;
        alloc_start.consume(header.file_alloc_table_offset as usize);

        let mut directories = Vec::with_capacity(total_dir_count as usize);
        let root_dir = Directory {
            name: AsciiString::from_ascii(b"root".to_vec()).unwrap(),
            first_file_idx: root_file_idx,
            files: vec![],
            directories: vec![],
        };
        directories.push((0xf000, root_dir));

        for i in 0..total_dir_count {
            let offset = meta_entries.read_u32::<LittleEndian>()?;
            let first_file_idx = meta_entries.read_u16::<LittleEndian>()?;
            let parent_id = meta_entries.read_u16::<LittleEndian>()?;

            if i >= directories.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "not enough directories",
                ));
            };

            let mut meta = meta_start;
            meta.consume(offset as usize);
            loop {
                let flags = meta.read_u8()?;
                let name_len = (flags & !0x80) as usize;
                if name_len == 0 {
                    break;
                }

                let mut name = vec![0; name_len];
                meta.read_exact(&mut name)?;
                let name = AsciiString::from_ascii(name).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "file table entry name is not valid ascii",
                    )
                })?;

                if flags & 0x80 != 0 {
                    let dir_id = meta.read_u16::<LittleEndian>()?;
                    let dir = Directory {
                        name,
                        first_file_idx,
                        files: vec![],
                        directories: vec![],
                    };

                    directories.push((dir_id, dir));
                    directories[i].1.directories.push(dir_id);
                } else {
                    let mut alloc = alloc_start;
                    let file_idx = first_file_idx as usize + directories[i].1.files.len();
                    alloc.consume(file_idx * 8);
                    let data_start = alloc.read_u32::<LittleEndian>()? as usize;
                    let data_end = alloc.read_u32::<LittleEndian>()? as usize;

                    // Files must not be in the secure area, and of course inside the rom.
                    if data_start < 0x8000 || data_start > data_end || data_end > rom.len() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "invalid file data address",
                        ));
                    }
                    let data = rom[data_start..data_end].to_vec();

                    directories[i].1.files.push(File { name, data });
                };
            }
        }

        Ok(Self { directories })
    }
}
