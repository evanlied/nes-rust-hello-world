use super::{addressing_modes::AddressingMode, CPU};
use super::MemAccess;

impl CPU {
    pub fn store_register_a(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    pub fn store_register_x(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    pub fn store_register_y(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }

    pub fn store_a_anded_x(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let val = self.register_a & self.register_x;
        self.mem_write(addr, val);
    }
}

#[cfg(test)]
mod store_test {
    use super::*;

    #[test]
    fn sta_zero_page() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x69;
        cpu.mem_write(0, 0xAB);
        
        cpu.store_register_a(&AddressingMode::ZeroPage);
        assert_eq!(cpu.mem_read(0xAB), 0x69);
    }

    #[test]
    fn stx_absolute() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x72;
        cpu.mem_write_u16(0, 0x700);
        cpu.store_register_x(&AddressingMode::Absolute);
        assert_eq!(cpu.mem_read(0x700), 0x72);
    }

    #[test]
    fn sty_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 5;
        cpu.register_y = 0x72;
        cpu.mem_write(0, 0x70);
        cpu.store_register_y(&AddressingMode::ZeroPageX);
        assert_eq!(cpu.mem_read(0x75), 0x72);
    }

    #[test]
    fn sax_absolute() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1010_1010;
        cpu.register_x = 0b1111_0000;
        cpu.program_counter = 0x8000;
        cpu.mem_write_u16(0x8000, 0x1A1A);
        cpu.store_a_anded_x(&AddressingMode::Absolute);
        assert_eq!(cpu.mem_read(0x1A1A), cpu.register_a & cpu.register_x);
    }
}
