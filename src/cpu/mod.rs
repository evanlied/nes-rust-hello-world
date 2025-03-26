mod load_instructions;
mod store_instructions;
mod arithmetic_instructions;
mod logical_instructions;
mod addressing_modes;
mod opcodes;

use opcodes::OP_CODE_REF_TABLE;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        // 0x8000 is the start of the 6502 ROM Addresses
        self.program_counter = 0x8000;
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr);
        let hi = self.mem_read(addr + 1);
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let lo = (data & 0x00FF) as u8;
        let hi = (data >> 8) as u8;
        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);
    }

    pub fn run(&mut self) {
        loop {
            let op_code = self.mem_read(self.program_counter);
            let op_code_params = OP_CODE_REF_TABLE.get(&op_code)
                .expect(&format!("${op_code} is not a valid operation"));
            self.program_counter += 1;
            match op_code_params.instruction {
                "AND" => {
                    self.and(op_code_params.addressing_mode.clone());
                    self.program_counter += op_code_params.bytes - 1;
                },
                "LDA" => {
                    self.load_register_a(op_code_params.addressing_mode.clone());
                    self.program_counter += op_code_params.bytes - 1;
                },
                "STA" => {
                    self.store_register_a(op_code_params.addressing_mode.clone());
                    self.program_counter += op_code_params.bytes - 1;
                },
                "TAX" => self.transfer_a_to_x(),
                "INX" => self.increment_x(),
                "BRK" => return,
                _=> println!("TODO for ${op_code}"), 
            }
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
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
        cpu.load(test_program);

        cpu.run();
        assert_eq!(cpu.register_a, 0x15);
        assert_eq!(cpu.register_x, 0x16);
        assert_eq!(cpu.status, 0b0000_0000);
        assert_eq!(cpu.program_counter, 0x8005);
    }

    #[test]
    pub fn store_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA9, 0x15, 0x85, 0x15, 0x00));

        assert_eq!(cpu.mem_read(0x15), 0x15);
        assert_eq!(cpu.program_counter, 0x8005);
    }

    #[test]
    pub fn and_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA9, 0b0001111, 0x29, 0b11111010, 0x00));

        assert_eq!(cpu.register_a, 0b00001010);
        assert_eq!(cpu.program_counter, 0x8005);
    }

    #[test]
    pub fn read_and_write_little_endian_memory() {
        let mut cpu = CPU::new();
        // cpu.program_counter = 0x8000;

        cpu.mem_write_u16(0x8000, 0xABCD);
        assert_eq!(cpu.mem_read_u16(0x8000), 0xABCD);
    }

    #[test]
    pub fn reset_cpu() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x99;
        cpu.register_x = 0xAA;
        cpu.status = 0xAB;
        cpu.memory[0xFFFC] = 0xCD;
        cpu.memory[0xFFFD] = 0xAB;

        cpu.reset();
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.status, 0);
        assert_eq!(cpu.program_counter, 0xABCD);
    }
}