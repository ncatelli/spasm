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
