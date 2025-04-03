use crate::MemAccess;

const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const PPU_START: u16 = 0x2000;
const PPU_END: u16 = 0x3FFF;

pub struct Bus {
    pub cpu_vram: [u8; 2048],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu_vram: [0; 2048],
        }
    }

    fn get_mapped_addr(addr: u16) -> u16 {
        match addr {
            RAM_START..=RAM_END => {
                addr & 0b0000_0111_1111_1111
            },
            PPU_START..=PPU_END => {
                todo!("PPU not supported yet")
            },
            _ => {
                println!("Invalid RAM access at {:#x}", addr);
                0
            }
        }
    }
}

impl MemAccess for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        let mapped_addr = Self::get_mapped_addr(addr);
        self.cpu_vram[mapped_addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        let mapped_addr = Self::get_mapped_addr(addr);
        self.cpu_vram[mapped_addr as usize] = data;
    }

    fn bulk_write(&mut self, start: usize, end: usize, program: Vec<u8>) {
        self.cpu_vram[start..end].copy_from_slice(&program);
    }
}