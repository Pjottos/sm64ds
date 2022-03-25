use crate::{
    file_table::{FileId, FileName, FileTable},
    header::NdsHeader,
};

use byteorder::{LittleEndian, ReadBytesExt};

use std::io;

#[derive(Debug, Clone, Copy)]
pub struct Overlay {
    address: u32,
    size: u32,
    pad_size: u32,
    file_id: FileId,
    static_initializer_start: u32,
    static_initializer_end: u32,
}

#[derive(Debug, Clone)]
pub struct OverlayTable {
    arm9_overlays: Vec<Overlay>,
    arm7_overlays: Vec<Overlay>,
}

impl OverlayTable {
    pub fn load(rom: &[u8], header: &NdsHeader, file_table: &FileTable) -> Result<Self, io::Error> {
        let arm9_overlays = Self::load_single(
            rom,
            header.arm9_overlay_offset,
            header.arm9_overlay_size,
            file_table,
        )?;
        let arm7_overlays = Self::load_single(
            rom,
            header.arm7_overlay_offset,
            header.arm7_overlay_size,
            file_table,
        )?;

        Ok(Self {
            arm9_overlays,
            arm7_overlays,
        })
    }

    fn load_single(
        rom: &[u8],
        offset: u32,
        size: u32,
        file_table: &FileTable,
    ) -> Result<Vec<Overlay>, io::Error> {
        let offset = offset as usize;
        let size = size as usize;

        if offset > rom.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid overlay table offset",
            ));
        }
        if size > rom.len() - offset {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid overlay table size",
            ));
        }
        let table = &rom[offset..offset + size];

        let mut res = vec![];

        for mut entry in table.chunks_exact(0x20) {
            let id = entry.read_u32::<LittleEndian>()?;
            if id as usize != res.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "overlay ids are nonsequential",
                ));
            }

            let address = entry.read_u32::<LittleEndian>()?;
            let size = entry.read_u32::<LittleEndian>()?;
            let pad_size = entry.read_u32::<LittleEndian>()?;
            let static_initializer_start = entry.read_u32::<LittleEndian>()?;
            let static_initializer_end = entry.read_u32::<LittleEndian>()?;
            let file_id = entry.read_u16::<LittleEndian>()?;

            if static_initializer_end < static_initializer_start {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "static_initializer_end is less than static_initializer_start",
                ));
            }

            let file_id = file_table
                .checked_file_id(file_id)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid file id"))
                .and_then(|id| match file_table.file(id).name() {
                    FileName::Overlay(_) => Ok(id),
                    FileName::Name(_) => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "overlay file id does not point to overlay file",
                    )),
                })?;

            res.push(Overlay {
                address,
                size,
                pad_size,
                static_initializer_start,
                static_initializer_end,
                file_id,
            });
        }

        Ok(res)
    }
}
