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

pub enum AddressMode {
    Accumlator,
    Implied,
    Immediate(u8),
    Absolute(u16),
    ZeroPage(u8),
    Relative(u8),
    AbsoluteIndirect(u16),
    AbsoluteIndexedWithX(u16),
    AbsoluteIndexedWithY(u16),
    ZeroPageIndexedWithX(u8),
    ZeroPageIndexedWithY(u8),
    ZeroPageIndexedIndirect(u8),
    ZeroPageIndirectIndexedWithY(u8),
}

pub struct Instruction {
    instruction: Mnemonic,
    address_mode: AddressMode,
}
