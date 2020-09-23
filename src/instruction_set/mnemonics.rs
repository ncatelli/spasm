use std::convert::TryFrom;

use crate::addressing;

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

impl addressing::SizeOf for Mnemonic {
    fn size_of(&self) -> usize {
        1
    }
}

impl TryFrom<&str> for Mnemonic {
    type Error = String;

    fn try_from(src: &str) -> Result<Mnemonic, Self::Error> {
        match src {
            "lda" => Ok(Mnemonic::LDA),
            "ldx" => Ok(Mnemonic::LDX),
            "ldy" => Ok(Mnemonic::LDY),
            "sta" => Ok(Mnemonic::STA),
            "stx" => Ok(Mnemonic::STX),
            "sty" => Ok(Mnemonic::STY),
            "adc" => Ok(Mnemonic::ADC),
            "sbc" => Ok(Mnemonic::SBC),
            "inc" => Ok(Mnemonic::INC),
            "inx" => Ok(Mnemonic::INX),
            "iny" => Ok(Mnemonic::INY),
            "dec" => Ok(Mnemonic::DEC),
            "dex" => Ok(Mnemonic::DEX),
            "dey" => Ok(Mnemonic::DEY),
            "asl" => Ok(Mnemonic::ASL),
            "lsr" => Ok(Mnemonic::LSR),
            "rol" => Ok(Mnemonic::ROL),
            "ror" => Ok(Mnemonic::ROR),
            "and" => Ok(Mnemonic::AND),
            "ora" => Ok(Mnemonic::ORA),
            "eor" => Ok(Mnemonic::EOR),
            "cmp" => Ok(Mnemonic::CMP),
            "cpx" => Ok(Mnemonic::CPX),
            "cpy" => Ok(Mnemonic::CPY),
            "bit" => Ok(Mnemonic::BIT),
            "bcc" => Ok(Mnemonic::BCC),
            "bcs" => Ok(Mnemonic::BCS),
            "bnd" => Ok(Mnemonic::BNE),
            "beq" => Ok(Mnemonic::BEQ),
            "bpl" => Ok(Mnemonic::BPL),
            "bmi" => Ok(Mnemonic::BMI),
            "bvc" => Ok(Mnemonic::BVC),
            "bvs" => Ok(Mnemonic::BVS),
            "tax" => Ok(Mnemonic::TAX),
            "txa" => Ok(Mnemonic::TXA),
            "tay" => Ok(Mnemonic::TAY),
            "tya" => Ok(Mnemonic::TYA),
            "tsx" => Ok(Mnemonic::TSX),
            "txs" => Ok(Mnemonic::TXS),
            "pha" => Ok(Mnemonic::PHA),
            "pla" => Ok(Mnemonic::PLA),
            "php" => Ok(Mnemonic::PHP),
            "plp" => Ok(Mnemonic::PLP),
            "jmp" => Ok(Mnemonic::JMP),
            "jsr" => Ok(Mnemonic::JSR),
            "rts" => Ok(Mnemonic::RTS),
            "rti" => Ok(Mnemonic::RTI),
            "clc" => Ok(Mnemonic::CLC),
            "sec" => Ok(Mnemonic::SEC),
            "cld" => Ok(Mnemonic::CLD),
            "sed" => Ok(Mnemonic::SED),
            "cli" => Ok(Mnemonic::CLI),
            "sei" => Ok(Mnemonic::SEI),
            "clv" => Ok(Mnemonic::CLV),
            "brk" => Ok(Mnemonic::BRK),
            "nop" => Ok(Mnemonic::NOP),
            _ => Err(format!("Invalid instruction: {}", src)),
        }
    }
}
