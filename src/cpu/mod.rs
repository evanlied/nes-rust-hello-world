mod load_instructions;
mod store_instructions;
mod branching_instructions;
mod arithmetic_instructions;
mod logical_instructions;
mod addressing_modes;
mod stack;
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
    stack_pointer: u16,
    memory: [u8; 0xFFFF],

    // The JMP Indirect instruction has a bug where fetches on addrress 0xXXFF would return the MSB from
    // 0xXX00 instead of (0xXXFF + 1) (ie XX + 1). For example AAFF would have MSB at AA00 instead of AB00.
    indirect_bug_enabled: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: StatusFlag(0),
            program_counter: 0,
            stack_pointer: 0x0100,
            memory: [0; 0xFFFF],
            indirect_bug_enabled: false,
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
        let hi = self.mem_read(addr.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let lo = (data & 0x00FF) as u8;
        let hi = (data >> 8) as u8;
        self.mem_write(addr, lo);
        self.mem_write(addr.wrapping_add(1), hi);
    }

    pub fn run(&mut self) {
        loop {
            let op_code = self.mem_read(self.program_counter);
            let op_code_params = OP_CODE_REF_TABLE.get(&op_code)
                .expect(&format!("${op_code:#x} is not a valid operation"));
            self.program_counter += 1;
            match op_code_params.instruction {
                "ADC" => self.add_with_carry(&op_code_params.addressing_mode),
                "AND" => self.and(&op_code_params.addressing_mode),
                "ASL" => self.arithmetic_shift_left(&op_code_params.addressing_mode),
                "BCC" => if self.branch_if_carry_clear() { continue; },
                "BCS" => if self.branch_if_carry_set() { continue; },
                "BEQ" => if self.branch_if_equal() { continue; },
                "BIT" => self.bit_test(&op_code_params.addressing_mode),
                "BMI" => if self.branch_if_minus() { continue; },
                "BNE" => if self.branch_if_not_equal() { continue; },
                "BPL" => if self.branch_if_positive() { continue; },
                "BVC" => if self.branch_if_overflow_clear() { continue; },
                "BVS" => if self.branch_if_overflow_set() { continue; },
                "CLC" => self.status.set_carry_flag(false),
                "CLD" => self.status.set_decimal_flag(false),
                "CLI" => self.status.set_interrupt_flag(false),
                "CLV" => self.status.set_overflow_flag(0),
                "CMP" => self.compare(&op_code_params.addressing_mode),
                "CPX" => self.compare_x(&op_code_params.addressing_mode),
                "CPY" => self.compare_y(&op_code_params.addressing_mode),
                "DEC" => self.decrement_mem(&op_code_params.addressing_mode),
                "DEX" => self.decrement_x(),
                "DEY" => self.decrement_y(),
                "EOR" => self.exclusive_or(&op_code_params.addressing_mode),
                "INC" => self.increment_mem(&op_code_params.addressing_mode),
                "INX" => self.increment_x(),
                "INY" => self.increment_y(),
                "JMP" => {
                    self.jump(&op_code_params.addressing_mode);
                    continue;
                },
                "JSR" => {
                    self.jump_subroutine();
                    continue;
                },
                "LDA" => self.load_register_a(&op_code_params.addressing_mode),
                "LDX" => self.load_register_x(&op_code_params.addressing_mode),
                "LDY" => self.load_register_y(&op_code_params.addressing_mode),
                "LSR" => self.logical_shift_right(&op_code_params.addressing_mode),
                "NOP" => (),
                "ORA" => self.inclusive_or(&op_code_params.addressing_mode),
                "PHA" => self.push_stack(self.register_a),
                "PHP" => self.push_processor_status(),
                "PLA" => self.pull_accumulator(),
                "PLP" => self.pull_processor_status(),
                "ROL" => self.rotate_left(&op_code_params.addressing_mode),
                "ROR" => self.rotate_right(&op_code_params.addressing_mode),
                "RTI" => {
                    self.return_from_interrupt();
                    continue;
                },
                "SBC" => self.subtract_with_carry(&op_code_params.addressing_mode),
                "SEC" => self.set_carry_flag(),
                "SED" => self.set_decimal_flag(),
                "SEI" => self.set_interrupt_flag(),
                "STA" => self.store_register_a(&op_code_params.addressing_mode),
                "TAX" => self.transfer_a_to_x(),
                "RTS" => self.return_subroutine(),
                "BRK" => return,
                _=> println!("TODO for ${op_code:#x}"), 
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
        self.stack_pointer = 0x100;
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
    pub fn adc_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.mem_write(0x70, 33);
        cpu.load_and_run(vec!(0xA9, 120, 0x65, 0x70, 0x0));

        assert_eq!(cpu.program_counter, 0x8005);
        assert_eq!(cpu.register_a, 153);
        assert_eq!(cpu.status.0, 0b1000_0000);
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
    pub fn cld_instruction() {
        let mut cpu = CPU::new();
        cpu.status.0 = 0xFF;
        cpu.load(vec!(0xD8, 0x00));
        cpu.run();

        assert_eq!(cpu.status.0, 0b1111_0111);
        assert_eq!(cpu.program_counter, 0x8002);
    }

    #[test]
    pub fn cli_instruction() {
        let mut cpu = CPU::new();
        cpu.status.0 = 0xFF;
        cpu.load(vec!(0x58, 0x00));
        cpu.run();
        
        assert_eq!(cpu.status.0, 0b1111_1011);
        assert_eq!(cpu.program_counter, 0x8002);
    }

    #[test]
    pub fn clv_instruction() {
        let mut cpu = CPU::new();
        cpu.status.0 = 0xFF;
        cpu.load(vec!(0xB8, 0x00));
        cpu.run();
        
        assert_eq!(cpu.status.0, 0b1011_1111);
        assert_eq!(cpu.program_counter, 0x8002);
    }

    #[test]
    pub fn cmp_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.mem_write_u16(0x7000, 0x15);
        cpu.load_and_run(vec!(0xA9, 0xA0, 0xCD, 0x00, 0x70, 0x00));

        assert_eq!(cpu.program_counter, 0x8006);
        assert_eq!(cpu.status.0, 0b1000_0001);
    }

    #[test]
    pub fn cpx_instruction() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x15;
        cpu.mem_write_u16(0x7000, 0x15);
        cpu.load(vec!(0xEC, 0x0, 0x70, 0x00));
        cpu.run();

        assert_eq!(cpu.program_counter, 0x8004);
        assert_eq!(cpu.status.0, 0b0000_0011);
    }

    #[test]
    pub fn cpy_instruction() {
        let mut cpu = CPU::new();
        cpu.register_y = 0xAB;
        cpu.mem_write_u16(0x7000, 0xA0);
        cpu.load(vec!(0xCC, 0x0, 0x70, 0x00));
        cpu.run();

        assert_eq!(cpu.program_counter, 0x8004);
        assert_eq!(cpu.status.0, 0b0000_0001);
    }

    #[test]
    pub fn dec_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.mem_write(0x7000, 155);
        cpu.load_and_run(vec!(0xCE, 0x0, 0x70, 0x00));

        assert_eq!(cpu.mem_read(0x7000), 154);
        assert_eq!(cpu.status.0, 0b1000_0000);
        assert_eq!(cpu.program_counter, 0x8004);
    }

    #[test]
    pub fn dex_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xCA, 0x00));
        assert_eq!(cpu.register_x, 0xFF);
        assert_eq!(cpu.status.0, 0b1000_0000);
        assert_eq!(cpu.program_counter, 0x8002);
    }

    #[test]
    pub fn dey_instruction() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x1;
        cpu.load(vec!(0x88, 0x00));
        cpu.run();
        assert_eq!(cpu.register_y, 0);
        assert_eq!(cpu.status.0, 0b0000_0010);
        assert_eq!(cpu.program_counter, 0x8002);
    }

    #[test]
    pub fn eor_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA9, 0xFF, 0x49, 0b1010_1010, 0x00));
        assert_eq!(cpu.program_counter, 0x8005);
        assert_eq!(cpu.register_a, 0b0101_0101);
        assert_eq!(cpu.status.0, 0);
    }

    #[test]
    pub fn inc_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.mem_write(0x7000, 0xD2);
        cpu.load_and_run(vec!(0xEE, 0x0, 0x70, 0x00));
        assert_eq!(cpu.program_counter, 0x8004);
        assert_eq!(cpu.mem_read(0x7000), 0xD3);
        assert_eq!(cpu.status.0, 0b1000_0000);
    }

    #[test]
    pub fn iny_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA0, 210, 0xC8, 0x00));
        assert_eq!(cpu.program_counter, 0x8004);
        assert_eq!(cpu.register_y, 211);
        assert_eq!(cpu.status.0, 0b1000_0000);
    }

    #[test]
    pub fn ldx_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA2, 0xFF, 0x00));
        assert_eq!(cpu.program_counter, 0x8003);
        assert_eq!(cpu.register_x, 0xFF);
        assert_eq!(cpu.status.0, 0b1000_0000);
    }

    #[test]
    pub fn ldy_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA0, 0x32, 0x00));
        assert_eq!(cpu.program_counter, 0x8003);
        assert_eq!(cpu.register_y, 0x32);
        assert_eq!(cpu.status.0, 0);
    }

    #[test]
    pub fn jmp_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.mem_write_u16(0x7000, 0xABCD);
        cpu.load_and_run(vec!(0x4C, 0x05, 0x80, 0x00, 0x00, 0x6C, 0x00, 0x70, 0x00));
        assert_eq!(cpu.program_counter, 0xABCE);
    }

    #[test]
    pub fn jsr_rts_instructions() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0x20, 0x06, 0x80, 0xA2, 0x69, 0x00, 0xA0, 0xDC, 0x60, 0x0));
        assert_eq!(cpu.program_counter, 0x8006);
        assert_eq!(cpu.register_x, 0x69);
        assert_eq!(cpu.register_y, 0xDC);
    }

    #[test]
    pub fn lsr_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA9, 0b0000_0001, 0x4A, 0x00));
        assert_eq!(cpu.program_counter, 0x8004);
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.status.0, 0b0000_0011);
    }

    #[test]
    pub fn nop_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xEA, 0xEA, 0x00));
        assert_eq!(cpu.program_counter, 0x8003);
    }

    #[test]
    pub fn ora_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA9, 0b1000_0001, 0x09, 0b0001_1000, 0x00));
        assert_eq!(cpu.program_counter, 0x8005);
        assert_eq!(cpu.register_a, 0b1001_1001);
        assert_eq!(cpu.status.0, 0b1000_0000);
    }

    #[test]
    pub fn pha_pla_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA9, 0xF0, 0x48, 0x69, 0x5, 0x68, 0x00));
        assert_eq!(cpu.program_counter, 0x8007);
        assert_eq!(cpu.mem_read(0x100), 0xF0);
        assert_eq!(cpu.register_a, 0xF0);
        assert_eq!(cpu.stack_pointer, 0x100);
    }

    #[test]
    pub fn php_plp_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA9, 0xFF, 0x08, 0x69, 0x10, 0x28, 0x0));
        assert_eq!(cpu.program_counter, 0x8007);
        assert_eq!(cpu.status.0, 0b1010_0000);
        assert_eq!(cpu.mem_read(0x100), 0b1000_0000);
    }

    #[test]
    pub fn rol_ror_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA9, 0b11000011, 0x2A, 0x2A, 0x6A, 00));
        assert_eq!(cpu.program_counter, 0x8006);
        assert_eq!(cpu.register_a, 0b1000_0111);
        assert_eq!(cpu.status.0, 0b1000_0001);
    }

    #[test]
    pub fn rti_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.push_stack_u16( 0x8050);
        cpu.push_stack( 0b1000_0010);
        cpu.load(vec!(0x40));
        cpu.run();
        assert_eq!(cpu.program_counter, 0x8051);
        assert_eq!(cpu.status.0, 0b1010_0010);
        assert_eq!(cpu.stack_pointer, 0x100);
    }

    #[test]
    pub fn sbc_instruction() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0xA9, 0, 0xE9, 10, 0x0));
        assert_eq!(cpu.program_counter, 0x8005);
        assert_eq!(cpu.status.0, 0b1000_0000);
        assert_eq!(cpu.register_a, 246);
    }

    #[test]
    pub fn sec_sed_sei_instructions() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xFFFC, 0x8000);
        cpu.load_and_run(vec!(0x38, 0xF8, 0x78, 0x0));
        assert_eq!(cpu.program_counter, 0x8004);
        assert_eq!(cpu.status.0, 0b0000_1101);
    }

    // ------------------------------------------------------------
    // END OF INSTRUCTION TEST SECTION
    // ------------------------------------------------------------

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