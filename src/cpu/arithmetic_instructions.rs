use super::{addressing_modes::AddressingMode, CPU};

impl CPU {
    pub fn increment_x(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.status.set_negative_and_zero_flag(self.register_x);
    }

    pub fn arithmetic_shift_left(&mut self, mode: AddressingMode) {
        // For AddressingMode Accumulator, shift and overwrite reg A, otherwise shift and overwrite in memory
        let (old_val, mem_ptr): (u8, &mut u8) = match mode {
            AddressingMode::Accumulator => (self.register_a, &mut self.register_a),
            _ => {
                let addr = self.get_operand_address(&mode);
                (self.mem_read(addr), &mut self.memory[addr as usize])
            }
        };
        let new_val = old_val << 1;
        *mem_ptr = new_val;

        self.status.set_carry_flag(old_val & 0b1000_0000 != 0);
        self.status.set_negative_and_zero_flag(new_val);
    }

    pub fn compare(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let param = self.mem_read(addr);
        self.status.set_carry_flag(self.register_a >= param);

        let result = self.register_a.wrapping_sub(param);
        self.status.set_negative_and_zero_flag(result);
    }
}

#[cfg(test)]
mod arithmetic_test {
    use super::*;

    #[test]
    pub fn increment_x_test() {
        let mut cpu = CPU::new();
        cpu.register_x = 0b01111111;
        cpu.increment_x();
        assert_eq!(cpu.register_x, 0b10000000);
        assert_eq!(cpu.status.0, 0b10000000);
    }

    #[test]
    pub fn arithmetic_shift_left_test() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b11111001;
        cpu.arithmetic_shift_left(AddressingMode::Accumulator);
        assert_eq!(cpu.register_a, 0b1111_0010);
        assert_eq!(cpu.status.0, 0b1000_0001);

        cpu.mem_write(0xAA, 0b1000_0000); // value to shift
        cpu.mem_write(0x8000, 0xAA); // zero page memory address
        cpu.program_counter = 0x8000;
        cpu.arithmetic_shift_left(AddressingMode::ZeroPage);
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
        cpu.compare(AddressingMode::ZeroPage);

        assert_eq!(cpu.status.0, 0b1000_0001);
    }

}