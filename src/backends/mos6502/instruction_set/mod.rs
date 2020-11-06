pub mod address_mode;
pub use address_mode::AddressMode;
use address_mode::AddressModeOrReference;
pub mod mnemonics;
use crate::addressing;
use crate::Emitter;
pub use mnemonics::Mnemonic;
use std::fmt;

#[cfg(test)]
mod tests;

/// OpCode represents an unsigned 8bit value.
pub type OpCode = u8;

/// Instruction represents a single 6502 instruction containing a mnemonic,
/// and either a static address_mode or a label.
#[derive(Clone, PartialEq, Debug)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub amor: AddressModeOrReference,
}

impl Instruction {
    pub fn new(mnemonic: Mnemonic, amor: AddressModeOrReference) -> Self {
        Self { mnemonic, amor }
    }
}

impl addressing::SizeOf for Instruction {
    fn size_of(&self) -> usize {
        self.mnemonic.size_of() + self.amor.size_of()
    }
}

impl From<StaticInstruction> for Instruction {
    fn from(si: StaticInstruction) -> Self {
        Self {
            mnemonic: si.mnemonic,
            amor: AddressModeOrReference::AddressMode(si.address_mode),
        }
    }
}

/// UnknownInstructionErr represents an Instruction that is unrepresentable or unknown.
#[derive(Debug, Copy, Clone)]
pub struct UnknownInstructionErr {
    mnemonic: Mnemonic,
    operand: AddressMode,
}

impl UnknownInstructionErr {
    pub fn new(mnemonic: Mnemonic, operand: AddressMode) -> Self {
        Self { mnemonic, operand }
    }
}

impl fmt::Display for UnknownInstructionErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unknown instruction: {:?} {:?}",
            &self.mnemonic, &self.operand
        )
    }
}

/// StaticInstruction represents a single 6502 instruction containing a mnemonic,
/// and static address mode, mapping directly to an address or byte value.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StaticInstruction {
    pub mnemonic: Mnemonic,
    pub address_mode: AddressMode,
}

impl StaticInstruction {
    pub fn new(mnemonic: Mnemonic, address_mode: AddressMode) -> Self {
        Self {
            mnemonic,
            address_mode,
        }
    }
}

impl addressing::SizeOf for StaticInstruction {
    fn size_of(&self) -> usize {
        self.mnemonic.size_of() + self.address_mode.size_of()
    }
}

impl Emitter<Result<OpCode, UnknownInstructionErr>> for StaticInstruction {
    fn emit(&self) -> Result<OpCode, UnknownInstructionErr> {
        let mc = match (self.mnemonic, self.address_mode) {
            (Mnemonic::BRK, AddressMode::Implied) => 0x00,
            (Mnemonic::ORA, AddressMode::IndexedIndirect(_)) => 0x01,
            (Mnemonic::ORA, AddressMode::ZeroPage(_)) => 0x05,
            (Mnemonic::ASL, AddressMode::ZeroPage(_)) => 0x06,
            (Mnemonic::PHP, AddressMode::Implied) => 0x08,
            (Mnemonic::ORA, AddressMode::Immediate(_)) => 0x09,
            (Mnemonic::ASL, AddressMode::Accumulator) => 0x0a,
            (Mnemonic::ORA, AddressMode::Absolute(_)) => 0x0d,
            (Mnemonic::ASL, AddressMode::Absolute(_)) => 0x0e,
            (Mnemonic::BPL, AddressMode::Relative(_)) => 0x10,
            (Mnemonic::ORA, AddressMode::IndirectIndexed(_)) => 0x11,
            (Mnemonic::ORA, AddressMode::ZeroPageIndexedWithX(_)) => 0x15,
            (Mnemonic::ASL, AddressMode::ZeroPageIndexedWithX(_)) => 0x16,
            (Mnemonic::CLC, AddressMode::Implied) => 0x18,
            (Mnemonic::ORA, AddressMode::AbsoluteIndexedWithY(_)) => 0x19,
            (Mnemonic::ORA, AddressMode::AbsoluteIndexedWithX(_)) => 0x1d,
            (Mnemonic::ASL, AddressMode::AbsoluteIndexedWithX(_)) => 0x1e,
            (Mnemonic::JSR, AddressMode::Absolute(_)) => 0x20,
            (Mnemonic::AND, AddressMode::IndexedIndirect(_)) => 0x21,
            (Mnemonic::BIT, AddressMode::ZeroPage(_)) => 0x24,
            (Mnemonic::AND, AddressMode::ZeroPage(_)) => 0x25,
            (Mnemonic::ROL, AddressMode::ZeroPage(_)) => 0x26,
            (Mnemonic::PLP, AddressMode::Implied) => 0x28,
            (Mnemonic::AND, AddressMode::Immediate(_)) => 0x29,
            (Mnemonic::ROL, AddressMode::Accumulator) => 0x2a,
            (Mnemonic::BIT, AddressMode::Absolute(_)) => 0x2c,
            (Mnemonic::AND, AddressMode::Absolute(_)) => 0x2d,
            (Mnemonic::ROL, AddressMode::Absolute(_)) => 0x2e,
            (Mnemonic::BMI, AddressMode::Relative(_)) => 0x30,
            (Mnemonic::AND, AddressMode::IndirectIndexed(_)) => 0x31,
            (Mnemonic::AND, AddressMode::ZeroPageIndexedWithX(_)) => 0x35,
            (Mnemonic::ROL, AddressMode::ZeroPageIndexedWithX(_)) => 0x36,
            (Mnemonic::SEC, AddressMode::Implied) => 0x38,
            (Mnemonic::AND, AddressMode::AbsoluteIndexedWithY(_)) => 0x39,
            (Mnemonic::AND, AddressMode::AbsoluteIndexedWithX(_)) => 0x3d,
            (Mnemonic::ROL, AddressMode::AbsoluteIndexedWithX(_)) => 0x3e,
            (Mnemonic::RTI, AddressMode::Implied) => 0x40,
            (Mnemonic::EOR, AddressMode::IndexedIndirect(_)) => 0x41,
            (Mnemonic::EOR, AddressMode::ZeroPage(_)) => 0x45,
            (Mnemonic::LSR, AddressMode::ZeroPage(_)) => 0x46,
            (Mnemonic::PHA, AddressMode::Implied) => 0x48,
            (Mnemonic::EOR, AddressMode::Immediate(_)) => 0x49,
            (Mnemonic::LSR, AddressMode::Accumulator) => 0x4a,
            (Mnemonic::JMP, AddressMode::Absolute(_)) => 0x4c,
            (Mnemonic::EOR, AddressMode::Absolute(_)) => 0x4d,
            (Mnemonic::LSR, AddressMode::Absolute(_)) => 0x4e,
            (Mnemonic::BVC, AddressMode::Relative(_)) => 0x50,
            (Mnemonic::EOR, AddressMode::IndirectIndexed(_)) => 0x51,
            (Mnemonic::EOR, AddressMode::ZeroPageIndexedWithX(_)) => 0x55,
            (Mnemonic::LSR, AddressMode::ZeroPageIndexedWithX(_)) => 0x56,
            (Mnemonic::CLI, AddressMode::Implied) => 0x58,
            (Mnemonic::EOR, AddressMode::AbsoluteIndexedWithY(_)) => 0x59,
            (Mnemonic::EOR, AddressMode::AbsoluteIndexedWithX(_)) => 0x5d,
            (Mnemonic::LSR, AddressMode::AbsoluteIndexedWithX(_)) => 0x5e,
            (Mnemonic::RTS, AddressMode::Implied) => 0x60,
            (Mnemonic::ADC, AddressMode::IndexedIndirect(_)) => 0x61,
            (Mnemonic::ADC, AddressMode::ZeroPage(_)) => 0x65,
            (Mnemonic::ROR, AddressMode::ZeroPage(_)) => 0x66,
            (Mnemonic::PLA, AddressMode::Implied) => 0x68,
            (Mnemonic::ADC, AddressMode::Immediate(_)) => 0x69,
            (Mnemonic::ROR, AddressMode::Accumulator) => 0x6a,
            (Mnemonic::JMP, AddressMode::Indirect(_)) => 0x6c,
            (Mnemonic::ADC, AddressMode::Absolute(_)) => 0x6d,
            (Mnemonic::ROR, AddressMode::Absolute(_)) => 0x6e,
            (Mnemonic::BVS, AddressMode::Relative(_)) => 0x70,
            (Mnemonic::ADC, AddressMode::IndirectIndexed(_)) => 0x71,
            (Mnemonic::ADC, AddressMode::ZeroPageIndexedWithX(_)) => 0x75,
            (Mnemonic::ROR, AddressMode::ZeroPageIndexedWithX(_)) => 0x76,
            (Mnemonic::SEI, AddressMode::Implied) => 0x78,
            (Mnemonic::ADC, AddressMode::AbsoluteIndexedWithY(_)) => 0x79,
            (Mnemonic::ADC, AddressMode::AbsoluteIndexedWithX(_)) => 0x7d,
            (Mnemonic::ROR, AddressMode::AbsoluteIndexedWithX(_)) => 0x7e,
            (Mnemonic::STA, AddressMode::IndexedIndirect(_)) => 0x81,
            (Mnemonic::STY, AddressMode::ZeroPage(_)) => 0x84,
            (Mnemonic::STA, AddressMode::ZeroPage(_)) => 0x85,
            (Mnemonic::STX, AddressMode::ZeroPage(_)) => 0x86,
            (Mnemonic::DEY, AddressMode::Implied) => 0x88,
            (Mnemonic::TXA, AddressMode::Implied) => 0x8a,
            (Mnemonic::STY, AddressMode::Absolute(_)) => 0x8c,
            (Mnemonic::STA, AddressMode::Absolute(_)) => 0x8d,
            (Mnemonic::STX, AddressMode::Absolute(_)) => 0x8e,
            (Mnemonic::BCC, AddressMode::Relative(_)) => 0x90,
            (Mnemonic::STA, AddressMode::IndirectIndexed(_)) => 0x91,
            (Mnemonic::STY, AddressMode::ZeroPageIndexedWithX(_)) => 0x94,
            (Mnemonic::STA, AddressMode::ZeroPageIndexedWithX(_)) => 0x95,
            (Mnemonic::STX, AddressMode::ZeroPageIndexedWithY(_)) => 0x96,
            (Mnemonic::TYA, AddressMode::Implied) => 0x98,
            (Mnemonic::STA, AddressMode::AbsoluteIndexedWithY(_)) => 0x99,
            (Mnemonic::TXS, AddressMode::Implied) => 0x9a,
            (Mnemonic::STA, AddressMode::AbsoluteIndexedWithX(_)) => 0x9d,
            (Mnemonic::LDY, AddressMode::Immediate(_)) => 0xa0,
            (Mnemonic::LDA, AddressMode::IndexedIndirect(_)) => 0xa1,
            (Mnemonic::LDX, AddressMode::Immediate(_)) => 0xa2,
            (Mnemonic::LDY, AddressMode::ZeroPage(_)) => 0xa4,
            (Mnemonic::LDA, AddressMode::ZeroPage(_)) => 0xa5,
            (Mnemonic::LDX, AddressMode::ZeroPage(_)) => 0xa6,
            (Mnemonic::TAY, AddressMode::Implied) => 0xa8,
            (Mnemonic::LDA, AddressMode::Immediate(_)) => 0xa9,
            (Mnemonic::TAX, AddressMode::Implied) => 0xaa,
            (Mnemonic::LDY, AddressMode::Absolute(_)) => 0xac,
            (Mnemonic::LDA, AddressMode::Absolute(_)) => 0xad,
            (Mnemonic::LDX, AddressMode::Absolute(_)) => 0xae,
            (Mnemonic::BCS, AddressMode::Relative(_)) => 0xb0,
            (Mnemonic::LDA, AddressMode::IndirectIndexed(_)) => 0xb1,
            (Mnemonic::LDY, AddressMode::ZeroPageIndexedWithX(_)) => 0xb4,
            (Mnemonic::LDA, AddressMode::ZeroPageIndexedWithX(_)) => 0xb5,
            (Mnemonic::LDX, AddressMode::ZeroPageIndexedWithY(_)) => 0xb6,
            (Mnemonic::CLV, AddressMode::Implied) => 0xb8,
            (Mnemonic::LDA, AddressMode::AbsoluteIndexedWithY(_)) => 0xb9,
            (Mnemonic::TSX, AddressMode::Implied) => 0xba,
            (Mnemonic::LDY, AddressMode::AbsoluteIndexedWithX(_)) => 0xbc,
            (Mnemonic::LDA, AddressMode::AbsoluteIndexedWithX(_)) => 0xbd,
            (Mnemonic::LDX, AddressMode::AbsoluteIndexedWithY(_)) => 0xbe,
            (Mnemonic::CPY, AddressMode::Immediate(_)) => 0xc0,
            (Mnemonic::CMP, AddressMode::IndexedIndirect(_)) => 0xc1,
            (Mnemonic::CPY, AddressMode::ZeroPage(_)) => 0xc4,
            (Mnemonic::CMP, AddressMode::ZeroPage(_)) => 0xc5,
            (Mnemonic::DEC, AddressMode::ZeroPage(_)) => 0xc6,
            (Mnemonic::INY, AddressMode::Implied) => 0xc8,
            (Mnemonic::CMP, AddressMode::Immediate(_)) => 0xc9,
            (Mnemonic::DEX, AddressMode::Implied) => 0xca,
            (Mnemonic::CPY, AddressMode::Absolute(_)) => 0xcc,
            (Mnemonic::CMP, AddressMode::Absolute(_)) => 0xcd,
            (Mnemonic::DEC, AddressMode::Absolute(_)) => 0xce,
            (Mnemonic::BNE, AddressMode::Relative(_)) => 0xd0,
            (Mnemonic::CMP, AddressMode::IndirectIndexed(_)) => 0xd1,
            (Mnemonic::CMP, AddressMode::ZeroPageIndexedWithX(_)) => 0xd5,
            (Mnemonic::DEC, AddressMode::ZeroPageIndexedWithX(_)) => 0xd6,
            (Mnemonic::CLD, AddressMode::Implied) => 0xd8,
            (Mnemonic::CMP, AddressMode::AbsoluteIndexedWithY(_)) => 0xd9,
            (Mnemonic::CMP, AddressMode::AbsoluteIndexedWithX(_)) => 0xdd,
            (Mnemonic::DEC, AddressMode::AbsoluteIndexedWithX(_)) => 0xde,
            (Mnemonic::CPX, AddressMode::Immediate(_)) => 0xe0,
            (Mnemonic::SBC, AddressMode::IndexedIndirect(_)) => 0xe1,
            (Mnemonic::CPX, AddressMode::ZeroPage(_)) => 0xe4,
            (Mnemonic::SBC, AddressMode::ZeroPage(_)) => 0xe5,
            (Mnemonic::INC, AddressMode::ZeroPage(_)) => 0xe6,
            (Mnemonic::INX, AddressMode::Implied) => 0xe8,
            (Mnemonic::SBC, AddressMode::Immediate(_)) => 0xe9,
            (Mnemonic::NOP, AddressMode::Implied) => 0xea,
            (Mnemonic::CPX, AddressMode::Absolute(_)) => 0xec,
            (Mnemonic::SBC, AddressMode::Absolute(_)) => 0xed,
            (Mnemonic::INC, AddressMode::Absolute(_)) => 0xee,
            (Mnemonic::BEQ, AddressMode::Relative(_)) => 0xf0,
            (Mnemonic::SBC, AddressMode::IndirectIndexed(_)) => 0xf1,
            (Mnemonic::SBC, AddressMode::ZeroPageIndexedWithX(_)) => 0xf5,
            (Mnemonic::INC, AddressMode::ZeroPageIndexedWithX(_)) => 0xf6,
            (Mnemonic::SED, AddressMode::Implied) => 0xf8,
            (Mnemonic::SBC, AddressMode::AbsoluteIndexedWithY(_)) => 0xf9,
            (Mnemonic::SBC, AddressMode::AbsoluteIndexedWithX(_)) => 0xfd,
            (Mnemonic::INC, AddressMode::AbsoluteIndexedWithX(_)) => 0xfe,
            _ => 0xff, // 0xff represents an unknown opcode
        };

        if mc == 0xff {
            Err(UnknownInstructionErr::new(self.mnemonic, self.address_mode))
        } else {
            Ok(mc)
        }
    }
}

impl Emitter<Result<Vec<u8>, UnknownInstructionErr>> for StaticInstruction {
    fn emit(&self) -> Result<Vec<u8>, UnknownInstructionErr> {
        let opcode_res: Result<u8, UnknownInstructionErr> = self.emit();
        let opcode = opcode_res?;
        let operand = self.address_mode.emit();
        Ok(vec![opcode].into_iter().chain(operand).collect())
    }
}

#[allow(unused_macros)]
macro_rules! instruction {
    ($mnemonic:expr, $amos:expr) => {
        $crate::backends::mos6502::instruction_set::Instruction::new($mnemonic, $amos)
    };
}

#[allow(unused_macros)]
macro_rules! static_instruction {
    ($mnemonic:expr, $am:expr) => {
        $crate::backends::mos6502::instruction_set::StaticInstruction::new($mnemonic, $am)
    };
}
