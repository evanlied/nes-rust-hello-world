mod load_instructions;
mod store_instructions;
mod branching_instructions;
mod arithmetic_instructions;
mod logical_instructions;
mod addressing_modes;
mod status_flags;
mod opcodes;

use opcodes::OP_CODE_REF_TABLE;
use status_flags::StatusFlag;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: StatusFlag,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: StatusFlag(0),
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
                "AND" => self.and(op_code_params.addressing_mode.clone()),
                "ASL" => self.arithmetic_shift_left(op_code_params.addressing_mode.clone()),
                "BCC" => if self.branch_if_carry_clear() { continue; },
                "BCS" => if self.branch_if_carry_set() { continue; },
                "BEQ" => if self.branch_if_equal() { continue; },
                "BIT" => self.bit_test(op_code_params.addressing_mode.clone()),
                "BMI" => if self.branch_if_minus() { continue; },
                "BNE" => if self.branch_if_not_equal() { continue; },
                "BPL" => if self.branch_if_positive() { continue; },
                "BVC" => if self.branch_if_overflow_clear() { continue; },
                "BVS" => if self.branch_if_overflow_set() { continue; },
                "CLC" => self.status.set_carry_flag(0),
                "LDA" => self.load_register_a(op_code_params.addressing_mode.clone()),
                "STA" => self.store_register_a(op_code_params.addressing_mode.clone()),
                "TAX" => self.transfer_a_to_x(),
                "INX" => self.increment_x(),
                "BRK" => return,
                _=> println!("TODO for ${op_code}"), 
            }
            self.program_counter += op_code_params.bytes - 1;
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = StatusFlag(0);
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
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
        assert_eq!(cpu.status.0, 0b0000_0000);
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
    pub fn asl_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA9, 0b1011_0001, 0x0A, 0x00));

        assert_eq!(cpu.register_a, 0b0110_0010);
        assert_eq!(cpu.program_counter, 0x8004);
    }

    #[test]
    pub fn bcc_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8001);
        cpu.load_and_run(vec!(0x00, 0x90, 0b1111_1101, 0x00)); // subtracts 3 from PC to get back to BRK command at 0x8000

        assert_eq!(cpu.program_counter, 0x8001);
    }

    #[test]
    pub fn bcs_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8001);
        cpu.load_and_run(vec!(0x00, 0xA9, 0b10000000, 0x0A, 0xB0, 0b1111_1010, 0x00)); // subtracts 6 from PC to get back to BRK command at 0x8000

        assert_eq!(cpu.program_counter, 0x8001);
    }

    #[test]
    pub fn beq_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8001);
        cpu.load_and_run(vec!(0x00, 0xA9, 0x00, 0xF0, 0b1111_1011, 0x00));

        assert_eq!(cpu.program_counter, 0x8001)
    }

    #[test]
    pub fn bit_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.mem_write(0xABAB, 0b1101_1010);
        cpu.load_and_run(vec!(0xA9, 0x0F, 0x2C, 0xAB, 0xAB, 0x00));

        assert_eq!(cpu.status.0, 0b11000000);
    }

    #[test]
    pub fn bmi_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8001);
        cpu.load_and_run(vec!(0x00, 0xA9, 0xCC, 0x30, 0b1111_1011, 0x00));

        assert_eq!(cpu.program_counter, 0x8001);
    }

    #[test]
    pub fn bne_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8001);
        cpu.load_and_run(vec!(0x00, 0xA9, 0x01, 0xD0, 0b1111_1011, 0x00));

        assert_eq!(cpu.program_counter, 0x8001);
    }

    #[test]
    pub fn bpl_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8001);
        cpu.load_and_run(vec!(0x00, 0xA9, 0x00, 0x10, 0b1111_1011, 0x00));

        assert_eq!(cpu.program_counter, 0x8001);
    }

    #[test]
    pub fn bvc_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8001);
        cpu.mem_write(0xAB, 0b1011_0000);
        cpu.load_and_run(vec!(0x00, 0xA9, 0x00, 0x24, 0xAB, 0x50, (7 as i8).wrapping_neg() as u8, 0x00));

        assert_eq!(cpu.program_counter, 0x8001);
    }

    #[test]
    pub fn bvs_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8001);
        cpu.mem_write(0xAB, 0b1111_0000);
        cpu.load_and_run(vec!(0x00, 0xA9, 0x00, 0x24, 0xAB, 0x70, (7 as i8).wrapping_neg() as u8, 0x00));

        assert_eq!(cpu.program_counter, 0x8001);
    }
    
    #[test]
    pub fn clc_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA9, 0b1011_0001, 0x0A, 0x18, 0x00));

        assert_eq!(cpu.status.0, 0b0000_0000);
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
        cpu.status.0 = 0xAB;
        cpu.memory[0xFFFC] = 0xCD;
        cpu.memory[0xFFFD] = 0xAB;

        cpu.reset();
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.status.0, 0);
        assert_eq!(cpu.program_counter, 0xABCD);
    }
}