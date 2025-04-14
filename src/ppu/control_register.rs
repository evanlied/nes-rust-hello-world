pub struct ControlRegister(u8);

/**
 * BIT 0: Name Table 1 - Base nametable address (00=0x2000, 01=0x2400, 10=0x2800, 11=0x2C00)
 * BIT 1: Name Table 2 - Second bit of the nametable 
 * BIT 2: VRAM address increment per CPU read/write of PPUDATA (0: Adds 1, going across, 1: Adds 32, going down)
 * BIT 3: Sprite pattern table address for 8x8 sprites (0: $0000; 1: $1000; ignored in 8x16 mode)
 * BIT 4: Background pattern table addresses (0: 0x0000, 1: 0x1000)
 * BIT 5: Sprite size (0: 8x8 pixels, 1: 8x16 pixels)
 * BIT 6: PPU master/slave select (0: read backdrop from EXT pins; 1: output color on EXT pins)
 * BIT 7: Generate an NMI at the start of the vertical blanking interval (0: off; 1: on)
 */

// Constructor and Getters/Setters
impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister(0b0)
    }

    pub fn set_name_table_1(&mut self, val: bool) {
        match val {
            true => self.0 = self.0 | 0b0000_0001,
            false => self.0 = self.0 & 0b1111_1110,
        }
    }

    pub fn is_name_table_1(&self) -> bool {
        self.0 & 0b0000_0001 != 0
    } 

    pub fn set_name_table_2(&mut self, val: bool) {
        match val {
            true => self.0 = self.0 | 0b0000_0010,
            false => self.0 = self.0 & 0b1111_1101,
        }
    }

    pub fn is_name_table_2(&self) -> bool {
        self.0 & 0b0000_0010 != 0
    }

    pub fn set_vram_add_increment(&mut self, val: bool) {
        match val {
            true => self.0 = self.0 | 0b0000_0100,
            false => self.0 = self.0 & 0b1111_1011,
        }
    }

    pub fn is_vram_add_increment(&self) -> bool {
        self.0 & 0b0000_0100 != 0
    }

    pub fn set_sprite_pattern_addr(&mut self, val: bool) {
        match val {
            true => self.0 = self.0 | 0b0000_1000,
            false => self.0 = self.0 & 0b1111_0111,
        }
    }

    pub fn is_sprite_pattern_addr(&self) -> bool {
        self.0 & 0b0000_1000 != 0
    }

    pub fn set_background_pattern_addr(&mut self, val: bool) {
        match val {
            true => self.0 = self.0 | 0b0001_0000,
            false => self.0 = self.0 & 0b1110_1111,
        }
    }

    pub fn is_background_pattern_addr(&self) -> bool {
        self.0 & 0b0001_0000 != 0
    }

    pub fn set_sprite_size(&mut self, val: bool) {
        match val {
            true => self.0 = self.0 | 0b0010_0000,
            false => self.0 = self.0 & 0b1101_1111,
        }
    }

    pub fn is_sprite_size(&self) -> bool {
        self.0 & 0b0010_0000 != 0
    }

    pub fn set_master_slave_select(&mut self, val: bool) {
        match val {
            true => self.0 = self.0 | 0b0100_0000,
            false => self.0 = self.0 & 0b1011_1111,
        }
    }

    pub fn is_master_slave_select(&self) -> bool {
        self.0 & 0b0100_0000 != 0
    }

    pub fn set_generate_nmi(&mut self, val: bool) {
        match val {
            true => self.0 = self.0 | 0b1000_0000,
            false => self.0 = self.0 & 0b0111_1111,
        }
    }

    pub fn is_generate_nmi(&self) -> bool {
        self.0 & 0b1000_0000 != 0
    }

    pub fn update(&mut self, value: u8) {
        self.0 = value;
    }
}

// Not getter/setters
impl ControlRegister {
    pub fn get_vram_increment_size(&self) -> u8 {
        match self.is_vram_add_increment() {
            true => 1,
            false => 28
        }
    }
}

#[cfg(test)]
mod control_register_tests {
    use super::*;

    #[test]
    pub fn getter_setter_tests() {
        let mut foo = ControlRegister::new();
        assert_eq!(foo.0, 0b0);

        assert_eq!(foo.is_name_table_1(), false);
        assert_eq!(foo.is_name_table_2(), false);
        assert_eq!(foo.is_vram_add_increment(), false);
        assert_eq!(foo.is_sprite_pattern_addr(), false);
        assert_eq!(foo.is_background_pattern_addr(), false);
        assert_eq!(foo.is_sprite_size(), false);
        assert_eq!(foo.is_master_slave_select(), false);
        assert_eq!(foo.is_generate_nmi(), false);

        foo.set_name_table_1(true);
        foo.set_name_table_2(true);
        foo.set_vram_add_increment(true);
        foo.set_sprite_pattern_addr(true);
        foo.set_background_pattern_addr(true);
        foo.set_sprite_size(true);
        foo.set_master_slave_select(true);
        foo.set_generate_nmi(true);

        assert_eq!(foo.0, 0xFF);
        assert_eq!(foo.is_name_table_1(), true);
        assert_eq!(foo.is_name_table_2(), true);
        assert_eq!(foo.is_vram_add_increment(), true);
        assert_eq!(foo.is_sprite_pattern_addr(), true);
        assert_eq!(foo.is_background_pattern_addr(), true);
        assert_eq!(foo.is_sprite_size(), true);
        assert_eq!(foo.is_master_slave_select(), true);
        assert_eq!(foo.is_generate_nmi(), true);

        foo.set_name_table_1(false);
        foo.set_name_table_2(false);
        foo.set_vram_add_increment(false);
        foo.set_sprite_pattern_addr(false);
        foo.set_background_pattern_addr(false);
        foo.set_sprite_size(false);
        foo.set_master_slave_select(false);
        foo.set_generate_nmi(false);

        assert_eq!(foo.0, 0x0);
    }
}