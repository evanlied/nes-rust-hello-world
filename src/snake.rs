use crate::cpu::CPU;
use crate::cpu::opcodes::OP_CODE_REF_TABLE;

pub fn snake_program() -> Vec<u8> {
    vec!(
        0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02, 0x85,
        0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9, 0x0f, 0x85,
        0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85, 0x00, 0xa5, 0xfe,
        0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20, 0x8d, 0x06, 0x20, 0xc3,
        0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c, 0x38, 0x06, 0xa5, 0xff, 0xc9,
        0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0, 0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60,
        0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85, 0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0,
        0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01, 0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02,
        0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05, 0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06,
        0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00, 0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07,
        0xe6, 0x03, 0xe6, 0x03, 0x20, 0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06,
        0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c,
        0x35, 0x07, 0x60, 0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02,
        0x4a, 0xb0, 0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9,
        0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
        0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10, 0xb0,
        0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5, 0x10, 0x29,
        0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe, 0x91, 0x00, 0x60,
        0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10, 0x60, 0xa2, 0x00, 0xea,
        0xea, 0xca, 0xd0, 0xfb, 0x60
    )
}

impl CPU {
    pub fn run_with_callback<F> (&mut self, mut callback: F) 
    where F: FnMut(&mut CPU) {
        loop {
            callback(self);
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
                "STX" => self.store_register_x(&op_code_params.addressing_mode),
                "STY" => self.store_register_y(&op_code_params.addressing_mode),
                "TAX" => self.transfer_a_to_x(),
                "TAY" => self.transfer_a_to_y(),
                "TSX" => self.transfer_stack_pointer_to_x(),
                "TXA" => self.transfer_x_to_a(),
                "TXS" => self.transfer_x_to_stack_pointer(),
                "TYA" => self.transfer_y_to_a(),
                "RTS" => self.return_subroutine(),
                "BRK" => return,
                _=> println!("TODO for ${op_code:#x}"), 
            }
            self.program_counter += op_code_params.bytes - 1;
    }
}
}
