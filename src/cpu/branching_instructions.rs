use super::CPU;

impl CPU {
    // Branch instructions return a bool to let the main body know whether to skip consuming or current PC or not
    pub fn branch_if_carry_clear(&mut self) -> bool {
        if self.status.is_carry_set() { return false; }
        self.branch();
        return true;
    }

    pub fn branch_if_carry_set(&mut self) -> bool {
        if !self.status.is_carry_set() { return false; }
        self.branch();
        return true;
    }

    pub fn branch_if_equal(&mut self) -> bool {
        if !self.status.is_zero_set() { return false; }
        self.branch();
        return true;
    }

    pub fn branch_if_not_equal(&mut self) -> bool {
        if self.status.is_zero_set() { return false; }
        self.branch();
        return true;
    }

    pub fn branch_if_minus(&mut self) -> bool {
        if !self.status.is_negative_set() { return false; }
        self.branch();
        return true;
    }

    fn branch(&mut self) {
        let displacement = self.mem_read(self.program_counter) as i8; // cast as an i8 to retain signed value
        self.program_counter = self.program_counter
            .wrapping_add(1) // Consumes the current program counter. Make sure not to increment in main cpu cycle body
            .wrapping_add(displacement as u16); // casting to u16 will retain the binary value even when adding
    }
}

#[cfg(test)]
mod branching_tests {
    use super::*;

    #[test]
    pub fn branch_if_carry_clear() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0b1111_1101); // Subtracts 3 from program counter

        cpu.status.0 = 0b0000_0001; // Carry flat set, will not branch
        cpu.branch_if_carry_clear();
        assert_eq!(cpu.program_counter, 0x8000);

        cpu.status.0 = 0; // Cary flat not set, will branch
        cpu.branch_if_carry_clear();
        assert_eq!(cpu.program_counter, 0x7FFE);
    }

    #[test]
    pub fn branch_if_carry_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0b1111_1101); // Subtracts 3 from program counter

        cpu.status.0 = 0; // Carry flag not set, will not branch
        cpu.branch_if_carry_set();
        assert_eq!(cpu.program_counter, 0x8000);

        cpu.status.0 = 0b0000_0001; // Cary flag set, will branch
        cpu.branch_if_carry_set();
        assert_eq!(cpu.program_counter, 0x7FFE);
    }

    #[test]
    pub fn branch_if_equal() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0b1111_1101);

        cpu.status.0 = 0; // Zero flag not set, will not branch
        cpu.branch_if_equal();
        assert_eq!(cpu.program_counter, 0x8000);

        cpu.status.0 = 0b0000_0010; // Zero flag is set, will branch
        cpu.branch_if_equal();
        assert_eq!(cpu.program_counter, 0x7FFE);
    }

    #[test]
    pub fn branch_if_not_equal() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0b1111_1101); // Subtracts 3 from program counter

        cpu.status.0 = 0b0000_0010; // Zero flag is set, will not branch
        cpu.branch_if_not_equal();
        assert_eq!(cpu.program_counter, 0x8000);

        cpu.status.0 = 0;
        cpu.branch_if_not_equal();
        assert_eq!(cpu.program_counter, 0x7FFE);
    }

    #[test]
    pub fn branch_if_minus() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0b1111_1101);

        cpu.status.0 = 0; // Will not branch
        cpu.branch_if_minus();
        assert_eq!(cpu.program_counter, 0x8000);

        cpu.status.0 = 0b10000000;
        cpu.branch_if_minus();
        assert_eq!(cpu.program_counter, 0x7FFE);
    }
}