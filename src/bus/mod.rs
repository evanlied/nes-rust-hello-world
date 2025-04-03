use crate::{rom::Rom, MemAccess};

const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const PPU_START: u16 = 0x2000;
const PPU_END: u16 = 0x3FFF;
const ROM_START: u16 = 0x8000;
const ROM_END: u16 = 0xFFFF;

pub struct Bus {
    pub cpu_vram: [u8; 2048],
    rom: Rom,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Bus {
            cpu_vram: [0; 2048],
            rom,
        }
    }

    pub fn empty() -> Self {
        Bus {
            cpu_vram: [0; 2048],
            rom: Rom {
                prg_rom: vec![0; 0x8000],
                chr_rom: vec![0; 1024],
                mapper: 0x0,
                screen_mirroring: crate::Mirroring::Horizontal
            }
        }
    }

    pub fn read_prg_rom(&self, addr: u16) -> u8 {
        let mut addr = addr - 0x8000;
        if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr = addr % 0x4000;
        }
        self.rom.prg_rom[addr as usize]
    }
}

impl MemAccess for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END => {
                let mapped_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mapped_addr as usize]
            },
            PPU_START..=PPU_END => {
                todo!("PPU not supported yet")
            },
            ROM_START..=ROM_END => self.read_prg_rom(addr),
            _ => {
                println!("Invalid RAM access at {:#x}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM_START..=RAM_END => {
                let mapped_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mapped_addr as usize] = data;
            },
            PPU_START..=PPU_END => {
                todo!("PPU not supported yet")
            },
            ROM_START..=ROM_END => {
                match cfg!(test) {
                    true => {
                        let mut addr = addr - 0x8000;
                        if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
                            addr = addr % 0x4000;
                        }
                        self.rom.prg_rom[addr as usize] = data;
                    },
                    false => panic!("Attempt to write to Cartridge ROM space"),
                }
            },
            _ => {
                println!("Invalid RAM access at {:#x}", addr);
            }
        }
    }

    fn bulk_write(&mut self, start: usize, end: usize, program: Vec<u8>) {
        self.cpu_vram[start..end].copy_from_slice(&program);
    }
}