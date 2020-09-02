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
