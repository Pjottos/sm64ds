use byteorder::{LittleEndian, ReadBytesExt};

use std::io::{self, prelude::*};

#[derive(Debug)]
pub struct NdsHeader {
    pub title: [u8; 0xc],
    pub game_code: [u8; 0x4],
    pub maker_code: [u8; 0x2],
    pub unit_code: u8,
    pub device_type: u8,
    pub device_size: u8,
    pub _pad1: [u8; 0x9],
    pub version: u8,
    pub flags: u8,
    pub arm9_rom_offset: u32,
    pub arm9_entry_address: u32,
    pub arm9_ram_address: u32,
    pub arm9_size: u32,
    pub arm7_rom_offset: u32,
    pub arm7_entry_address: u32,
    pub arm7_ram_address: u32,
    pub arm7_size: u32,
    pub file_name_table_offset: u32,
    pub file_name_table_size: u32,
    pub file_alloc_table_offset: u32,
    pub file_alloc_table_size: u32,
    pub arm9_overlay_offset: u32,
    pub arm9_overlay_size: u32,
    pub arm7_overlay_offset: u32,
    pub arm7_overlay_size: u32,
    pub command_port_normal: u32,
    pub command_port_key1: u32,
    pub icon_title_offset: u32,
    pub secure_area_crc: u16,
    pub secure_area_loading_timeout: u16,
    pub arm9_auto_loads_address: u32,
    pub arm7_auto_loads_address: u32,
    pub secure_area_disable: u64,
    pub used_size: u32,
    pub header_size: u32,
    pub _pad2: [u8; 0x38],
    pub logo: [u8; 0x9c],
    pub logo_crc: u16,
    pub header_crc: u16,
    pub _pad3: [u8; 0xA0],
}

impl NdsHeader {
    pub fn load(mut c: &[u8]) -> Result<Self, io::Error> {
        let mut title = [0; 0xc];
        c.read_exact(&mut title)?;

        let mut game_code = [0; 0x4];
        c.read_exact(&mut game_code)?;

        let mut maker_code = [0; 0x2];
        c.read_exact(&mut maker_code)?;

        let unit_code = c.read_u8()?;
        let device_type = c.read_u8()?;
        let device_size = c.read_u8()?;

        let mut _pad1 = [0; 0x9];
        c.read_exact(&mut _pad1)?;

        let version = c.read_u8()?;
        let flags = c.read_u8()?;
        let arm9_rom_offset = c.read_u32::<LittleEndian>()?;
        let arm9_entry_address = c.read_u32::<LittleEndian>()?;
        let arm9_ram_address = c.read_u32::<LittleEndian>()?;
        let arm9_size = c.read_u32::<LittleEndian>()?;
        let arm7_rom_offset = c.read_u32::<LittleEndian>()?;
        let arm7_entry_address = c.read_u32::<LittleEndian>()?;
        let arm7_ram_address = c.read_u32::<LittleEndian>()?;
        let arm7_size = c.read_u32::<LittleEndian>()?;
        let file_name_table_offset = c.read_u32::<LittleEndian>()?;
        let file_name_table_size = c.read_u32::<LittleEndian>()?;
        let file_alloc_table_offset = c.read_u32::<LittleEndian>()?;
        let file_alloc_table_size = c.read_u32::<LittleEndian>()?;
        let arm9_overlay_offset = c.read_u32::<LittleEndian>()?;
        let arm9_overlay_size = c.read_u32::<LittleEndian>()?;
        let arm7_overlay_offset = c.read_u32::<LittleEndian>()?;
        let arm7_overlay_size = c.read_u32::<LittleEndian>()?;
        let command_port_normal = c.read_u32::<LittleEndian>()?;
        let command_port_key1 = c.read_u32::<LittleEndian>()?;
        let icon_title_offset = c.read_u32::<LittleEndian>()?;
        let secure_area_crc = c.read_u16::<LittleEndian>()?;
        let secure_area_loading_timeout = c.read_u16::<LittleEndian>()?;
        let arm9_auto_loads_address = c.read_u32::<LittleEndian>()?;
        let arm7_auto_loads_address = c.read_u32::<LittleEndian>()?;
        let secure_area_disable = c.read_u64::<LittleEndian>()?;
        let used_size = c.read_u32::<LittleEndian>()?;
        let header_size = c.read_u32::<LittleEndian>()?;

        let mut _pad2 = [0; 0x38];
        c.read_exact(&mut _pad2)?;

        let mut logo = [0; 0x9c];
        c.read_exact(&mut logo)?;

        let logo_crc = c.read_u16::<LittleEndian>()?;
        let header_crc = c.read_u16::<LittleEndian>()?;

        let mut _pad3 = [0; 0xa0];
        c.read_exact(&mut _pad3)?;

        let res = Self {
            title,
            game_code,
            maker_code,
            unit_code,
            device_type,
            device_size,
            _pad1,
            version,
            flags,
            arm9_rom_offset,
            arm9_entry_address,
            arm9_ram_address,
            arm9_size,
            arm7_rom_offset,
            arm7_entry_address,
            arm7_ram_address,
            arm7_size,
            file_name_table_offset,
            file_name_table_size,
            file_alloc_table_offset,
            file_alloc_table_size,
            arm9_overlay_offset,
            arm9_overlay_size,
            arm7_overlay_offset,
            arm7_overlay_size,
            command_port_normal,
            command_port_key1,
            icon_title_offset,
            secure_area_crc,
            secure_area_loading_timeout,
            arm9_auto_loads_address,
            arm7_auto_loads_address,
            secure_area_disable,
            used_size,
            header_size,
            _pad2,
            logo,
            logo_crc,
            header_crc,
            _pad3,
        };

        Ok(res)
    }
}
