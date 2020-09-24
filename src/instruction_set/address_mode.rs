use crate::addressing;

pub type Label = String;
pub type Symbol = String;

/// AddressModeOrReference handles for parsing either an explicit address mode or a
/// label mapping.
#[derive(Clone, PartialEq, Debug)]
pub enum AddressModeOrReference {
    AddressMode(AddressMode),
    Label(Label),
    Symbol(Symbol),
}

impl addressing::SizeOf for AddressModeOrReference {
    fn size_of(&self) -> u16 {
        match self {
            Self::AddressMode(am) => am.size_of(),
            Self::Label(_) => 2,
            Self::Symbol(_) => 1,
        }
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
            //            AddressMode::Relative(operand) => vec![operand], // need to implement for i8
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

impl addressing::SizeOf for AddressMode {
    fn size_of(&self) -> u16 {
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
