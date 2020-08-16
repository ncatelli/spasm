use crate::instruction_set::op_codes::OpCodeOctal;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mnemonic {
    // Load-Store
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,

    // Arithmetic
    ADC,
    SBC,
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,

    // Shift and Rotate
    ASL,
    LSR,
    ROL,
    ROR,
    AND,
    ORA,
    EOR,

    // Compare and Test Bit
    CMP,
    CPX,
    CPY,
    BIT,

    // Branch
    BCC,
    BCS,
    BNE,
    BEQ,
    BPL,
    BMI,
    BVC,
    BVS,

    // Transfer
    TAX,
    TXA,
    TAY,
    TYA,
    TSX,
    TXS,

    // Stack
    PHA,
    PLA,
    PHP,
    PLP,

    // Subroutines and Jump
    JMP,
    JSR,
    RTS,
    RTI,

    // Set and Clear
    CLC,
    SEC,
    CLD,
    SED,
    CLI,
    SEI,
    CLV,

    // Misc
    BRK,
    NOP,
}

impl From<OpCode> for Mnemonic {
    fn from(oco: OpCode) -> Self {
        match oco.into() {
            (0, 0, 0) => Mnemonic::BRK,
            (0, 2, 0) => Mnemonic::PHP,
            (0, 4, 0) => Mnemonic::BPL,
            (0, 6, 0) => Mnemonic::CLC,
            (1, 0, 0) => Mnemonic::JSR,
            (1, 1, 0) | (1, 3, 0) => Mnemonic::BIT,
            (1, 2, 0) => Mnemonic::PLP,
            (1, 4, 0) => Mnemonic::BMI,
            (1, 6, 0) => Mnemonic::SEC,
            (2, 0, 0) => Mnemonic::RTI,
            (2, 2, 0) => Mnemonic::PHA,
            (2, 3, 0) | (3, 3, 0) => Mnemonic::JMP,
            (2, 4, 0) => Mnemonic::BVC,
            (2, 6, 0) => Mnemonic::CLI,
            (3, 0, 0) => Mnemonic::RTS,
            (3, 2, 0) => Mnemonic::PLA,
            (3, 4, 0) => Mnemonic::BVS,
            (3, 6, 0) => Mnemonic::SEI,
            (4, 1, 0) | (4, 3, 0) | (4, 5, 0) => Mnemonic::STY,
            (4, 2, 0) => Mnemonic::DEY,
            (4, 4, 0) => Mnemonic::BCC,
            (4, 6, 0) => Mnemonic::TYA,
            (5, 0, 0) | (5, 1, 0) | (5, 3, 0) | (5, 5, 0) | (5, 7, 0) => Mnemonic::LDY,
            (5, 2, 0) => Mnemonic::TAY,
            (5, 4, 0) => Mnemonic::BCS,
            (5, 6, 0) => Mnemonic::CLV,
            (6, 0, 0) | (6, 1, 0) | (6, 3, 0) => Mnemonic::CPY,
            (6, 2, 0) => Mnemonic::INY,
            (6, 4, 0) => Mnemonic::BNE,
            (6, 6, 0) => Mnemonic::CLD,
            (7, 0, 0) | (7, 1, 0) | (7, 3, 0) => Mnemonic::CPX,
            (7, 2, 0) => Mnemonic::INX,
            (7, 4, 0) => Mnemonic::BEQ,
            (7, 6, 0) => Mnemonic::SED,
            (0, _, 1) => Mnemonic::ORA,
            (1, _, 1) => Mnemonic::AND,
            (2, _, 1) => Mnemonic::EOR,
            (3, _, 1) => Mnemonic::ADC,
            (4, 0..=1, 1) => Mnemonic::STA,
            (4, 3..=7, 1) => Mnemonic::STA,
            (5, _, 1) => Mnemonic::LDA,
            (6, _, 1) => Mnemonic::CMP,
            (7, _, 1) => Mnemonic::SBC,
            (0, 1..=3, 2) => Mnemonic::ASL,
            (0, 5, 2) => Mnemonic::ASL,
            (0, 7, 2) => Mnemonic::ASL,
            (1, 1..=3, 2) => Mnemonic::ROL,
            (1, 5, 2) => Mnemonic::ROL,
            (1, 7, 2) => Mnemonic::ROL,
            (2, 1..=3, 2) => Mnemonic::LSR,
            (2, 5, 2) => Mnemonic::LSR,
            (2, 7, 2) => Mnemonic::LSR,
            (3, 1..=3, 2) => Mnemonic::ROR,
            (3, 5, 2) => Mnemonic::ROR,
            (3, 7, 2) => Mnemonic::ROR,
            (4, 1, 2) | (4, 3, 2) | (4, 5, 2) => Mnemonic::STX,
            (4, 2, 2) => Mnemonic::TXA,
            (4, 6, 2) => Mnemonic::TXS,
            (5, 0..=1, 2) | (5, 3, 2) | (5, 5, 2) | (5, 7, 2) => Mnemonic::LDX,
            (5, 2, 2) => Mnemonic::TAX,
            (5, 6, 2) => Mnemonic::TSX,
            (6, 1, 2) | (6, 3, 2) | (6, 5, 2) | (6, 7, 2) => Mnemonic::DEC,
            (6, 2, 2) => Mnemonic::DEX,
            (7, 1, 2) | (7, 3, 2) | (7, 5, 2) | (7, 7, 2) => Mnemonic::INC,
            (7, 2, 2) => Mnemonic::NOP,

            // Placeholder... probably should convert to HLT
            _ => Mnemonic::NOP,
        }
    }
}
