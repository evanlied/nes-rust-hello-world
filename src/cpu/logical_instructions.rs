use super::{addressing_modes::AddressingMode, CPU};
use super::MemAccess;

impl CPU {
    pub fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let param = self.mem_read(addr);
        self.register_a = self.register_a & param;
        self.status.set_negative_and_zero_flag(self.register_a);
    }

    pub fn bit_test(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let param = self.mem_read(addr);
        let result = param & self.register_a;

        self.status.set_zero_flag(result);
        self.status.set_overflow_flag(param);
        self.status.set_negative_flag(param);
    }

    pub fn exclusive_or(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let param = self.mem_read(addr);
        let result = self.register_a ^ param;
        self.register_a = result;
        self.status.set_negative_and_zero_flag(result);
    }

    pub fn inclusive_or(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let param = self.mem_read(addr);
        self.register_a = self.register_a | param;
        self.status.set_negative_and_zero_flag(self.register_a);
    }

    fn perform_op_on_mem<F: FnMut(u8) -> u8>(&mut self, mode: &AddressingMode, mut op: F) -> (u8, u8) {
        match mode {
            AddressingMode::Accumulator => {
                let old_val = self.register_a;
                self.register_a = op(old_val);
                (old_val, self.register_a)
            },
            _ => {
                let addr = self.get_operand_address(mode);
                let old_val = self.mem_read(addr);
                let new_val = op(old_val);
                self.mem_write(addr, new_val);
                (old_val, new_val)
            }
        }
    }

    pub fn logical_shift_right(&mut self, mode: &AddressingMode) {
        let (old_val, new_val) = self
            .perform_op_on_mem(mode, |old_val| old_val >> 1);

        self.status.set_carry_flag(old_val & 0b0000_0001 != 0);
        self.status.set_zero_flag(new_val);
        self.status.set_negative_flag(new_val);
    }

    pub fn rotate_left(&mut self, mode: &AddressingMode) {
        let is_carry_set = self.status.is_carry_set();
        let (old_val, new_val) = self
            .perform_op_on_mem(mode, |old_val| {
                match is_carry_set {
                    true => old_val << 1 | 0b0000_0001,
                    false => old_val << 1 & 0b1111_1110
                }
            });

        self.status.set_carry_flag(old_val & 0b1000_0000 != 0);
        self.status.set_negative_and_zero_flag(new_val);
    }

    pub fn rotate_right(&mut self, mode: &AddressingMode) {
        let is_carry_set = self.status.is_carry_set();
        let (old_val, new_val) = self
            .perform_op_on_mem(mode, |old_val| {
                match is_carry_set {
                    true => old_val >> 1 | 0b1000_0000,
                    false => old_val >> 1 & 0b0111_1111
                }
            });

        self.status.set_carry_flag(old_val & 0b0000_0001 != 0);
        self.status.set_negative_and_zero_flag(new_val);
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
        cpu.and(&AddressingMode::Immediate);
        assert_eq!(cpu.register_a, 0b0110_0000);
        assert_eq!(cpu.status.0, 0b0010_0100);
        // Do another AND to make sure the status flag is set to 0
        cpu.mem_write(0x8000, 0);
        cpu.and(&AddressingMode::Immediate);
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.status.0, 0b0010_0110);
    }

    #[test]
    pub fn logical_and_zeropage() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1110_1001;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAC);
        cpu.mem_write(0x00AC, 0b1110_0110);
        cpu.and(&AddressingMode::ZeroPage);
        assert_eq!(cpu.register_a, 0b1110_0000);
        assert_eq!(cpu.status.0, 0b1010_0100);
    }

    #[test]
    pub fn bit_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b00001111;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAB);
        cpu.mem_write(0x00AB, 0b11011001);
        cpu.bit_test(&AddressingMode::ZeroPage);
        
        assert_eq!(cpu.status.0, 0b1110_0100);
    }

    #[test]
    pub fn exclusive_or_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1001_1001;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAB);
        cpu.mem_write(0xAB, 0b0000_1111);
        cpu.exclusive_or(&AddressingMode::ZeroPage);
        assert_eq!(cpu.register_a, 0b1001_0110);
        assert_eq!(cpu.status.0, 0b1010_0100);
    }

    #[test]
    pub fn  inclusive_or_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1010_0011;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0b1001_0001);
        cpu.inclusive_or(&AddressingMode::Immediate);
        assert_eq!(cpu.register_a, 0b1011_0011);
        assert_eq!(cpu.status.0, 0b1010_0100);
    }

    #[test]
    pub fn logical_shift_right() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x80, 0b0101_1111);
        cpu.mem_write(0x8000, 0x80);
        cpu.logical_shift_right(&AddressingMode::ZeroPage);
        assert_eq!(cpu.mem_read(0x80), 0b0010_1111);
        assert_eq!(cpu.status.0, 0b0010_0101);
    }

    #[test]
    pub fn rotate_left() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x70, 0b0000_1111);
        cpu.mem_write_u16(0x8000, 0x70);
        cpu.rotate_left(&AddressingMode::Absolute);
        assert_eq!(cpu.mem_read(0x70), 0b0001_1110);
        assert_eq!(cpu.status.0, 0b0010_0100);

        cpu.mem_write(0x70, 0b0);
        cpu.rotate_left(&AddressingMode::Absolute);
        assert_eq!(cpu.mem_read(0x70), 0);
        assert_eq!(cpu.status.0, 0b0010_0110);
    }

    #[test]
    pub fn rotate_right() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.status.set_carry_flag(true);
        cpu.mem_write(0x70, 0b0000_1111);
        cpu.mem_write_u16(0x8000, 0x70);
        cpu.rotate_right(&AddressingMode::Absolute);
        assert_eq!(cpu.mem_read(0x70), 0b1000_0111);
        assert_eq!(cpu.status.0, 0b1010_0101);

        cpu.mem_write(0x70, 0b0);
        cpu.status.set_carry_flag(false);
        cpu.rotate_right(&AddressingMode::Absolute);
        assert_eq!(cpu.mem_read(0x70), 0);
        assert_eq!(cpu.status.0, 0b0010_0110);
    }
}