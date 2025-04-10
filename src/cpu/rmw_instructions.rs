// RMW stands for read-modify-write

use super::{addressing_modes::AddressingMode, CPU, MemAccess};

impl CPU {
    pub fn decrement_compare_a(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let param = self.mem_read(addr);
        let result = param.wrapping_sub(1);
        self.mem_write(addr, result);

        self.status.set_carry_flag(self.register_a >= result);
        self.status.set_negative_and_zero_flag(self.register_a.wrapping_sub(result));
    }
}

#[cfg(test)]
mod rmw_tests {
    use super::*;

    #[test]
    pub fn dcp_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x69;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAB);
        cpu.mem_write(0xAB, 0x6A);
        cpu.decrement_compare_a(&AddressingMode::ZeroPage);
        assert_eq!(cpu.mem_read(0xAB), 0x69);
        assert_eq!(cpu.status.0, 0b0010_0111);
    }
}