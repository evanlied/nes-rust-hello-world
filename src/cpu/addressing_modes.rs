use super::CPU;
use super::MemAccess;

#[derive(Clone, Debug, PartialEq)]
pub enum AddressingMode {
    Immediate,
    Accumulator,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Implied,
    Relative,
}

impl CPU {
    pub fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPageX => self.mem_read(self.program_counter).wrapping_add(self.register_x) as u16,
            AddressingMode::ZeroPageY => self.mem_read(self.program_counter).wrapping_add(self.register_y) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::AbsoluteX => self.mem_read_u16(self.program_counter).wrapping_add(self.register_x as u16),
            AddressingMode::AbsoluteY => self.mem_read_u16(self.program_counter).wrapping_add(self.register_y as u16),
            AddressingMode::Indirect => {
                let ptr_addr = self.mem_read(self.program_counter);
                let ptr = self.mem_read_u16(ptr_addr as u16);
                match self.indirect_bug_enabled && (ptr & 0xFF == 0xFF ) {
                    true => {
                        let bugged_ptr = ptr & 0xFF00;
                        bugged_ptr
                    },
                    false => ptr
                }
            },
            AddressingMode::IndirectX => {
                let base = self.mem_read(self.program_counter) as u8;
                let ptr = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                u16::from_le_bytes([lo, hi])
            },
            AddressingMode::IndirectY => {
                let base = self.mem_read(self.program_counter) as u8;
                let ptr = base.wrapping_add(self.register_y);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                u16::from_le_bytes([lo, hi])
            },
            _ => panic!("Can't get addr for addressing mode {:?}", mode),
        }
    }

    /** This function is mostly used by shift / rotate operations that can work directly on the accumulator */
    pub fn get_val_and_mem_ptr(&mut self, mode: &AddressingMode) -> (u8, &mut u8) {
        match mode {
            AddressingMode::Accumulator => (self.register_a, &mut self.register_a),
            _ => {
                let addr = self.get_operand_address(mode);
                (self.mem_read(addr), &mut self.bus.cpu_vram[addr as usize])
            }
        }
    }
}

#[cfg(test)]
mod test_addressing_modes {
    use super::*;

    #[test]
    pub fn immediate_addr() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0xABCD;
        assert_eq!(cpu.get_operand_address(&AddressingMode::Immediate), 0xABCD);
    }

    #[test]
    pub fn zero_page_addr() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x800;
        cpu.mem_write(0x800, 0x05);
        cpu.mem_write(0x801, 0x0A);
        cpu.register_x = 0x06;
        cpu.register_y = 0x05;
        assert_eq!(cpu.get_operand_address(&AddressingMode::ZeroPage), 0x05);
    }

    #[test]
    pub fn zero_page_x_addr() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x800;
        cpu.mem_write(0x800, 0x05);
        cpu.mem_write(0x801, 0x0A);
        cpu.register_x = 0x06;
        cpu.register_y = 0x05;
        assert_eq!(cpu.get_operand_address(&AddressingMode::ZeroPageX), 0x0B);
    }

    #[test]
    pub fn zero_page_y_addr() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x800;
        cpu.mem_write(0x800, 0x05);
        cpu.mem_write(0x801, 0x0A);
        cpu.register_x = 0x06;
        cpu.register_y = 0x05;
        assert_eq!(cpu.get_operand_address(&AddressingMode::ZeroPageY), 0x0A);
    }

    #[test]
    pub fn absolute_addr() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x800;
        cpu.mem_write(0x800, 0x05);
        cpu.mem_write(0x801, 0x0A);
        cpu.register_x = 0x06;
        cpu.register_y = 0x05;
        assert_eq!(cpu.get_operand_address(&AddressingMode::Absolute), 0x0A05);
    }

    #[test]
    pub fn absolute_x_addr() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x800;
        cpu.mem_write(0x800, 0x05);
        cpu.mem_write(0x801, 0x0A);
        cpu.register_x = 0x06;
        cpu.register_y = 0x05;
        assert_eq!(cpu.get_operand_address(&AddressingMode::AbsoluteX), 0x0A0B);
    }

    #[test]
    pub fn absolute_y_addr() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x800;
        cpu.mem_write(0x800, 0x05);
        cpu.mem_write(0x801, 0x0A);
        cpu.register_x = 0x06;
        cpu.register_y = 0x05;
        assert_eq!(cpu.get_operand_address(&AddressingMode::AbsoluteY), 0x0A0A);
    }

    #[test]
    pub fn indirect_x_addr() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x800;
        cpu.mem_write(0x800, 0x05);
        cpu.mem_write(0x801, 0x0A);
        cpu.mem_write(0xB, 0x12);
        cpu.mem_write(0xC, 0xC);
        cpu.register_x = 0x06;
        cpu.register_y = 0x05;
        assert_eq!(cpu.get_operand_address(&AddressingMode::IndirectX), 0x0C12);
    }

    #[test]
    pub fn indirect_y_addr() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x800;
        cpu.mem_write(0x800, 0x05);
        cpu.mem_write(0x801, 0x0A);
        cpu.mem_write(0xA, 0x12);
        cpu.mem_write(0xB, 0xC);
        cpu.register_x = 0x06;
        cpu.register_y = 0x05;
        assert_eq!(cpu.get_operand_address(&AddressingMode::IndirectY), 0x0C12);
    }

    #[test]
    pub fn indirect_bug_disabled() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x800;
        cpu.mem_write_u16(0x800, 0x1FF);
        cpu.mem_write_u16(0x1FF, 0xABAB);
        assert_eq!(cpu.get_operand_address(&AddressingMode::Indirect), 0xABAB);
    }

    #[test]
    pub fn indirect_bug_enabled() {
        let mut cpu = CPU::new();
        cpu.indirect_bug_enabled = true;
        cpu.program_counter = 0x800;
        cpu.mem_write_u16(0x800, 0x1FF);
        cpu.mem_write(0x1FF, 0xCD);
        cpu.mem_write(0x100, 0xAB);
        assert_eq!(cpu.get_operand_address(&AddressingMode::Indirect), 0xABCD);
    }
}