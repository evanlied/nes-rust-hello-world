mod addr_register;

use crate::{rom::Rom, Mirroring};

pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
}

impl PPU {
    pub fn from_rom(rom: Rom) -> Self {
        PPU {
            chr_rom: rom.chr_rom,
            mirroring: rom.screen_mirroring,
            vram: [0; 2048],
            oam_data: [0; 256],
            palette_table: [0; 32],
        }
    }
}