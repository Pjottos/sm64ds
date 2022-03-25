use crate::header::NdsHeader;

use ascii::AsciiString;
use byteorder::{LittleEndian, ReadBytesExt};

use std::io::{self, prelude::*};

#[derive(Debug, Clone)]
pub struct File {
    name: FileName,
    data: Vec<u8>,
}

impl File {
    pub fn name(&self) -> &FileName {
        &self.name
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Debug, Clone)]
pub enum FileName {
    Overlay(usize),
    Name(AsciiString),
}

#[derive(Debug, Clone)]
pub struct Directory {
    name: AsciiString,
    entries: Vec<Entry>,
}

impl Directory {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn iter(&self) -> impl Iterator<Item = Entry> + '_ {
        self.entries.iter().copied()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FileId(u16);

impl FileId {
    fn as_idx(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DirId(u16);

impl DirId {
    fn as_idx(self) -> usize {
        (self.0 & !0xf000) as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Entry {
    File(FileId),
    Directory(DirId),
}

#[derive(Debug, Clone)]
pub struct FileTable {
    directories: Vec<Directory>,
    files: Vec<File>,
}

impl FileTable {
    pub fn load(rom: &[u8], header: &NdsHeader) -> Result<Self, io::Error> {
        let mut meta_start = rom;
        meta_start.consume(header.file_name_table_offset as usize);

        let mut meta_entries = meta_start;

        let root_offset = meta_entries.read_u32::<LittleEndian>()?;
        let root_file_id = FileId(meta_entries.read_u16::<LittleEndian>()?);
        let total_dir_count = meta_entries.read_u16::<LittleEndian>()? as usize;
        // Directory ids are in range 0xF000..=0xFFFF
        if total_dir_count > 0x1000 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid total directory count",
            ));
        }

        let mut alloc_start = rom;
        alloc_start.consume(header.file_alloc_table_offset as usize);

        let mut overlay_alloc = alloc_start;
        let mut files = (0..root_file_id.as_idx())
            .map(|i| {
                Self::read_file_data(rom, &mut overlay_alloc).map(|data| File {
                    name: FileName::Overlay(i),
                    data,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let root = Directory {
            name: AsciiString::from_ascii(b"root".to_vec()).unwrap(),
            entries: vec![],
        };

        let mut directories = Vec::with_capacity(total_dir_count as usize);
        directories.push(root);

        let mut root_meta = meta_start;
        root_meta.consume(root_offset as usize);
        let mut dir_stack = vec![(0, root_meta, root_file_id.as_idx())];

        'dirs: while let Some((i, mut meta, mut file_idx)) = dir_stack.pop() {
            loop {
                let flags = meta.read_u8()?;
                let name_len = (flags & !0x80) as usize;
                if name_len == 0 {
                    continue 'dirs;
                }

                let mut name = vec![0; name_len];
                meta.read_exact(&mut name)?;
                let name = AsciiString::from_ascii(name).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "file table entry name is not valid ascii",
                    )
                })?;

                if name == "." || name == ".." {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "file table entry name is \".\" or \"..\"",
                    ));
                }

                if flags & 0x80 != 0 {
                    let offset = meta_entries.read_u32::<LittleEndian>()?;
                    let first_file_id = FileId(meta_entries.read_u16::<LittleEndian>()?);

                    let parent_id = DirId(meta_entries.read_u16::<LittleEndian>()?);
                    if parent_id.as_idx() >= directories.len() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "directory has invalid parent id",
                        ));
                    }

                    let dir_id = DirId(meta.read_u16::<LittleEndian>()?);
                    if dir_id.as_idx() != directories.len() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "encountered nonsequential directory id while walking filesystem tree",
                        ));
                    }

                    let next_dir = Directory {
                        name,
                        entries: vec![],
                    };
                    directories.push(next_dir);

                    let next_i = dir_id.as_idx();
                    let mut next_meta = meta_start;
                    next_meta.consume(offset as usize);

                    directories[i].entries.push(Entry::Directory(dir_id));
                    dir_stack.push((i, meta, file_idx));
                    dir_stack.push((next_i, next_meta, first_file_id.as_idx()));
                    continue 'dirs;
                } else {
                    let mut alloc = alloc_start;
                    alloc.consume(file_idx * 8);
                    let data = Self::read_file_data(rom, &mut alloc)?;

                    let name = FileName::Name(name);
                    let file = File { name, data };
                    let id = FileId(file_idx.try_into().unwrap());
                    file_idx += 1;

                    files.push(file);
                    directories[i].entries.push(Entry::File(id));
                };
            }
        }

        Ok(Self { directories, files })
    }

    pub fn root(&self) -> &Directory {
        self.directories.get(0).expect("no root directory")
    }

    pub fn dir(&self, id: DirId) -> &Directory {
        self.directories.get(id.as_idx()).expect("invalid DirId")
    }

    pub fn file(&self, id: FileId) -> &File {
        self.files.get(id.as_idx()).expect("invalid FileId")
    }

    pub fn checked_file_id(&self, raw: u16) -> Option<FileId> {
        let res = FileId(raw);

        (res.as_idx() < self.files.len()).then(|| res)
    }

    fn read_file_data(rom: &[u8], alloc: &mut &[u8]) -> Result<Vec<u8>, io::Error> {
        let data_start = alloc.read_u32::<LittleEndian>()? as usize;
        let data_end = alloc.read_u32::<LittleEndian>()? as usize;

        // Files must not be in the secure area, and of course inside the rom.
        if data_start < 0x8000 || data_start > data_end || data_end > rom.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid file data address",
            ));
        }

        Ok(rom[data_start..data_end].to_vec())
    }
}
