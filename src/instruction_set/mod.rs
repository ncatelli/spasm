#[cfg(test)]
mod tests;

pub mod address_mode;
pub mod mnemonics;
pub mod op_codes;

/// Instruction represents a single 6502 instruction containing a mnemonic,
/// address mode and optionally any operands.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Instruction {
    mnemonic: mnemonics::Mnemonic,
    address_mode: address_mode::AddressModeWithOperand,
}
