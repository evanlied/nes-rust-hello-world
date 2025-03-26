use super::CPU;

impl CPU {
    pub fn increment_x(&mut self) {
        self.register_x += 1;
        self.status.set_carry_and_zero_flag(self.register_x);
    }
}

#[cfg(test)]
mod arithmetic_test {
    use super::*;

    #[test]
    pub fn increment_x_test() {
        let mut cpu = CPU::new();
        cpu.register_x = 0b01111111;
        cpu.increment_x();
        assert_eq!(cpu.register_x, 0b10000000);
        assert_eq!(cpu.status.0, 0b10000000);
    }
}