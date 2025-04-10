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
    // ADC
    0x69u8 => OpCode::new("ADC", 2, 2, AddressingMode::Immediate),
    0x65u8 => OpCode::new("ADC", 2, 3, AddressingMode::ZeroPage),
    0x75u8 => OpCode::new("ADC", 2, 4, AddressingMode::ZeroPageX),
    0x6Du8 => OpCode::new("ADC", 3, 4, AddressingMode::Absolute),
    0x7Du8 => OpCode::new("ADC", 3, 4, AddressingMode::AbsoluteX),
    0x79u8 => OpCode::new("ADC", 3, 4, AddressingMode::AbsoluteY),
    0x61u8 => OpCode::new("ADC", 2, 6, AddressingMode::IndirectX),
    0x71u8 => OpCode::new("ADC", 2, 5, AddressingMode::IndirectY),

    // AND
    0x29u8 => OpCode::new("AND", 2, 2, AddressingMode::Immediate),
    0x25u8 => OpCode::new("AND", 2, 3, AddressingMode::ZeroPage),
    0x35u8 => OpCode::new("AND", 2, 4, AddressingMode::ZeroPageX),
    0x2Du8 => OpCode::new("AND", 3, 4, AddressingMode::Absolute),
    0x3Du8 => OpCode::new("AND", 3, 4, AddressingMode::AbsoluteX),
    0x39u8 => OpCode::new("AND", 3, 4, AddressingMode::AbsoluteY),
    0x21u8 => OpCode::new("AND", 2, 6, AddressingMode::IndirectX),
    0x31u8 => OpCode::new("AND", 2, 5, AddressingMode::IndirectY),

    // ASL
    0x0Au8 => OpCode::new("ASL", 1, 2, AddressingMode::Accumulator),
    0x06u8 => OpCode::new("ASL", 2, 5, AddressingMode::ZeroPage),
    0x16u8 => OpCode::new("ASL", 2, 6, AddressingMode::ZeroPageX),
    0x0Eu8 => OpCode::new("ASL", 3, 6, AddressingMode::Absolute),
    0x1Eu8 => OpCode::new("ASL", 3, 7, AddressingMode::AbsoluteX),

    // Branching
    0x90u8 => OpCode::new("BCC", 2, 2, AddressingMode::Relative),
    0xB0u8 => OpCode::new("BCS", 2, 2, AddressingMode::Relative),
    0xF0u8 => OpCode::new("BEQ", 2, 2, AddressingMode::Relative),
    0x30u8 => OpCode::new("BMI", 2, 2, AddressingMode::Relative),
    0xD0u8 => OpCode::new("BNE", 2, 2, AddressingMode::Relative),
    0x10u8 => OpCode::new("BPL", 2, 2, AddressingMode::Relative),
    0x50u8 => OpCode::new("BVC", 2, 2, AddressingMode::Relative),
    0x70u8 => OpCode::new("BVS", 2, 2, AddressingMode::Relative),

    // BIT
    0x24u8 => OpCode::new("BIT", 2, 3, AddressingMode::ZeroPage),
    0x2Cu8 => OpCode::new("BIT", 3, 4, AddressingMode::Absolute),

    // Flag Controls
    0x18u8 => OpCode::new("CLC", 1, 2, AddressingMode::Implied),
    0xD8u8 => OpCode::new("CLD", 1, 2, AddressingMode::Implied),
    0x58u8 => OpCode::new("CLI", 1, 2, AddressingMode::Implied),
    0xB8u8 => OpCode::new("CLV", 1, 2, AddressingMode::Implied),

    // CMP
    0xC9u8 => OpCode::new("CMP", 2, 2, AddressingMode::Immediate),
    0xC5u8 => OpCode::new("CMP", 2, 3, AddressingMode::ZeroPage),
    0xD5u8 => OpCode::new("CMP", 2, 4, AddressingMode::ZeroPageX),
    0xCDu8 => OpCode::new("CMP", 3, 4, AddressingMode::Absolute),
    0xDDu8 => OpCode::new("CMP", 3, 4, AddressingMode::AbsoluteX),
    0xD9u8 => OpCode::new("CMP", 3, 4, AddressingMode::AbsoluteY),
    0xC1u8 => OpCode::new("CMP", 2, 6, AddressingMode::IndirectX),
    0xD1u8 => OpCode::new("CMP", 2, 5, AddressingMode::IndirectY),

    // CPX
    0xE0u8 => OpCode::new("CPX", 2, 2, AddressingMode::Immediate),
    0xE4u8 => OpCode::new("CPX", 2, 3, AddressingMode::ZeroPage),
    0xEcu8 => OpCode::new("CPX", 3, 4, AddressingMode::Absolute),

    // CPY
    0xC0u8 => OpCode::new("CPY", 2, 2, AddressingMode::Immediate),
    0xC4u8 => OpCode::new("CPY", 2, 3, AddressingMode::ZeroPage),
    0xCCu8 => OpCode::new("CPY", 3, 4, AddressingMode::Absolute),

    // DEC
    0xC6u8 => OpCode::new("DEC", 2, 5, AddressingMode::ZeroPage),
    0xD6u8 => OpCode::new("DEC", 2, 6, AddressingMode::ZeroPageX),
    0xCEu8 => OpCode::new("DEC", 3, 6, AddressingMode::Absolute),
    0xDEu8 => OpCode::new("DEC", 3, 7, AddressingMode::AbsoluteX),

    // DEX
    0xCAu8 => OpCode::new("DEX", 1, 2, AddressingMode::Implied),

    // DEY
    0x88u8 => OpCode::new("DEY", 1, 2, AddressingMode::Implied),

    // EOR
    0x49u8 => OpCode::new("EOR", 2, 2, AddressingMode::Immediate),
    0x45u8 => OpCode::new("EOR", 2, 3, AddressingMode::ZeroPage),
    0x55u8 => OpCode::new("EOR", 2, 4, AddressingMode::ZeroPageX),
    0x4Du8 => OpCode::new("EOR", 3, 4, AddressingMode::Absolute),
    0x5Du8 => OpCode::new("EOR", 3, 4, AddressingMode::AbsoluteX),
    0x59u8 => OpCode::new("EOR", 3, 4, AddressingMode::AbsoluteY),
    0x41u8 => OpCode::new("EOR", 2, 6, AddressingMode::IndirectX),
    0x51u8 => OpCode::new("EOR", 2, 5, AddressingMode::IndirectY),

    // INC
    0xE6u8 => OpCode::new("INC", 2, 5, AddressingMode::ZeroPage),
    0xF6u8 => OpCode::new("INC", 2, 6, AddressingMode::ZeroPageX),
    0xEEu8 => OpCode::new("INC", 3, 6, AddressingMode::Absolute),
    0xFEu8 => OpCode::new("INC", 3, 7, AddressingMode::AbsoluteX),

    // INX
    0xE8u8 => OpCode::new("INX", 1, 2, AddressingMode::Implied),

    // INY
    0xC8u8 => OpCode::new("INY", 1, 2, AddressingMode::Implied),

    // JMP
    0x4Cu8 => OpCode::new("JMP", 3, 3, AddressingMode::Absolute),
    0x6Cu8 => OpCode::new("JMP", 3, 5, AddressingMode::Indirect),

    // JSR
    0x20u8 => OpCode::new("JSR", 3, 6, AddressingMode::Absolute),

    // LDA
    0xA9u8 => OpCode::new("LDA", 2, 2, AddressingMode::Immediate),
    0xA5u8 => OpCode::new("LDA", 2, 3, AddressingMode::ZeroPage),
    0xB5u8 => OpCode::new("LDA", 2, 4, AddressingMode::ZeroPageX),
    0xADu8 => OpCode::new("LDA", 3, 4, AddressingMode::Absolute),
    0xBDu8 => OpCode::new("LDA", 3, 4, AddressingMode::AbsoluteX),
    0xB9u8 => OpCode::new("LDA", 3, 4, AddressingMode::AbsoluteY),
    0xA1u8 => OpCode::new("LDA", 2, 6, AddressingMode::IndirectX),
    0xB1u8 => OpCode::new("LDA", 2, 5, AddressingMode::IndirectY),

    // LDX
    0xA2u8 => OpCode::new("LDX", 2, 2, AddressingMode::Immediate),
    0xA6u8 => OpCode::new("LDX", 2, 3, AddressingMode::ZeroPage),
    0xB6u8 => OpCode::new("LDX", 2, 4, AddressingMode::ZeroPageY),
    0xAEu8 => OpCode::new("LDX", 3, 4, AddressingMode::Absolute),
    0xBEu8 => OpCode::new("LDX", 3, 4, AddressingMode::AbsoluteY),

    // LDY
    0xA0u8 => OpCode::new("LDY", 2, 2, AddressingMode::Immediate),
    0xA4u8 => OpCode::new("LDY", 2, 3, AddressingMode::ZeroPage),
    0xB4u8 => OpCode::new("LDY", 2, 4, AddressingMode::ZeroPageX),
    0xACu8 => OpCode::new("LDY", 3, 4, AddressingMode::Absolute),
    0xBCu8 => OpCode::new("LDY", 3, 4, AddressingMode::AbsoluteX),

    // LSR
    0x4Au8 => OpCode::new("LSR", 1, 2, AddressingMode::Accumulator),
    0x46u8 => OpCode::new("LSR", 2, 5, AddressingMode::ZeroPage),
    0x56u8 => OpCode::new("LSR", 2, 6, AddressingMode::ZeroPageX),
    0x4Eu8 => OpCode::new("LSR", 3, 6, AddressingMode::Absolute),
    0x5Eu8 => OpCode::new("LSR", 3, 7, AddressingMode::AbsoluteX),

    // NOP
    0xEAu8 => OpCode::new("NOP", 1, 2, AddressingMode::Implied),

    // ORA
    0x09u8 => OpCode::new("ORA", 2, 2, AddressingMode::Immediate),
    0x05u8 => OpCode::new("ORA", 2, 3, AddressingMode::ZeroPage),
    0x15u8 => OpCode::new("ORA", 2, 4, AddressingMode::ZeroPageX),
    0x0Du8 => OpCode::new("ORA", 3, 4, AddressingMode::Absolute),
    0x1Du8 => OpCode::new("ORA", 3, 4, AddressingMode::AbsoluteX),
    0x19u8 => OpCode::new("ORA", 3, 4, AddressingMode::AbsoluteY),
    0x01u8 => OpCode::new("ORA", 2, 6, AddressingMode::IndirectX),
    0x11u8 => OpCode::new("ORA", 2, 5, AddressingMode::IndirectY),

    // PHA
    0x48u8 => OpCode::new("PHA", 1, 3, AddressingMode::Implied),

    // PHP
    0x08u8 => OpCode::new("PHP", 1, 3, AddressingMode::Implied),

    // PLA
    0x68u8 => OpCode::new("PLA", 1, 4, AddressingMode::Implied),

    // PLP
    0x28u8 => OpCode::new("PLP", 1, 4, AddressingMode::Implied),

    // ROL
    0x2Au8 => OpCode::new("ROL", 1, 2, AddressingMode::Accumulator),
    0x26u8 => OpCode::new("ROL", 2, 5, AddressingMode::ZeroPage),
    0x36u8 => OpCode::new("ROL", 2, 6, AddressingMode::ZeroPageX),
    0x2Eu8 => OpCode::new("ROL", 3, 6, AddressingMode::Absolute),
    0x3Eu8 => OpCode::new("ROL", 3, 7, AddressingMode::AbsoluteX),

    // ROR
    0x6Au8 => OpCode::new("ROR", 1, 2, AddressingMode::Accumulator),
    0x66u8 => OpCode::new("ROR", 2, 5, AddressingMode::ZeroPage),
    0x76u8 => OpCode::new("ROR", 2, 6, AddressingMode::ZeroPageX),
    0x6Eu8 => OpCode::new("ROR", 3, 6, AddressingMode::Absolute),
    0x7Eu8 => OpCode::new("ROR", 3, 7, AddressingMode::AbsoluteX),

    // RTI
    0x40u8 => OpCode::new("RTI", 1, 6, AddressingMode::Implied),

    // RTS
    0x60u8 => OpCode::new("RTS", 1, 6, AddressingMode::Implied),

    // SBC
    0xE9u8 => OpCode::new("SBC", 2, 2, AddressingMode::Immediate),
    0xE5u8 => OpCode::new("SBC", 2, 3, AddressingMode::ZeroPage),
    0xF5u8 => OpCode::new("SBC", 2, 4, AddressingMode::ZeroPageX),
    0xEDu8 => OpCode::new("SBC", 3, 4, AddressingMode::Absolute),
    0xFDu8 => OpCode::new("SBC", 3, 4, AddressingMode::AbsoluteX),
    0xF9u8 => OpCode::new("SBC", 3, 4, AddressingMode::AbsoluteY),
    0xE1u8 => OpCode::new("SBC", 2, 6, AddressingMode::IndirectX),
    0xF1u8 => OpCode::new("SBC", 2, 5, AddressingMode::IndirectY),

    // Flag Setters
    0x38u8 => OpCode::new("SEC", 1, 2, AddressingMode::Implied),
    0xF8u8 => OpCode::new("SED", 1, 2, AddressingMode::Implied),
    0x78u8 => OpCode::new("SEI", 1, 2, AddressingMode::Implied),
 
    // STA
    0x85u8 => OpCode::new("STA", 2, 3, AddressingMode::ZeroPage),
    0x95u8 => OpCode::new("STA", 2, 4, AddressingMode::ZeroPageX),
    0x8Du8 => OpCode::new("STA", 3, 4, AddressingMode::Absolute),
    0x9Du8 => OpCode::new("STA", 3, 5, AddressingMode::AbsoluteX),
    0x99u8 => OpCode::new("STA", 3, 5, AddressingMode::AbsoluteY),
    0x81u8 => OpCode::new("STA", 2, 6, AddressingMode::IndirectX),
    0x91u8 => OpCode::new("STA", 2, 6, AddressingMode::IndirectY),

    // STX
    0x86u8 => OpCode::new("STX", 2, 3, AddressingMode::ZeroPage),
    0x96u8 => OpCode::new("STX", 2, 4, AddressingMode::ZeroPageY),
    0x8Eu8 => OpCode::new("STX", 3, 4, AddressingMode::Absolute),

    // STY
    0x84u8 => OpCode::new("STY", 2, 3, AddressingMode::ZeroPage),
    0x94u8 => OpCode::new("STY", 2, 4, AddressingMode::ZeroPageY),
    0x8Cu8 => OpCode::new("STY", 3, 4, AddressingMode::Absolute),

    // TAX
    0xAAu8 => OpCode::new("TAX", 1, 2, AddressingMode::Implied),

    // TAY
    0xA8u8 => OpCode::new("TAY", 1, 2, AddressingMode::Implied),

    // TSX
    0xBAu8 => OpCode::new("TSX", 1, 2, AddressingMode::Implied),

    // TXA
    0x8Au8 => OpCode::new("TXA", 1, 2, AddressingMode::Implied),

    // TXS
    0x9au8 => OpCode::new("TXS", 1, 2, AddressingMode::Implied),

    // TYA
    0x98u8 => OpCode::new("TYA", 1, 2, AddressingMode::Implied),

    // BRK
    0x00u8 => OpCode::new("BRK", 1, 7, AddressingMode::Implied),

    // Unofficial instructions
    // https://www.nesdev.org/wiki/Programming_with_unofficial_opcodes

    // NOP
    0x1Au8 => OpCode::new("NOP", 1, 2, AddressingMode::Implied),
    0x3Au8 => OpCode::new("NOP", 1, 2, AddressingMode::Implied),
    0x5Au8 => OpCode::new("NOP", 1, 2, AddressingMode::Implied),
    0x7Au8 => OpCode::new("NOP", 1, 2, AddressingMode::Implied),
    0xDAu8 => OpCode::new("NOP", 1, 2, AddressingMode::Implied),
    0xFAu8 => OpCode::new("NOP", 1, 2, AddressingMode::Implied),

    // IGN
    0x0Cu8 => OpCode::new("IGN", 3, 4, AddressingMode::Absolute),
    0x1Cu8 => OpCode::new("IGN", 3, 4, AddressingMode::AbsoluteX),
    0x3Cu8 => OpCode::new("IGN", 3, 4, AddressingMode::AbsoluteX),
    0x5Cu8 => OpCode::new("IGN", 3, 4, AddressingMode::AbsoluteX),
    0x7Cu8 => OpCode::new("IGN", 3, 4, AddressingMode::AbsoluteX),
    0xDCu8 => OpCode::new("IGN", 3, 4, AddressingMode::AbsoluteX),
    0xFCu8 => OpCode::new("IGN", 3, 4, AddressingMode::AbsoluteX),
    0x04u8 => OpCode::new("IGN", 2, 3, AddressingMode::IndirectY),
    0x44u8 => OpCode::new("IGN", 2, 3, AddressingMode::IndirectY),
    0x14u8 => OpCode::new("IGN", 2, 4, AddressingMode::IndirectX),
    0x34u8 => OpCode::new("IGN", 2, 4, AddressingMode::IndirectX),
    0x54u8 => OpCode::new("IGN", 2, 4, AddressingMode::IndirectX),
    0x74u8 => OpCode::new("IGN", 2, 4, AddressingMode::IndirectX),
    0xD4u8 => OpCode::new("IGN", 2, 4, AddressingMode::IndirectX),
    0xF4u8 => OpCode::new("IGN", 2, 4, AddressingMode::IndirectX),

    // SKB
    0x80u8 => OpCode::new("SKB", 2, 2, AddressingMode::Immediate),
    0x82u8 => OpCode::new("SKB", 2, 2, AddressingMode::Immediate),
    0x89u8 => OpCode::new("SKB", 2, 2, AddressingMode::Immediate),
    0xC2u8 => OpCode::new("SKB", 2, 2, AddressingMode::Immediate),
    0xE2u8 => OpCode::new("SKB", 2, 2, AddressingMode::Immediate),
};