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
    internal_data_buffer: u8,
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
            internal_data_buffer: 0,
        }
    }

    pub fn write_to_ppu_addr(&mut self, addr: u8) {
        self.addr_register.update(addr);
    }

    pub fn write_to_control_register(&mut self, value: u8) {
        self.control_register.update(value);
    }

    fn increment_vram_addr(&mut self) {
        let increment_amount = self.control_register.get_vram_increment_size();
        self.addr_register.increment(increment_amount);
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.addr_register.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1FFF => {
                let result = self.internal_data_buffer;
                self.internal_data_buffer = self.chr_rom[addr as usize];
                result
            },
            0x2000..=0x2FFF => {
                let result = self.internal_data_buffer;
                let mirrored_addr = self.mirror_vram_addr(addr);
                self.internal_data_buffer = self.vram[mirrored_addr as usize];
                result
            },
            0x3000..=0x3EFF => panic!("0x3000 - 0x3eff addresses are not expected to be used, requested {addr:04X}"),
            0x3F00..=0x3FFF => self.palette_table[(addr - 0x3F00) as usize],
            _ => panic!("Unexpected read to a mirrored addressed {addr:04X}")
        }
    }

    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let normalized_addr = addr & 0x2FFF; // Mirros down 0x3000 - 0x3FFF addr to 0x2000 - 0x2FFF range

        match normalized_addr {
            0x2000..0x2400 => addr - 0x2400,
            0x2400..0x2800 => {
                let offset = if self.mirroring == Mirroring::Horizontal { 0 } else { 0x400 };
                addr - 0x2400 + offset
            },
            0x2800..0x2C00 => {
                let offset = if self.mirroring == Mirroring::Vertical { 0 } else { 0x400 };
                addr - 0x2800 + offset
            },
            0x2C00..0x3000 => {
                addr - 0x2C00 + 400
            },
            _ => panic!("{addr:04X} cannot be mirrored onto VRAM")
        }
    }
}