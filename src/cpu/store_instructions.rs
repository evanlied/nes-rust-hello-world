use super::{addressing_modes::AddressingMode, CPU};

impl CPU {
    // sta
    pub fn store_register_a(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        self.mem_write(addr, self.register_a);
    }
}

#[cfg(test)]
mod store_test {
    use super::*;

    #[test]
    fn sta_zero_page() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x69;
        cpu.mem_write(0, 0xAB);
        
        cpu.store_register_a(&AddressingMode::ZeroPage);
        assert_eq!(cpu.memory[0x00AB], 0x69);
    }
}