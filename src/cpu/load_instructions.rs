use super::CPU;
use super::addressing_modes::AddressingMode;
use super::MemAccess;

impl CPU {
    // LDA
    pub fn load_register_a(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let param = self.mem_read(addr);
        self.register_a = param;

        self.status.set_negative_and_zero_flag(param);
    }

    pub fn load_register_x(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let param = self.mem_read(addr);
        self.register_x = param;

        self.status.set_negative_and_zero_flag(param);
    }

    pub fn load_register_y(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let param = self.mem_read(addr);
        self.register_y = param;

        self.status.set_negative_and_zero_flag(param);
    }

    pub fn transfer_a_to_x(&mut self) {
        self.register_x = self.register_a;
        self.status.set_negative_and_zero_flag(self.register_a);
    }

    pub fn transfer_x_to_a(&mut self) {
        self.register_a = self.register_x;
        self.status.set_negative_and_zero_flag(self.register_a);
    }

    pub fn transfer_a_to_y(&mut self) {
        self.register_y = self.register_a;
        self.status.set_negative_and_zero_flag(self.register_a);
    }

    pub fn transfer_y_to_a(&mut self) {
        self.register_a = self.register_y;
        self.status.set_negative_and_zero_flag(self.register_a);
    }

    pub fn transfer_stack_pointer_to_x(&mut self) {
        self.register_x = self.stack_pointer;
        self.status.set_negative_and_zero_flag(self.register_x);
    }

    pub fn transfer_x_to_stack_pointer(&mut self) {
        self.stack_pointer = self.register_x;
    }
}

#[cfg(test)]
mod load_tests {
    use super::*;

    #[test]
    pub fn lda_zero_flag_status() {
        let test_program: Vec<u8> = vec!(0xa9, 0x00, 0x00);
        let mut cpu = CPU::new();
        cpu.load_and_run(test_program);
        assert_eq!(cpu.status.0, 0b0000_0010);
    }

    #[test]
    pub fn lda_negative_flag_status() {
        let test_program: Vec<u8> = vec!(0xa9, 0xc0, 0x00);
        let mut cpu = CPU::new();
        cpu.load_and_run(test_program);
        assert_eq!(cpu.status.0, 0b1000_0000);
    }

    #[test]
    pub fn ldx_test() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_y = 2;
        cpu.mem_write(0x8000, 0xA0);
        cpu.mem_write(0xA2, 0xDC);
        cpu.load_register_x(&AddressingMode::ZeroPageY);
        assert_eq!(cpu.register_x, 0xDC);
        assert_eq!(cpu.status.0, 0b1000_0000);
    }

    #[test]
    pub fn ldy_test() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_x = 2;
        cpu.mem_write_u16(0x8000, 0x700);
        cpu.mem_write(0x702, 0xC);
        cpu.load_register_y(&AddressingMode::AbsoluteX);
        assert_eq!(cpu.register_y, 0xC);
        assert_eq!(cpu.status.0, 0);
    }
}
