use byteorder::{ReadBytesExt, LittleEndian};

use std::io::{self, prelude::*};

#[derive(Debug)]
pub enum Error {
}

#[derive(Debug)]
pub struct NdsHeader {
    pub title: [u8; 0xc],
    pub game_code: [u8; 0x4],
    pub creator_code: [u8; 0x2],
    pub unit_code: u8,
    pub encryption_seed: u8,
    pub capacity: u32,
    pub version: u8,
    pub auto_start: u8,
    pub arm9_rom_offset: u32,
    pub arm9_entry_address: u32,
    pub arm9_ram_address: u32,
    pub arm9_size: u32,
    pub arm7_rom_offset: u32,
    pub arm7_entry_address: u32,
    pub arm7_ram_address: u32,
    pub arm7_size: u32,
    pub file_table_offset: u32,
    pub file_table_size: u32,
    pub file_alloc_offset: u32,
    pub file_alloc_size: u32,
    pub arm9_overlay_offset: u32,
    pub arm9_overlay_size: u32,
    pub arm7_overlay_offset: u32,
    pub arm7_overlay_size: u32,
    pub command_port_normal: u32,
    pub command_port_key1: u32,
    pub icon_title_offset: u32,
    pub secure_checksum: u16,
    pub secure_loading_timeout: u16,
    pub arm9_auto_load: u32,
    pub arm7_auto_load: u32,
    pub secure_area_disable: u64,
    pub used_size: u32,
    pub header_size: u32,
}

impl NdsHeader {
    pub fn load(mut c: &[u8]) -> Result<Self, io::Error> {
        let mut title = [0; 0xc];
        c.read_exact(&mut title)?;

        let mut game_code = [0; 0x4];
        c.read_exact(&mut game_code)?;

        let mut creator_code = [0; 0x2];
        c.read_exact(&mut creator_code)?;

        let unit_code = c.read_u8()?;
        let encryption_seed = c.read_u8()?;
        let capacity = c.read_u32::<LittleEndian>()?;
        c.consume(0x6);

        let res = Self {
            title,
            game_code,
            creator_code,
            unit_code,
            encryption_seed,
            capacity,
            version: c.read_u8()?,
            auto_start: c.read_u8()?,
            arm9_rom_offset: c.read_u32::<LittleEndian>()?,
            arm9_entry_address: c.read_u32::<LittleEndian>()?,
            arm9_ram_address: c.read_u32::<LittleEndian>()?,
            arm9_size: c.read_u32::<LittleEndian>()?,
            arm7_rom_offset: c.read_u32::<LittleEndian>()?,
            arm7_entry_address: c.read_u32::<LittleEndian>()?,
            arm7_ram_address: c.read_u32::<LittleEndian>()?,
            arm7_size: c.read_u32::<LittleEndian>()?,
            file_table_offset: c.read_u32::<LittleEndian>()?,
            file_table_size: c.read_u32::<LittleEndian>()?,
            file_alloc_offset: c.read_u32::<LittleEndian>()?,
            file_alloc_size: c.read_u32::<LittleEndian>()?,
            arm9_overlay_offset: c.read_u32::<LittleEndian>()?,
            arm9_overlay_size: c.read_u32::<LittleEndian>()?,
            arm7_overlay_offset: c.read_u32::<LittleEndian>()?,
            arm7_overlay_size: c.read_u32::<LittleEndian>()?,
            command_port_normal: c.read_u32::<LittleEndian>()?,
            command_port_key1: c.read_u32::<LittleEndian>()?,
            icon_title_offset: c.read_u32::<LittleEndian>()?,
            secure_checksum: c.read_u16::<LittleEndian>()?,
            secure_loading_timeout: c.read_u16::<LittleEndian>()?,
            arm9_auto_load: c.read_u32::<LittleEndian>()?,
            arm7_auto_load: c.read_u32::<LittleEndian>()?,
            secure_area_disable: c.read_u64::<LittleEndian>()?,
            used_size: c.read_u32::<LittleEndian>()?,
            header_size: c.read_u32::<LittleEndian>()?,
        };

        Ok(res)
    }
}
