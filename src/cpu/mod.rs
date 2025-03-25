mod load_instructions;
mod arithmetic_instructions;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        // CPU will be given a program which is a sequence of instruction codes
        // It will read through the instruction codes and decode the instruction while performing them
        
        // Set the program counter to 0 to start at the beginning of the program
        self.program_counter = 0;

        // while let Some(op_code) = program.get(self.program_counter as usize) {
        loop {
            let op_code = *program.get(self.program_counter as usize).expect("Tried to access out of bound program code");
            match op_code {
                0xA9 => self.load_register_a(&program),
                0xAA => self.transfer_a_to_x(),
                0xE8 => self.increment_x(),
                0x00 => return,
                _=> println!("TODO for ${op_code}"), 
            }

            self.program_counter += 1;
        }
    }

    pub fn set_status_flag(&mut self, value: u8) {
        // If the value is zero set the zero flag to 1 otherwise set it to 0
        if value == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        // If the value is negative, set the negative flag to 1 otherwise set it to 0
        if value & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
}

#[cfg(test)]
mod cpu_tests {
    use super::*;

    #[test]
    pub fn simple_program() {
        let test_program: Vec<u8> = vec!(0xa9, 0x15, 0xaa, 0xe8, 0x00);
        let mut cpu = CPU::new();

        cpu.interpret(test_program);
        assert_eq!(cpu.register_a, 0x15);
        assert_eq!(cpu.register_x, 0x16);
        assert_eq!(cpu.status, 0b0000_0000);
        assert_eq!(cpu.program_counter, 4);
    }

    #[test]
    pub fn lda_zero_flag_status() {
        let test_program: Vec<u8> = vec!(0xa9, 0x00, 0x00);
        let mut cpu = CPU::new();

        cpu.interpret(test_program);
        assert_eq!(cpu.status, 0b0000_0010);
    }

    #[test]
    pub fn lda_negative_flag_status() {
        let test_program: Vec<u8> = vec!(0xa9, 0xc0, 0x00);
        let mut cpu = CPU::new();

        cpu.interpret(test_program);
        assert_eq!(cpu.status, 0b1000_0000);
    }
}