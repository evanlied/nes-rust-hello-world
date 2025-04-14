mod addr_register;
mod control_register;

use addr_register::AddrRegister;
use control_register::ControlRegister;

use crate::{rom::Rom, Mirroring};

pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
    pub addr_register: AddrRegister,
    pub control_register: ControlRegister,
}

impl PPU {
    pub fn from_rom(rom: Rom) -> Self {
        PPU {
            chr_rom: rom.chr_rom,
            mirroring: rom.screen_mirroring,
            vram: [0; 2048],
            oam_data: [0; 256],
            palette_table: [0; 32],
            addr_register: AddrRegister::new(),
            control_register: ControlRegister::new(),
        }
    }

    pub fn write_to_ppu_addr(&mut self, addr: u8) {
        self.addr_register.update(addr);
    }

    pub fn write_to_control_register(&mut self, value: u8) {
        self.control_register.update(value);
    }
}