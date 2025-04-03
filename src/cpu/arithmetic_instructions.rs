use super::{addressing_modes::AddressingMode, CPU};
use super::MemAccess;

impl CPU {
    pub fn add_with_carry(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let param = self.mem_read(addr);
        let carry: u16 = if self.status.is_carry_set() { 1 } else { 0 };
        let result: u16 = (self.register_a as u16) + (param as u16) + carry;
        let normalized_result = (result % 256) as u8;

        self.register_a = normalized_result;
        self.status.set_carry_flag(result > 255);
        self.status.set_negative_and_zero_flag(normalized_result);
    }

    pub fn subtract_with_carry(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let param = self.mem_read(addr);
        let carry = if self.status.is_carry_set() { 1 } else { 0 };
        let result: u16 = (self.register_a.wrapping_add(carry) as u16) 
            + ((param as i8).wrapping_neg() as u8) as u16;
        let normalized_result = (result % 256) as u8;

        self.register_a = normalized_result;
        self.status.set_carry_flag(result > 255);
        self.status.set_negative_and_zero_flag(normalized_result);
    }

    pub fn increment_x(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.status.set_negative_and_zero_flag(self.register_x);
    }

    pub fn decrement_x(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.status.set_negative_and_zero_flag(self.register_x);
    }

    pub fn increment_y(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.status.set_negative_and_zero_flag(self.register_y);
    }

    pub fn decrement_y(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.status.set_negative_and_zero_flag(self.register_y);
    }

    pub fn increment_mem(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let param = self.mem_read(addr);
        let result = param.wrapping_add(1);

        self.mem_write(addr, result);
        self.status.set_negative_and_zero_flag(result);
    }

    pub fn decrement_mem(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let param = self.mem_read(addr);
        let result = param.wrapping_sub(1);

        self.mem_write(addr, result);
        self.status.set_negative_and_zero_flag(result);
    }

    pub fn arithmetic_shift_left(&mut self, mode: &AddressingMode) {
        // For AddressingMode Accumulator, shift and overwrite reg A, otherwise shift and overwrite in memory
        let (old_val, new_val) = match mode {
            AddressingMode::Accumulator => {
                let old_val = self.register_a;
                self.register_a = old_val << 1;
                (old_val, self.register_a)
            },
            _ => {
                let addr = self.get_operand_address(mode);
                let old_val = self.mem_read(addr);
                let new_val = old_val << 1;
                self.mem_write(addr, new_val);
                (old_val, new_val)
            }
        };

        self.status.set_carry_flag(old_val & 0b1000_0000 != 0);
        self.status.set_negative_and_zero_flag(new_val);
    }

    pub fn compare(&mut self, mode: &AddressingMode) {
        self._compare(mode, self.register_a);
    }

    pub fn compare_x(&mut self, mode: &AddressingMode) {
        self._compare(mode, self.register_x);
    }

    pub fn compare_y(&mut self, mode: &AddressingMode) {
        self._compare(mode, self.register_y);
    }

    fn _compare(&mut self, mode: &AddressingMode, reg_val: u8) {
        let addr = self.get_operand_address(&mode);
        let param = self.mem_read(addr);
        self.status.set_carry_flag(reg_val >= param);

        let result = reg_val.wrapping_sub(param);
        self.status.set_negative_and_zero_flag(result);
    }
}

#[cfg(test)]
mod arithmetic_test {
    use super::*;

    #[test]
    pub fn add_with_carry() {
        let mut cpu = CPU::new();
        cpu.register_a = 250;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 10);
        cpu.add_with_carry(&AddressingMode::Immediate);
        assert_eq!(cpu.register_a, (250 as u8).wrapping_add(10));
        assert_eq!(cpu.status.0, 0b0000_0001);

        // Second addition now that carry bit is set will add an additional 1
        cpu.mem_write(0x8000, 22);
        cpu.add_with_carry(&AddressingMode::Immediate);
        assert_eq!(cpu.register_a, 27);
        assert_eq!(cpu.status.0, 0b0);

        // Third addition to check how zero result is handled
        cpu.mem_write(0x8000, 229);
        cpu.add_with_carry(&AddressingMode::Immediate);
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.status.0, 0b000_0011);
    }

    #[test]
    pub fn subtract_with_cary() {
        let mut cpu = CPU::new();
        cpu.register_a = 5;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 10);
        cpu.subtract_with_carry(&AddressingMode::Immediate);
        assert_eq!(cpu.register_a, 251);
        assert_eq!(cpu.status.0, 0b1000_0000);

        cpu.register_a = (125 as i8).wrapping_neg() as u8;
        println!("{:#8b}", cpu.register_a);
        cpu.mem_write(0x8000, 10);
        cpu.subtract_with_carry(&AddressingMode::Immediate);
        assert_eq!(cpu.register_a, 121);
        assert_eq!(cpu.status.0, 0b0000_0001);
    }

    #[test]
    pub fn increment_x_test() {
        let mut cpu = CPU::new();
        cpu.register_x = 0b01111111;
        cpu.increment_x();
        assert_eq!(cpu.register_x, 0b10000000);
        assert_eq!(cpu.status.0, 0b10000000);
    }

    #[test]
    pub fn decrement_x_test() {
        let mut cpu = CPU::new();
        cpu.register_x = 155;
        cpu.decrement_x();
        assert_eq!(cpu.register_x, 154);
        assert_eq!(cpu.status.0, 0b1000_0000);
    }

    #[test]
    pub fn increment_y_test() {
        let mut cpu = CPU::new();
        cpu.register_y = 0xFF;
        cpu.increment_y();
        assert_eq!(cpu.register_y, 0);
        assert_eq!(cpu.status.0, 0b0000_0010);
    }

    #[test]
    pub fn decrement_y_test() {
        let mut cpu = CPU::new();
        cpu.register_y = 55;
        cpu.decrement_y();
        assert_eq!(cpu.register_y, 54);
        assert_eq!(cpu.status.0, 0b0000_0000);
    }

    #[test]
    pub fn increment_memory_test() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xDC);
        cpu.mem_write(0xDC, 0xFF);
        cpu.increment_mem(&AddressingMode::ZeroPage);
        assert_eq!(cpu.mem_read(0xDC), 0x0);
        assert_eq!(cpu.status.0, 0b0000_0010);
    }

    #[test]
    pub fn decrement_memory_test() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_x = 0x2;
        cpu.mem_write(0x8000, 0xA9);
        cpu.mem_write(0xAB, 0x1);
        cpu.decrement_mem(&AddressingMode::ZeroPageX);
        assert_eq!(cpu.mem_read(0xAB), 0);
        assert_eq!(cpu.status.0, 0b0000_0010);
    }

    #[test]
    pub fn arithmetic_shift_left_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b11111001;
        cpu.arithmetic_shift_left(&AddressingMode::Accumulator);
        assert_eq!(cpu.register_a, 0b1111_0010);
        assert_eq!(cpu.status.0, 0b1000_0001);

        cpu.mem_write(0xAA, 0b1000_0000); // value to shift
        cpu.mem_write(0x8000, 0xAA); // zero page memory address
        cpu.program_counter = 0x8000;
        cpu.arithmetic_shift_left(&AddressingMode::ZeroPage);
        assert_eq!(cpu.mem_read(0xAA), 0b0000_0000);
        assert_eq!(cpu.status.0, 0b0000_0011);
    }

    #[test]
    pub fn compare_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xA0;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xAB);
        cpu.mem_write(0x00AB, 0x15);
        cpu.compare(&AddressingMode::ZeroPage);

        assert_eq!(cpu.status.0, 0b1000_0001);
    }

    #[test]
    pub fn compare_x_test() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x15;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0xA0);
        cpu.compare_x(&AddressingMode::Immediate);
        
        assert_eq!(cpu.status.0, 0b0000_0000);
    }
    
    #[test]
    pub fn compare_y_test() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x15;
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0x10);
        cpu.compare_y(&AddressingMode::Immediate);

        assert_eq!(cpu.status.0, 0b0000_0001);
    }

}