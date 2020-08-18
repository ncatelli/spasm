// AddressMode represents the 6502 addressing mode only.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AddressMode {
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

impl From<AddressModeWithOperand> for AddressMode {
    fn from(am: AddressModeWithOperand) -> AddressMode {
        match am {
            AddressModeWithOperand::Accumulator => AddressMode::Accumulator,
            AddressModeWithOperand::Implied => AddressMode::Implied,
            AddressModeWithOperand::Immediate(_) => AddressMode::Immediate,
            AddressModeWithOperand::Absolute(_) => AddressMode::Absolute,
            AddressModeWithOperand::ZeroPage(_) => AddressMode::ZeroPage,
            AddressModeWithOperand::Relative(_) => AddressMode::Relative,
            AddressModeWithOperand::Indirect(_) => AddressMode::Indirect,
            AddressModeWithOperand::AbsoluteIndexedWithX(_) => AddressMode::AbsoluteIndexedWithX,
            AddressModeWithOperand::AbsoluteIndexedWithY(_) => AddressMode::AbsoluteIndexedWithY,
            AddressModeWithOperand::ZeroPageIndexedWithX(_) => AddressMode::ZeroPageIndexedWithX,
            AddressModeWithOperand::ZeroPageIndexedWithY(_) => AddressMode::ZeroPageIndexedWithY,
            AddressModeWithOperand::IndexedIndirect(_) => AddressMode::IndexedIndirect,
            AddressModeWithOperand::IndirectIndexed(_) => AddressMode::IndirectIndexed,
        }
    }
}

/// AddressModeWithOperand captures the Address mode type with a corresponding
/// operand of the appropriate bit length.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AddressModeWithOperand {
    Accumulator,
    Implied,
    Immediate(u8),
    Absolute(u16),
    ZeroPage(u8),
    Relative(u8),
    Indirect(u16),
    AbsoluteIndexedWithX(u16),
    AbsoluteIndexedWithY(u16),
    ZeroPageIndexedWithX(u8),
    ZeroPageIndexedWithY(u8),
    IndexedIndirect(u8),
    IndirectIndexed(u8),
}

impl PartialEq<AddressMode> for AddressModeWithOperand {
    fn eq(&self, other: &AddressMode) -> bool {
        match self {
            AddressModeWithOperand::Accumulator => *other == AddressMode::Accumulator,
            AddressModeWithOperand::Implied => *other == AddressMode::Implied,
            AddressModeWithOperand::Immediate(_) => *other == AddressMode::Immediate,
            AddressModeWithOperand::Absolute(_) => *other == AddressMode::Absolute,
            AddressModeWithOperand::ZeroPage(_) => *other == AddressMode::ZeroPage,
            AddressModeWithOperand::Relative(_) => *other == AddressMode::Relative,
            AddressModeWithOperand::Indirect(_) => *other == AddressMode::Indirect,
            AddressModeWithOperand::AbsoluteIndexedWithX(_) => {
                *other == AddressMode::AbsoluteIndexedWithX
            }
            AddressModeWithOperand::AbsoluteIndexedWithY(_) => {
                *other == AddressMode::AbsoluteIndexedWithY
            }
            AddressModeWithOperand::ZeroPageIndexedWithX(_) => {
                *other == AddressMode::ZeroPageIndexedWithX
            }
            AddressModeWithOperand::ZeroPageIndexedWithY(_) => {
                *other == AddressMode::ZeroPageIndexedWithY
            }
            AddressModeWithOperand::IndexedIndirect(_) => *other == AddressMode::IndexedIndirect,
            AddressModeWithOperand::IndirectIndexed(_) => *other == AddressMode::IndirectIndexed,
        }
    }
}

impl Into<[u8; 2]> for AddressModeWithOperand {
    fn into(self) -> [u8; 2] {
        match self {
            AddressModeWithOperand::Accumulator => [0, 0],
            AddressModeWithOperand::Implied => [0, 0],
            AddressModeWithOperand::Immediate(operand) => [operand, 0],
            AddressModeWithOperand::Absolute(operand) => operand.to_le_bytes(),
            AddressModeWithOperand::ZeroPage(operand) => [operand, 0],
            AddressModeWithOperand::Relative(operand) => [operand, 0],
            AddressModeWithOperand::Indirect(operand) => operand.to_le_bytes(),
            AddressModeWithOperand::AbsoluteIndexedWithX(operand) => operand.to_le_bytes(),
            AddressModeWithOperand::AbsoluteIndexedWithY(operand) => operand.to_le_bytes(),
            AddressModeWithOperand::ZeroPageIndexedWithX(operand) => [operand, 0],
            AddressModeWithOperand::ZeroPageIndexedWithY(operand) => [operand, 0],
            AddressModeWithOperand::IndexedIndirect(operand) => [operand, 0],
            AddressModeWithOperand::IndirectIndexed(operand) => [operand, 0],
        }
    }
}
