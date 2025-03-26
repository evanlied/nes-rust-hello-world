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

    pub fn set_negative_flag(&mut self, value: u8) {
         // If the value is negative, set the negative flag to 1 otherwise set it to 0
         if value & 0b1000_0000 != 0 {
            self.0 = self.0 | 0b1000_0000;
        } else {
            self.0 = self.0 & 0b0111_1111;
        }
    }

    pub fn set_carry_flag(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.0 = self.0 | 0b0000_0001;
        } else {
            self.0 = self.0 & 0b1111_1110;
        }
    }

    pub fn set_negative_and_zero_flag(&mut self, value: u8) {
        self.set_zero_flag(value);
        self.set_negative_flag(value);
    }

    pub fn is_carry_set(&self) -> bool {
        self.0 & 0b0000_0001 != 0
    }

    pub fn is_zero_set(&self) -> bool {
        self.0 & 0b0000_0010 != 0
    }
}

#[cfg(test)]
mod status_flag_tests {
    use super::*;

    #[test]
    pub fn is_carry_set_test() {
        let status = StatusFlag(0b0000_0001);
        assert!(status.is_carry_set());

        let status = StatusFlag(0b0000_0000);
        assert!(!status.is_carry_set());
    }
}
