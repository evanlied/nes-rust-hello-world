use super::CPU;
use crate::MemAccess;

const MIN_STACK: u16 = 0x0100;

// Methods that deal with manipulating the stack memory located at 0x0100-0x01FF
impl CPU {
    pub fn push_stack(&mut self, new_val: u8) {
        let stack_addr = MIN_STACK + self.stack_pointer as u16;
        self.mem_write(stack_addr, new_val);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn push_stack_u16(&mut self, new_val: u16) {
        let lo_bits = (new_val & 0x00FF) as u8;
        let hi_bits = (new_val >> 8) as u8;
        self.push_stack(hi_bits);
        self.push_stack(lo_bits);
    }

    pub fn pop_stack(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let stack_addr = MIN_STACK + self.stack_pointer as u16;
        let val = self.mem_read(stack_addr);
        return val;
    }

    pub fn pop_stack_u16(&mut self) -> u16 {
        let lo_bits = self.pop_stack();
        let hi_bits = self.pop_stack();
        u16::from_le_bytes([lo_bits, hi_bits])
    }

    pub fn push_processor_status(&mut self) {
        let status = self.status.0;
        self.push_stack(status);
        self.status.set_break_flag_1(false);
        self.status.set_break_flag_2(true);
    }

    pub fn pull_processor_status(&mut self) {
        self.status.0 = self.pop_stack();
        self.status.set_break_flag_1(false);
        self.status.set_break_flag_2(true);
    }

    pub fn pull_accumulator(&mut self) {
        self.register_a = self.pop_stack();
        self.status.set_negative_and_zero_flag(self.register_a);
    }
}

#[cfg(test)]
mod stack_controller_test {
    use super::*;

    const MIN_STACK: u16 = 0x0100;

    #[test]
    pub fn push_pop_upper_limit() {
        let mut cpu = CPU::new();
        cpu.stack_pointer = 0x0;
        cpu.push_stack(0xAB);
        assert_eq!(cpu.mem_read(0x100), 0xAB);
        assert_eq!(cpu.stack_pointer, 0xFF);

        assert_eq!(cpu.pop_stack(), 0xAB);
        assert_eq!(cpu.stack_pointer, 0x0);
    }

    #[test]
    pub fn push_pop_u16() {
        let mut cpu = CPU::new();
        cpu.stack_pointer = 0x0;
        cpu.push_stack_u16(0xABCD);
        assert_eq!(cpu.stack_pointer, 0xFE);
        assert_eq!(cpu.mem_read(0x1FF), 0xCD);
        assert_eq!(cpu.mem_read(0x100), 0xAB);

        assert_eq!(cpu.pop_stack_u16(), 0xABCD);
        assert_eq!(cpu.stack_pointer, 0x0);
    }

    // push processor status to be done in mod.rs
}