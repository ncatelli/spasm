pub mod addressing_mode;
use crate::addressing;
use crate::Emitter;
use addressing_mode::AddressingModeOrReference;
use isa_mos6502::{addressing_mode::AddressingMode, mnemonic::Mnemonic};
use std::fmt;

#[cfg(test)]
mod tests;

/// OpCode represents an unsigned 8bit value.
pub type OpCode = u8;

impl addressing::SizeOf for Mnemonic {
    fn size_of(&self) -> usize {
        use isa_mos6502::ByteSized;
        self.byte_size()
    }
}
/// Instruction represents a single 6502 instruction containing a mnemonic,
/// and either a static address_mode or a label.
#[derive(Clone, PartialEq, Debug)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub amor: AddressingModeOrReference,
}

impl Instruction {
    pub fn new(mnemonic: Mnemonic, amor: AddressingModeOrReference) -> Self {
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
            amor: AddressingModeOrReference::AddressingMode(si.address_mode),
        }
    }
}

/// UnknownInstructionErr represents an Instruction that is unrepresentable or unknown.
#[derive(Debug, Copy, Clone)]
pub struct UnknownInstructionErr {
    mnemonic: Mnemonic,
    operand: AddressingMode,
}

impl UnknownInstructionErr {
    pub fn new(mnemonic: Mnemonic, operand: AddressingMode) -> Self {
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
    pub address_mode: AddressingMode,
}

impl StaticInstruction {
    pub fn new(mnemonic: Mnemonic, address_mode: AddressingMode) -> Self {
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
            (Mnemonic::BRK, AddressingMode::Implied) => 0x00,
            (Mnemonic::ORA, AddressingMode::XIndexedIndirect(_)) => 0x01,
            (Mnemonic::ORA, AddressingMode::ZeroPage(_)) => 0x05,
            (Mnemonic::ASL, AddressingMode::ZeroPage(_)) => 0x06,
            (Mnemonic::PHP, AddressingMode::Implied) => 0x08,
            (Mnemonic::ORA, AddressingMode::Immediate(_)) => 0x09,
            (Mnemonic::ASL, AddressingMode::Accumulator) => 0x0a,
            (Mnemonic::ORA, AddressingMode::Absolute(_)) => 0x0d,
            (Mnemonic::ASL, AddressingMode::Absolute(_)) => 0x0e,
            (Mnemonic::BPL, AddressingMode::Relative(_)) => 0x10,
            (Mnemonic::ORA, AddressingMode::IndirectYIndexed(_)) => 0x11,
            (Mnemonic::ORA, AddressingMode::ZeroPageIndexedWithX(_)) => 0x15,
            (Mnemonic::ASL, AddressingMode::ZeroPageIndexedWithX(_)) => 0x16,
            (Mnemonic::CLC, AddressingMode::Implied) => 0x18,
            (Mnemonic::ORA, AddressingMode::AbsoluteIndexedWithY(_)) => 0x19,
            (Mnemonic::ORA, AddressingMode::AbsoluteIndexedWithX(_)) => 0x1d,
            (Mnemonic::ASL, AddressingMode::AbsoluteIndexedWithX(_)) => 0x1e,
            (Mnemonic::JSR, AddressingMode::Absolute(_)) => 0x20,
            (Mnemonic::AND, AddressingMode::XIndexedIndirect(_)) => 0x21,
            (Mnemonic::BIT, AddressingMode::ZeroPage(_)) => 0x24,
            (Mnemonic::AND, AddressingMode::ZeroPage(_)) => 0x25,
            (Mnemonic::ROL, AddressingMode::ZeroPage(_)) => 0x26,
            (Mnemonic::PLP, AddressingMode::Implied) => 0x28,
            (Mnemonic::AND, AddressingMode::Immediate(_)) => 0x29,
            (Mnemonic::ROL, AddressingMode::Accumulator) => 0x2a,
            (Mnemonic::BIT, AddressingMode::Absolute(_)) => 0x2c,
            (Mnemonic::AND, AddressingMode::Absolute(_)) => 0x2d,
            (Mnemonic::ROL, AddressingMode::Absolute(_)) => 0x2e,
            (Mnemonic::BMI, AddressingMode::Relative(_)) => 0x30,
            (Mnemonic::AND, AddressingMode::IndirectYIndexed(_)) => 0x31,
            (Mnemonic::AND, AddressingMode::ZeroPageIndexedWithX(_)) => 0x35,
            (Mnemonic::ROL, AddressingMode::ZeroPageIndexedWithX(_)) => 0x36,
            (Mnemonic::SEC, AddressingMode::Implied) => 0x38,
            (Mnemonic::AND, AddressingMode::AbsoluteIndexedWithY(_)) => 0x39,
            (Mnemonic::AND, AddressingMode::AbsoluteIndexedWithX(_)) => 0x3d,
            (Mnemonic::ROL, AddressingMode::AbsoluteIndexedWithX(_)) => 0x3e,
            (Mnemonic::RTI, AddressingMode::Implied) => 0x40,
            (Mnemonic::EOR, AddressingMode::XIndexedIndirect(_)) => 0x41,
            (Mnemonic::EOR, AddressingMode::ZeroPage(_)) => 0x45,
            (Mnemonic::LSR, AddressingMode::ZeroPage(_)) => 0x46,
            (Mnemonic::PHA, AddressingMode::Implied) => 0x48,
            (Mnemonic::EOR, AddressingMode::Immediate(_)) => 0x49,
            (Mnemonic::LSR, AddressingMode::Accumulator) => 0x4a,
            (Mnemonic::JMP, AddressingMode::Absolute(_)) => 0x4c,
            (Mnemonic::EOR, AddressingMode::Absolute(_)) => 0x4d,
            (Mnemonic::LSR, AddressingMode::Absolute(_)) => 0x4e,
            (Mnemonic::BVC, AddressingMode::Relative(_)) => 0x50,
            (Mnemonic::EOR, AddressingMode::IndirectYIndexed(_)) => 0x51,
            (Mnemonic::EOR, AddressingMode::ZeroPageIndexedWithX(_)) => 0x55,
            (Mnemonic::LSR, AddressingMode::ZeroPageIndexedWithX(_)) => 0x56,
            (Mnemonic::CLI, AddressingMode::Implied) => 0x58,
            (Mnemonic::EOR, AddressingMode::AbsoluteIndexedWithY(_)) => 0x59,
            (Mnemonic::EOR, AddressingMode::AbsoluteIndexedWithX(_)) => 0x5d,
            (Mnemonic::LSR, AddressingMode::AbsoluteIndexedWithX(_)) => 0x5e,
            (Mnemonic::RTS, AddressingMode::Implied) => 0x60,
            (Mnemonic::ADC, AddressingMode::XIndexedIndirect(_)) => 0x61,
            (Mnemonic::ADC, AddressingMode::ZeroPage(_)) => 0x65,
            (Mnemonic::ROR, AddressingMode::ZeroPage(_)) => 0x66,
            (Mnemonic::PLA, AddressingMode::Implied) => 0x68,
            (Mnemonic::ADC, AddressingMode::Immediate(_)) => 0x69,
            (Mnemonic::ROR, AddressingMode::Accumulator) => 0x6a,
            (Mnemonic::JMP, AddressingMode::Indirect(_)) => 0x6c,
            (Mnemonic::ADC, AddressingMode::Absolute(_)) => 0x6d,
            (Mnemonic::ROR, AddressingMode::Absolute(_)) => 0x6e,
            (Mnemonic::BVS, AddressingMode::Relative(_)) => 0x70,
            (Mnemonic::ADC, AddressingMode::IndirectYIndexed(_)) => 0x71,
            (Mnemonic::ADC, AddressingMode::ZeroPageIndexedWithX(_)) => 0x75,
            (Mnemonic::ROR, AddressingMode::ZeroPageIndexedWithX(_)) => 0x76,
            (Mnemonic::SEI, AddressingMode::Implied) => 0x78,
            (Mnemonic::ADC, AddressingMode::AbsoluteIndexedWithY(_)) => 0x79,
            (Mnemonic::ADC, AddressingMode::AbsoluteIndexedWithX(_)) => 0x7d,
            (Mnemonic::ROR, AddressingMode::AbsoluteIndexedWithX(_)) => 0x7e,
            (Mnemonic::STA, AddressingMode::XIndexedIndirect(_)) => 0x81,
            (Mnemonic::STY, AddressingMode::ZeroPage(_)) => 0x84,
            (Mnemonic::STA, AddressingMode::ZeroPage(_)) => 0x85,
            (Mnemonic::STX, AddressingMode::ZeroPage(_)) => 0x86,
            (Mnemonic::DEY, AddressingMode::Implied) => 0x88,
            (Mnemonic::TXA, AddressingMode::Implied) => 0x8a,
            (Mnemonic::STY, AddressingMode::Absolute(_)) => 0x8c,
            (Mnemonic::STA, AddressingMode::Absolute(_)) => 0x8d,
            (Mnemonic::STX, AddressingMode::Absolute(_)) => 0x8e,
            (Mnemonic::BCC, AddressingMode::Relative(_)) => 0x90,
            (Mnemonic::STA, AddressingMode::IndirectYIndexed(_)) => 0x91,
            (Mnemonic::STY, AddressingMode::ZeroPageIndexedWithX(_)) => 0x94,
            (Mnemonic::STA, AddressingMode::ZeroPageIndexedWithX(_)) => 0x95,
            (Mnemonic::STX, AddressingMode::ZeroPageIndexedWithY(_)) => 0x96,
            (Mnemonic::TYA, AddressingMode::Implied) => 0x98,
            (Mnemonic::STA, AddressingMode::AbsoluteIndexedWithY(_)) => 0x99,
            (Mnemonic::TXS, AddressingMode::Implied) => 0x9a,
            (Mnemonic::STA, AddressingMode::AbsoluteIndexedWithX(_)) => 0x9d,
            (Mnemonic::LDY, AddressingMode::Immediate(_)) => 0xa0,
            (Mnemonic::LDA, AddressingMode::XIndexedIndirect(_)) => 0xa1,
            (Mnemonic::LDX, AddressingMode::Immediate(_)) => 0xa2,
            (Mnemonic::LDY, AddressingMode::ZeroPage(_)) => 0xa4,
            (Mnemonic::LDA, AddressingMode::ZeroPage(_)) => 0xa5,
            (Mnemonic::LDX, AddressingMode::ZeroPage(_)) => 0xa6,
            (Mnemonic::TAY, AddressingMode::Implied) => 0xa8,
            (Mnemonic::LDA, AddressingMode::Immediate(_)) => 0xa9,
            (Mnemonic::TAX, AddressingMode::Implied) => 0xaa,
            (Mnemonic::LDY, AddressingMode::Absolute(_)) => 0xac,
            (Mnemonic::LDA, AddressingMode::Absolute(_)) => 0xad,
            (Mnemonic::LDX, AddressingMode::Absolute(_)) => 0xae,
            (Mnemonic::BCS, AddressingMode::Relative(_)) => 0xb0,
            (Mnemonic::LDA, AddressingMode::IndirectYIndexed(_)) => 0xb1,
            (Mnemonic::LDY, AddressingMode::ZeroPageIndexedWithX(_)) => 0xb4,
            (Mnemonic::LDA, AddressingMode::ZeroPageIndexedWithX(_)) => 0xb5,
            (Mnemonic::LDX, AddressingMode::ZeroPageIndexedWithY(_)) => 0xb6,
            (Mnemonic::CLV, AddressingMode::Implied) => 0xb8,
            (Mnemonic::LDA, AddressingMode::AbsoluteIndexedWithY(_)) => 0xb9,
            (Mnemonic::TSX, AddressingMode::Implied) => 0xba,
            (Mnemonic::LDY, AddressingMode::AbsoluteIndexedWithX(_)) => 0xbc,
            (Mnemonic::LDA, AddressingMode::AbsoluteIndexedWithX(_)) => 0xbd,
            (Mnemonic::LDX, AddressingMode::AbsoluteIndexedWithY(_)) => 0xbe,
            (Mnemonic::CPY, AddressingMode::Immediate(_)) => 0xc0,
            (Mnemonic::CMP, AddressingMode::XIndexedIndirect(_)) => 0xc1,
            (Mnemonic::CPY, AddressingMode::ZeroPage(_)) => 0xc4,
            (Mnemonic::CMP, AddressingMode::ZeroPage(_)) => 0xc5,
            (Mnemonic::DEC, AddressingMode::ZeroPage(_)) => 0xc6,
            (Mnemonic::INY, AddressingMode::Implied) => 0xc8,
            (Mnemonic::CMP, AddressingMode::Immediate(_)) => 0xc9,
            (Mnemonic::DEX, AddressingMode::Implied) => 0xca,
            (Mnemonic::CPY, AddressingMode::Absolute(_)) => 0xcc,
            (Mnemonic::CMP, AddressingMode::Absolute(_)) => 0xcd,
            (Mnemonic::DEC, AddressingMode::Absolute(_)) => 0xce,
            (Mnemonic::BNE, AddressingMode::Relative(_)) => 0xd0,
            (Mnemonic::CMP, AddressingMode::IndirectYIndexed(_)) => 0xd1,
            (Mnemonic::CMP, AddressingMode::ZeroPageIndexedWithX(_)) => 0xd5,
            (Mnemonic::DEC, AddressingMode::ZeroPageIndexedWithX(_)) => 0xd6,
            (Mnemonic::CLD, AddressingMode::Implied) => 0xd8,
            (Mnemonic::CMP, AddressingMode::AbsoluteIndexedWithY(_)) => 0xd9,
            (Mnemonic::CMP, AddressingMode::AbsoluteIndexedWithX(_)) => 0xdd,
            (Mnemonic::DEC, AddressingMode::AbsoluteIndexedWithX(_)) => 0xde,
            (Mnemonic::CPX, AddressingMode::Immediate(_)) => 0xe0,
            (Mnemonic::SBC, AddressingMode::XIndexedIndirect(_)) => 0xe1,
            (Mnemonic::CPX, AddressingMode::ZeroPage(_)) => 0xe4,
            (Mnemonic::SBC, AddressingMode::ZeroPage(_)) => 0xe5,
            (Mnemonic::INC, AddressingMode::ZeroPage(_)) => 0xe6,
            (Mnemonic::INX, AddressingMode::Implied) => 0xe8,
            (Mnemonic::SBC, AddressingMode::Immediate(_)) => 0xe9,
            (Mnemonic::NOP, AddressingMode::Implied) => 0xea,
            (Mnemonic::CPX, AddressingMode::Absolute(_)) => 0xec,
            (Mnemonic::SBC, AddressingMode::Absolute(_)) => 0xed,
            (Mnemonic::INC, AddressingMode::Absolute(_)) => 0xee,
            (Mnemonic::BEQ, AddressingMode::Relative(_)) => 0xf0,
            (Mnemonic::SBC, AddressingMode::IndirectYIndexed(_)) => 0xf1,
            (Mnemonic::SBC, AddressingMode::ZeroPageIndexedWithX(_)) => 0xf5,
            (Mnemonic::INC, AddressingMode::ZeroPageIndexedWithX(_)) => 0xf6,
            (Mnemonic::SED, AddressingMode::Implied) => 0xf8,
            (Mnemonic::SBC, AddressingMode::AbsoluteIndexedWithY(_)) => 0xf9,
            (Mnemonic::SBC, AddressingMode::AbsoluteIndexedWithX(_)) => 0xfd,
            (Mnemonic::INC, AddressingMode::AbsoluteIndexedWithX(_)) => 0xfe,
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
