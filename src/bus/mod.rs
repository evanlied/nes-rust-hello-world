use crate::{rom::Rom, MemAccess, ppu::PPU};

const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
// const PPU_START: u16 = 0x2000;
const PPU_END: u16 = 0x3FFF;
const ROM_START: u16 = 0x8000;
const ROM_END: u16 = 0xFFFF;

pub struct Bus {
    pub cpu_vram: [u8; 2048],
    rom: Rom,
    ppu: PPU,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Bus {
            cpu_vram: [0; 2048],
            ppu: PPU::from_rom(&rom),
            rom,
        }
    }

    pub fn empty() -> Self {
        let rom = Rom {
            prg_rom: vec![0; 0x8000],
            chr_rom: vec![0; 1024],
            mapper: 0x0,
            screen_mirroring: crate::Mirroring::Horizontal
        };
        Self::new(rom)
    }

    pub fn load_rom(&mut self, rom: Rom) {
        self.rom = rom;
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
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END => {
                let mapped_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mapped_addr as usize]
            },
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Attempt to read from write-only PPU register {addr:04X}")
            },
            0x2007 => self.ppu.read_data(),
            0x2008..=PPU_END => {
                // any attempts at reading PPU data should be done through one of the registers 0x2000 - 0x2007
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_read(mirror_down_addr)
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
            0x2000 => self.ppu.write_to_control_register(data),
            0x2002 => panic!("Attempting to write to read only register 0x2002"),
            0x2006 => self.ppu.write_to_ppu_addr(data),
            0x2007 => self.ppu.write_to_ppu_data(data),
            0x2008..PPU_END => {
                // any attempts at writing PPU data should be done through one of the registers 0x2000 - 0x2007
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                self.mem_read(mirror_down_addr);
            }
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
}