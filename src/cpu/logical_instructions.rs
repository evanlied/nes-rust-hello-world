use super::{addressing_modes::AddressingMode, CPU};

impl CPU {
    pub fn and(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let param = self.mem_read(addr);
        self.register_a = self.register_a & param;
        self.status.set_carry_and_zero_flag(self.register_a);
        // self.set_status_flag(self.register_a);
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
        assert_eq!(cpu.status.0, 0);
        // Do another AND to make sure the status flag is set to 0
        cpu.mem_write(0x8000, 0);
        cpu.and(AddressingMode::Immediate);
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.status.0, 0b0000_0010);
    }

    #[test]
    pub fn logical_and_zeropage() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1110_1001;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAC);
        cpu.mem_write(0x00AC, 0b1110_0110);
        cpu.and(AddressingMode::ZeroPage);
        assert_eq!(cpu.register_a, 0b1110_0000);
        assert_eq!(cpu.status.0, 0b1000_0000);
    }
}