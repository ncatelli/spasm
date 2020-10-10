use crate::addressing;
use crate::Emitter;
use std::fmt;

pub type Label = String;

#[derive(Clone, PartialEq, Debug)]
pub struct Symbol {
    pub address_mode_type: AddressModeType,
    pub symbol: String,
}

impl Symbol {
    pub fn new(amt: AddressModeType, symbol: String) -> Self {
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

/// AddressModeOrReference handles for parsing either an explicit address mode or a
/// label mapping.
#[derive(Clone, PartialEq, Debug)]
pub enum AddressModeOrReference {
    AddressMode(AddressMode),
    Label(Label),
    Symbol(Symbol),
}

impl addressing::SizeOf for AddressModeOrReference {
    fn size_of(&self) -> usize {
        match self {
            Self::AddressMode(am) => am.size_of(),
            Self::Label(_) => 2,
            Self::Symbol(_) => 1,
        }
    }
}

/// AddressModeType captures the Address mode type sans the value.
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Hash, Debug)]
pub enum AddressModeType {
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
    IndexedIndirect,
    IndirectIndexed,
}

impl fmt::Display for AddressModeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*
        match self {
            Accumulator => write!(f, ""),
            Implied => write!(f, ""),
            Immediate => write!(f, "#"),
            Absolute => write!(f, "",),
            ZeroPage => write!(f, "{}", ""),
            Relative => write!(f, "{}", ""),
            Indirect => write!(f, "{}", ""),
            AbsoluteIndexedWithX => write!(f, "{}", ""),
            AbsoluteIndexedWithY => write!(f, "{}", ""),
            ZeroPageIndexedWithX => write!(f, "{}", ""),
            ZeroPageIndexedWithY => write!(f, "{}", ""),
            IndexedIndirect => write!(f, "{}", ""),
            IndirectIndexed => write!(f, "\{\}", ""),
        }*/
        write!(f, "{:?}", self)
    }
}

/// AddressMode captures the Address mode type with a corresponding
/// operand of the appropriate bit length.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AddressMode {
    Accumulator,
    Implied,
    Immediate(u8),
    Absolute(u16),
    ZeroPage(u8),
    Relative(i8),
    Indirect(u16),
    AbsoluteIndexedWithX(u16),
    AbsoluteIndexedWithY(u16),
    ZeroPageIndexedWithX(u8),
    ZeroPageIndexedWithY(u8),
    IndexedIndirect(u8),
    IndirectIndexed(u8),
}

impl Into<Vec<u8>> for AddressMode {
    fn into(self) -> Vec<u8> {
        match self {
            AddressMode::Immediate(operand) => vec![operand],
            AddressMode::Absolute(operand) => operand.to_le_bytes().to_vec(),
            AddressMode::ZeroPage(operand) => vec![operand],
            AddressMode::Relative(operand) => {
                let o_to_u8 = unsafe { std::mem::transmute::<i8, u8>(operand) };
                vec![o_to_u8]
            }
            AddressMode::Indirect(operand) => operand.to_le_bytes().to_vec(),
            AddressMode::AbsoluteIndexedWithX(operand) => operand.to_le_bytes().to_vec(),
            AddressMode::AbsoluteIndexedWithY(operand) => operand.to_le_bytes().to_vec(),
            AddressMode::ZeroPageIndexedWithX(operand) => vec![operand],
            AddressMode::ZeroPageIndexedWithY(operand) => vec![operand],
            AddressMode::IndexedIndirect(operand) => vec![operand],
            AddressMode::IndirectIndexed(operand) => vec![operand],
            _ => vec![],
        }
    }
}

impl Emitter<Vec<u8>> for AddressMode {
    fn emit(&self) -> Vec<u8> {
        Into::<Vec<u8>>::into(*self)
    }
}

impl addressing::SizeOf for AddressMode {
    fn size_of(&self) -> usize {
        match self {
            AddressMode::Accumulator | AddressMode::Implied => 0,
            AddressMode::Absolute(_)
            | AddressMode::Indirect(_)
            | AddressMode::AbsoluteIndexedWithX(_)
            | AddressMode::AbsoluteIndexedWithY(_) => 2,
            _ => 1,
        }
    }
}
