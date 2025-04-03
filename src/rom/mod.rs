use crate::Mirroring;

// TODO update these with the actual values later
const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PGR_ROM_PAGE_SIZE: usize = 1024;
const CHR_ROM_PAGE_SIZE: usize = 1024;

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring
}

impl Rom {
    pub fn new(raw: &Vec<u8>) -> Result<Rom, String> {
        if &raw[0..4] != NES_TAG {
            return Err(String::from("File is not in the iNES file format"));
        }

        // Take the MSB of raw[7] and combine it with the LSB of raw[6]
        let mapper = (raw[7] & 0b1111_0000) | (raw[6] >> 4);

        // Read bits (3,2). If 10, then iNES 2.0 format, if 00 then iNES 1.0 format
        let ines_ver = (raw[7] >> 2) & 0b11;
        if ines_ver != 0 {
            return Err(String::from("iNES2.0 format is not supported"));
        }

        let four_screen = raw[6] & 0b0000_1000 != 0;
        let vertical_mirroring = raw[6] & 0b0000_0001 != 0;
        let screen_mirroring = match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let prg_rom_size = raw[4] as usize * PGR_ROM_PAGE_SIZE;
        let chr_rom_size = raw[5] as usize * CHR_ROM_PAGE_SIZE;

        let skip_trainer = raw[6] & 0b0000_0100 != 0;

        let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        Ok(Rom {
            prg_rom: raw[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr_rom: raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            mapper,
            screen_mirroring
        })
    }
}

#[cfg(test)]
mod rom_constructor_test {
    use super::*;

    fn get_test_raw() -> Vec<u8> {
        let mut vec = vec!(
            0x4E, 0x45, 0x53, 0x1A,     // NES header tag
            1, 1,                       // PGR and CHR Rom size
            0x0, 0x0,                   // Control bytes 1 and 2
            0, 0, 0, 0, 0, 0, 0, 0,     // padding bytes to get to 16 bit header 
        );
        // populate the prg and chr rom with 0s
        for _ in 0..(PGR_ROM_PAGE_SIZE + CHR_ROM_PAGE_SIZE) {
            vec.push(0);
        }

        vec
    }

    #[test]
    pub fn header_incorrect() {
        let tester: Vec<u8> = vec!(1, 1, 1, 1);
        let result = Rom::new(&tester);
        match result {
            Ok(_) => panic!("Should have been an error"),
            Err(str) => assert_eq!(str, "File is not in the iNES file format")
        }
    }

    #[test]
    pub fn ines_ver_incorrect() {
        let mut tester = get_test_raw();
        tester[7] = 0b0000_1000;
        match Rom::new(&tester) {
            Ok(_) => panic!("Should have been an error"),
            Err(str) => assert_eq!(str, "iNES2.0 format is not supported"),
        } 
    }

    #[test]
    pub fn positive_test_case() {
        let tester = get_test_raw();
        match Rom::new(&tester) {
            Ok(_) => (),
            Err(_) => panic!("Should have built"),
        }
    }
}