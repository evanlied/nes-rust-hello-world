use super::CPU;

const MAX_STACK: u16 = 0x01FF;
const MIN_STACK: u16 = 0x0100;

// Methods that deal with manipulating the stack memory located at 0x0100-0x01FF
impl CPU {
    pub fn push_stack(&mut self, new_val: u8) {
        if self.stack_pointer > MAX_STACK {
            panic!("Stack out of memory");
        }
        self.mem_write(self.stack_pointer, new_val);
        self.stack_pointer += 1;
    }

    pub fn push_stack_u16(&mut self, new_val: u16) {
        let lo_bits = (new_val & 0x00FF) as u8;
        let hi_bits = (new_val >> 8) as u8;
        self.push_stack(lo_bits);
        self.push_stack(hi_bits);
    }

    pub fn pop_stack(&mut self) -> u8 {
        if self.stack_pointer <= MIN_STACK {
            panic!("Stack is empty");
        }
        self.stack_pointer -= 1;
        let val = self.mem_read(self.stack_pointer);
        return val;
    }

    pub fn pop_stack_u16(&mut self) -> u16 {
        let hi_bits = self.pop_stack();
        let lo_bits = self.pop_stack();
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
        self.status.set_zero_flag(self.register_a);
    }
}

#[cfg(test)]
mod stack_controller_test {
    use super::*;

    #[test]
    #[should_panic]
    pub fn push_pop_upper_limit() {
        let mut cpu = CPU::new();
        cpu.stack_pointer = MAX_STACK;
        cpu.push_stack(0xAB);
        assert_eq!(cpu.mem_read(MAX_STACK), 0xAB);
        assert_eq!(cpu.stack_pointer, MAX_STACK + 1);

        assert_eq!(cpu.pop_stack(), 0xAB);
        assert_eq!(cpu.stack_pointer, MAX_STACK);

        cpu.push_stack(0xAB);
        cpu.push_stack(0xCD);
    }

    #[test]
    #[should_panic]
    pub fn push_pop_lower_limit() {
        let mut cpu = CPU::new();
        cpu.push_stack(0xAB);
        assert_eq!(cpu.mem_read(MIN_STACK), 0xAB);
        assert_eq!(cpu.stack_pointer, MIN_STACK + 1);

        assert_eq!(cpu.pop_stack(), 0xAB);
        assert_eq!(cpu.stack_pointer, MIN_STACK);

        cpu.pop_stack();
    }

    #[test]
    pub fn push_pop_u16() {
        let mut cpu = CPU::new();
        cpu.push_stack_u16(0xABCD);
        assert_eq!(cpu.stack_pointer, MIN_STACK + 2);
        assert_eq!(cpu.mem_read_u16(MIN_STACK), 0xABCD);

        assert_eq!(cpu.pop_stack_u16(), 0xABCD);
        assert_eq!(cpu.stack_pointer, MIN_STACK);
    }

    // push processor status to be done in mod.rs
}