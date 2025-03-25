use super::CPU;

impl CPU {
    pub fn load_register_a(&mut self) {
        self.program_counter += 1;
        let param = &self.memory[self.program_counter as usize];
        self.register_a = *param;

        self.set_status_flag(*param);
    }

    pub fn transfer_a_to_x(&mut self) {
        self.register_x = self.register_a;
        self.set_status_flag(self.register_x);
    }
}

#[cfg(test)]
mod load_tests {
    use super::*;

    #[test]
    pub fn lda_zero_flag_status() {
        let test_program: Vec<u8> = vec!(0xa9, 0x00, 0x00);
        let mut cpu = CPU::new();
        cpu.load(test_program);

        cpu.run();
        assert_eq!(cpu.status, 0b0000_0010);
    }

    #[test]
    pub fn lda_negative_flag_status() {
        let test_program: Vec<u8> = vec!(0xa9, 0xc0, 0x00);
        let mut cpu = CPU::new();
        cpu.load(test_program);

        cpu.run();
        assert_eq!(cpu.status, 0b1000_0000);
    }
}
