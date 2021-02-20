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

impl From<(Mnemonic, AddressingMode)> for Instruction {
    fn from((m, am): (Mnemonic, AddressingMode)) -> Self {
        Self {
            mnemonic: m,
            amor: AddressingModeOrReference::AddressingMode(am),
        }
    }
}

/// UnknownInstructionErr represents an Instruction that is unrepresentable or unknown.
#[derive(Debug, Copy, Clone)]
pub struct UnknownInstructionErr {
    mnemonic: isa_mos6502::mnemonic::Mnemonic,
    addressing_mode: isa_mos6502::addressing_mode::AddressingModeType,
}

impl UnknownInstructionErr {
    pub fn new(
        mnemonic: isa_mos6502::mnemonic::Mnemonic,
        addressing_mode: isa_mos6502::addressing_mode::AddressingModeType,
    ) -> Self {
        Self {
            mnemonic,
            addressing_mode,
        }
    }
}

impl fmt::Display for UnknownInstructionErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unknown instruction: {:?} {:?}",
            &self.mnemonic, &self.addressing_mode
        )
    }
}

impl Emitter<Result<Vec<OpCode>, UnknownInstructionErr>> for isa_mos6502::InstructionVariant {
    fn emit(&self) -> Result<Vec<OpCode>, UnknownInstructionErr> {
        Ok(Vec::<OpCode>::from(*self))
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
