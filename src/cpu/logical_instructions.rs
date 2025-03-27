use super::{addressing_modes::AddressingMode, CPU};

impl CPU {
    pub fn and(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let param = self.mem_read(addr);
        self.register_a = self.register_a & param;
        self.status.set_negative_and_zero_flag(self.register_a);
        // self.set_status_flag(self.register_a);
    }

    pub fn bit_test(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let param = self.mem_read(addr);
        let result = param & self.register_a;

        self.status.set_zero_flag(result);
        self.status.set_overflow_flag(param);
        self.status.set_negative_flag(param);
    }

    pub fn exclusive_or(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let param = self.mem_read(addr);
        let result = self.register_a ^ param;
        self.register_a = result;
        self.status.set_negative_and_zero_flag(result);
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

    #[test]
    pub fn bit_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b00001111;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAB);
        cpu.mem_write(0x00AB, 0b11011001);
        cpu.bit_test(AddressingMode::ZeroPage);
        
        assert_eq!(cpu.status.0, 0b1100_0000);
    }

    #[test]
    pub fn eclusive_or_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1001_1001;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAB);
        cpu.mem_write(0xAB, 0b0000_1111);
        cpu.exclusive_or(AddressingMode::ZeroPage);
        assert_eq!(cpu.register_a, 0b1001_0110);
        assert_eq!(cpu.status.0, 0b1000_0000);
    }
}