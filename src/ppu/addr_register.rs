pub struct AddrRegister {
    value: (u8, u8),
    hi_ptr: bool,
}

impl AddrRegister {
    pub fn new() -> Self {
        AddrRegister {
            value: (0, 0), // (hi byte, lo byte) Big Endian
            hi_ptr: true,
        }
    }

    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xFF) as u8;
    }

    pub fn update(&mut self, data: u8) {
        match self.hi_ptr {
            true => self.value.0 = data,
            false => self.value.1 = data,
        };
        
        self.mirror_down();
        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        let prev_lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc);

        if prev_lo > self.value.1 { // Increment overflowed 255
            self.value.0 = self.value.0.wrapping_add(1); // Carry add to hi byte
        }
        self.mirror_down();
    }

    // If value is greater than 0x3FFF, blank the first two bits to keep the address range between 0x0 - 0x3FFF
    fn mirror_down(&mut self) {
        let val = self.get();
        if val > 0x3FFF { self.set(val & 0x3FFF) };
    }

    pub fn get(&self) -> u16 {
        u16::from_be_bytes([self.value.0, self.value.1])
    }
}

#[cfg(test)]
mod addr_register_tests {
    use super::*;

    #[test]
    pub fn update_mirrors_down() {
        let mut foo = AddrRegister::new();
        foo.update(0xFF);
        assert_eq!(foo.get(), 0x3F00);
    }

    #[test]
    pub fn increment_overflows_and_mirrors_down() {
        let mut foo = AddrRegister::new();
        foo.set(0x4FFD);
        foo.increment(3);
        assert_eq!(foo.get(), 0x1000)
    }
}