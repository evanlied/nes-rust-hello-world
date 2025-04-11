// RMW stands for read-modify-write

use super::{addressing_modes::AddressingMode, CPU, MemAccess};
use super::arithmetic_instructions::is_sign_incorrect;

impl CPU {
    pub fn decrement_compare_a(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let param = self.mem_read(addr);
        let result = param.wrapping_sub(1);
        self.mem_write(addr, result);

        self.status.set_carry_flag(self.register_a >= result);
        self.status.set_negative_and_zero_flag(self.register_a.wrapping_sub(result));
    }

    pub fn increment_subtract_carry(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let param = self.mem_read(addr).wrapping_add(1);
        self.mem_write(addr, param);

        let neg_param = param.wrapping_neg().wrapping_sub(1);
        let carry = if self.status.is_carry_set() { 1 } else { 0 };
        let result: u16 = (self.register_a as u16).wrapping_add(carry) 
            .wrapping_add(neg_param as u16);
        let normalized_result = result as u8;
        self.status.set_carry_flag(result > 255);
        self.status.set_overflow_flag(is_sign_incorrect(normalized_result, self.register_a, neg_param));
        self.status.set_negative_and_zero_flag(normalized_result);
        self.register_a = normalized_result;
    }

    pub fn shift_left_or_a(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        let shifted = val << 1;
        let ored = self.register_a | shifted;

        self.mem_write(addr, shifted);
        self.register_a = ored;
        self.status.set_carry_flag(val & 0b1000_0000 != 0);
        self.status.set_negative_and_zero_flag(ored);
    }

    pub fn shift_right_eor_a(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        let shifted = val >> 1;
        let eored = self.register_a ^ shifted;

        self.mem_write(addr, shifted);
        self.register_a = eored;
        self.status.set_carry_flag(val &0b0000_0001 != 0);
        self.status.set_negative_and_zero_flag(eored);
    }

    pub fn rotate_left_and_a(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        let shifted = match self.status.is_carry_set() {
            true => val << 1 | 0b0000_0001,
            false => val << 1 & 0b1111_1110,
        };
        let anded = self.register_a & shifted;

        self.mem_write(addr, shifted);
        self.register_a = anded;
        self.status.set_carry_flag(val & 0b1000_0000 != 0);
        self.status.set_negative_and_zero_flag(anded);
    }

    pub fn rotate_right_add_a(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        let shifted = match self.status.is_carry_set() {
            true => val >> 1 | 0b1000_0000,
            false => val >> 1 & 0b0111_1111,
        };
        self.status.set_carry_flag(val & 0b0000_0001 != 0);
        let carry: u16 = if self.status.is_carry_set() { 1 } else { 0 };
        let added: u16 = (self.register_a as u16).wrapping_add(shifted as u16).wrapping_add(carry);
        let added_u8 = added as u8;

        self.status.set_carry_flag(added > 255);
        self.status.set_overflow_flag(is_sign_incorrect(added_u8, self.register_a, shifted));
        self.status.set_negative_and_zero_flag(added_u8);
        self.mem_write(addr, shifted);
        self.register_a = added_u8;
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

    #[test]
    pub fn isc_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xFF;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAB);
        cpu.mem_write(0xAB, 0xB);
        cpu.increment_subtract_carry(&AddressingMode::ZeroPage);
        assert_eq!(cpu.register_a, 0xF2);
        assert_eq!(cpu.mem_read(0xAB), 0xC);
        assert_eq!(cpu.status.0, 0b1010_0101);
    }

    #[test]
    pub fn slo_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x0F;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAB);
        cpu.mem_write(0xAB, 0xF);
        cpu.shift_left_or_a(&AddressingMode::ZeroPage);
        assert_eq!(cpu.register_a, 0b0001_1111);
        assert_eq!(cpu.mem_read(0xAB), 0b0001_1110);
        assert_eq!(cpu.status.0, 0b0010_0100);
    }

    #[test]
    pub fn sre_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xF;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAB);
        cpu.mem_write(0xAB, 0xF);
        cpu.shift_right_eor_a(&AddressingMode::ZeroPage);
        assert_eq!(cpu.register_a, 0b0000_1000);
        assert_eq!(cpu.mem_read(0xAB), 0b0000_0111);
        assert_eq!(cpu.status.0, 0b0010_0101);
    }

    #[test]
    pub fn rla_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xF;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAB);
        cpu.mem_write(0xAB, 0xF);
        cpu.rotate_left_and_a(&AddressingMode::ZeroPage);
        assert_eq!(cpu.register_a, 0b0000_1110);
        assert_eq!(cpu.mem_read(0xAB), 0b0001_1110);
        assert_eq!(cpu.status.0, 0b0010_0100);
    }

    #[test]
    pub fn rra_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xB2;
        cpu.program_counter = 0x8000;
        cpu.status.0 = 0xE4;
        cpu.mem_write(0x8000, 0xAB);
        cpu.mem_write(0xAB, 0xA5);
        cpu.rotate_right_add_a(&AddressingMode::ZeroPage);
        assert_eq!(cpu.register_a, 0x5);
        assert_eq!(cpu.mem_read(0xAB), 82);
        assert_eq!(cpu.status.0, 0b0010_0101);
    }
}