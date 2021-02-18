use crate::addressing;
use crate::Emitter;
use isa_mos6502::{addressing_mode::AddressingMode, ByteSized};
use std::fmt;

pub type Label = String;

#[derive(Clone, PartialEq, Debug)]
pub struct Symbol {
    pub address_mode_type: AddressingModeType,
    pub symbol: String,
}

impl Symbol {
    pub fn new(amt: AddressingModeType, symbol: String) -> Self {
        Self {
            address_mode_type: amt,
            symbol,
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", &self.address_mode_type, self.symbol)
    }
}

impl addressing::SizeOf for Symbol {
    fn size_of(&self) -> usize {
        1
    }
}

/// AddressingModeOrReference handles for parsing either an explicit address mode or a
/// label mapping.
#[derive(Clone, PartialEq, Debug)]
pub enum AddressingModeOrReference {
    AddressingMode(AddressingMode),
    Label(Label),
    Symbol(Symbol),
}

impl addressing::SizeOf for AddressingModeOrReference {
    fn size_of(&self) -> usize {
        match self {
            Self::AddressingMode(am) => am.size_of(),
            Self::Label(_) => 2,
            Self::Symbol(_) => 1,
        }
    }
}

/// AddressingModeType captures the Address mode type sans the value.
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Hash, Debug)]
pub enum AddressingModeType {
    Accumulator,
    Implied,
    Immediate,
    Absolute,
    ZeroPage,
    Relative,
    Indirect,
    AbsoluteIndexedWithX,
    AbsoluteIndexedWithY,
    ZeroPageIndexedWithX,
    ZeroPageIndexedWithY,
    XIndexedIndirect,
    IndirectYIndexed,
}

impl fmt::Display for AddressingModeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Emitter<Vec<u8>> for AddressingMode {
    fn emit(&self) -> Vec<u8> {
        Into::<Vec<u8>>::into(*self)
    }
}

impl addressing::SizeOf for AddressingMode {
    fn size_of(&self) -> usize {
        self.byte_size()
    }
}
