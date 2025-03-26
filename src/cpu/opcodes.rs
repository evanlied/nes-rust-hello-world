use phf::phf_map;

use super::addressing_modes::AddressingMode;

#[derive(Clone)]
pub struct OpCode {
    pub instruction: &'static str,
    pub cycles: u8,
    pub bytes: u16,
    pub addressing_mode: AddressingMode,
}

impl OpCode {
    pub const fn new(instruction: &'static str, bytes: u16, cycles: u8, addressing_mode: AddressingMode) -> Self {
        OpCode{ instruction, cycles, bytes, addressing_mode}
    }
}

pub static OP_CODE_REF_TABLE: phf::Map<u8, OpCode> = phf_map! {
    // AND
    0x29u8 => OpCode::new("AND", 2, 2, AddressingMode::Immediate),
    0x25u8 => OpCode::new("AND", 2, 3, AddressingMode::ZeroPage),
    0x35u8 => OpCode::new("AND", 2, 4, AddressingMode::ZeroPageX),
    0x2Du8 => OpCode::new("AND", 3, 4, AddressingMode::Absolute),
    0x3Du8 => OpCode::new("AND", 3, 4, AddressingMode::AbsoluteX),
    0x39u8 => OpCode::new("AND", 3, 4, AddressingMode::AbsoluteY),
    0x21u8 => OpCode::new("AND", 2, 6, AddressingMode::IndirectX),
    0x31u8 => OpCode::new("AND", 2, 5, AddressingMode::IndirectY),

    // LDA
    0xA9u8 => OpCode::new("LDA", 2, 2, AddressingMode::Immediate),
    0xA5u8 => OpCode::new("LDA", 2, 3, AddressingMode::ZeroPage),
    0xB5u8 => OpCode::new("LDA", 2, 4, AddressingMode::ZeroPageX),
    0xADu8 => OpCode::new("LDA", 3, 4, AddressingMode::Absolute),
    0xBDu8 => OpCode::new("LDA", 3, 4, AddressingMode::AbsoluteX),
    0xB9u8 => OpCode::new("LDA", 3, 4, AddressingMode::AbsoluteY),
    0xA1u8 => OpCode::new("LDA", 2, 6, AddressingMode::IndirectX),
    0xB1u8 => OpCode::new("LDA", 2, 5, AddressingMode::IndirectY),

    // STA
    0x85u8 => OpCode::new("STA", 2, 3, AddressingMode::ZeroPage),
    0x95u8 => OpCode::new("STA", 2, 4, AddressingMode::ZeroPageX),
    0x8Du8 => OpCode::new("STA", 3, 4, AddressingMode::Absolute),
    0x9Du8 => OpCode::new("STA", 3, 5, AddressingMode::AbsoluteX),
    0x99u8 => OpCode::new("STA", 3, 5, AddressingMode::AbsoluteY),
    0x81u8 => OpCode::new("STA", 2, 6, AddressingMode::IndirectX),
    0x91u8 => OpCode::new("STA", 2, 6, AddressingMode::IndirectY),

    // TAX
    0xAAu8 => OpCode::new("TAX", 1, 2, AddressingMode::Implied),

    // INX
    0xE8u8 => OpCode::new("INX", 1, 2, AddressingMode::Implied),

    // BRK
    0x00u8 => OpCode::new("BRK", 1, 7, AddressingMode::Implied),
};