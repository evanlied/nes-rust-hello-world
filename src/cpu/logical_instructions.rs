use super::{addressing_modes::AddressingMode, CPU};

impl CPU {
    pub fn and(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let param = self.mem_read(addr);
        self.register_a = self.register_a & param;
        self.set_status_flag(self.register_a);
    }
}

#[cfg(test)]
mod logical_tests {
    use super::*;

    #[test]
    pub fn logical_and_immediate() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0110_1001;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0b0110_0110);
        cpu.and(AddressingMode::Immediate);
        assert_eq!(cpu.register_a, 0b0110_0000);
    }

    #[test]
    pub fn logical_and_zeropage() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0110_1001;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAC);
        cpu.mem_write(0x00AC, 0b0110_0110);
        cpu.and(AddressingMode::ZeroPage);
        assert_eq!(cpu.register_a, 0b0110_0000);
    }
}