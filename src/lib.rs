pub mod cpu;
pub mod bus;

pub trait MemAccess {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);
}