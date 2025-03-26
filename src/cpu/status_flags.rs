use std::ops::Deref;

pub struct StatusFlag(pub u8);

impl Deref for StatusFlag {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StatusFlag {
    pub fn set_zero_flag(&mut self, value: u8) {
        // If the value is zero set the zero flag to 1 otherwise set it to 0
        if value == 0 {
            self.0 = self.0 | 0b0000_0010;
        } else {
            self.0 = self.0 & 0b1111_1101;
        }
    }

    pub fn set_carry_flag(&mut self, value: u8) {
         // If the value is negative, set the negative flag to 1 otherwise set it to 0
         if value & 0b1000_0000 != 0 {
            self.0 = self.0 | 0b1000_0000;
        } else {
            self.0 = self.0 & 0b0111_1111;
        }
    }

    pub fn set_carry_and_zero_flag(&mut self, value: u8) {
        self.set_zero_flag(value);
        self.set_carry_flag(value);
    }
}
