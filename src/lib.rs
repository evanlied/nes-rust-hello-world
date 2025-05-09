pub mod cpu;
pub mod bus;
pub mod rom;
pub mod ppu;
pub mod format_test;

#[derive(PartialEq, Clone)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

pub trait MemAccess {
    fn mem_read(&mut self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.mem_read(addr);
        let hi = self.mem_read(addr.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let lo = (data & 0x00FF) as u8;
        let hi = (data >> 8) as u8;
        self.mem_write(addr, lo);
        self.mem_write(addr.wrapping_add(1), hi);
    }
}