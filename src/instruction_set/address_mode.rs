// AddressMode represents the 6502 addressing mode only.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AddressMode {
    Accumlator,
    Implied,
    Immediate,
    Absolute,
    ZeroPage,
    Relative,
    AbsoluteIndirect,
    AbsoluteIndexedWithX,
    AbsoluteIndexedWithY,
    ZeroPageIndexedWithX,
    ZeroPageIndexedWithY,
    ZeroPageIndexedIndirect,
    ZeroPageIndirectIndexedWithY,
}

impl From<AddressModeWithOperand> for AddressMode {
    fn from(am: AddressModeWithOperand) -> AddressMode {
        match am {
            AddressModeWithOperand::Accumlator => AddressMode::Accumlator,
            AddressModeWithOperand::Implied => AddressMode::Implied,
            AddressModeWithOperand::Immediate(_) => AddressMode::Immediate,
            AddressModeWithOperand::Absolute(_) => AddressMode::Absolute,
            AddressModeWithOperand::ZeroPage(_) => AddressMode::ZeroPage,
            AddressModeWithOperand::Relative(_) => AddressMode::Relative,
            AddressModeWithOperand::AbsoluteIndirect(_) => AddressMode::AbsoluteIndirect,
            AddressModeWithOperand::AbsoluteIndexedWithX(_) => AddressMode::AbsoluteIndexedWithX,
            AddressModeWithOperand::AbsoluteIndexedWithY(_) => AddressMode::AbsoluteIndexedWithY,
            AddressModeWithOperand::ZeroPageIndexedWithX(_) => AddressMode::ZeroPageIndexedWithX,
            AddressModeWithOperand::ZeroPageIndexedWithY(_) => AddressMode::ZeroPageIndexedWithY,
            AddressModeWithOperand::ZeroPageIndexedIndirect(_) => {
                AddressMode::ZeroPageIndexedIndirect
            }
            AddressModeWithOperand::ZeroPageIndirectIndexedWithY(_) => {
                AddressMode::ZeroPageIndirectIndexedWithY
            }
        }
    }
}

/// AddressModeWithOperand captures the Address mode type with a corresponding
/// operand of the appropriate bit length.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AddressModeWithOperand {
    Accumlator,
    Implied,
    Immediate(u8),
    Absolute(u16),
    ZeroPage(u8),
    Relative(u8),
    AbsoluteIndirect(u16),
    AbsoluteIndexedWithX(u16),
    AbsoluteIndexedWithY(u16),
    ZeroPageIndexedWithX(u8),
    ZeroPageIndexedWithY(u8),
    ZeroPageIndexedIndirect(u8),
    ZeroPageIndirectIndexedWithY(u8),
}

impl PartialEq<AddressMode> for AddressModeWithOperand {
    fn eq(&self, other: &AddressMode) -> bool {
        match self {
            AddressModeWithOperand::Accumlator => *other == AddressMode::Accumlator,
            AddressModeWithOperand::Implied => *other == AddressMode::Implied,
            AddressModeWithOperand::Immediate(_) => *other == AddressMode::Immediate,
            AddressModeWithOperand::Absolute(_) => *other == AddressMode::Absolute,
            AddressModeWithOperand::ZeroPage(_) => *other == AddressMode::ZeroPage,
            AddressModeWithOperand::Relative(_) => *other == AddressMode::Relative,
            AddressModeWithOperand::AbsoluteIndirect(_) => *other == AddressMode::AbsoluteIndirect,
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
            AddressModeWithOperand::ZeroPageIndexedIndirect(_) => {
                *other == AddressMode::ZeroPageIndexedIndirect
            }
            AddressModeWithOperand::ZeroPageIndirectIndexedWithY(_) => {
                *other == AddressMode::ZeroPageIndirectIndexedWithY
            }
        }
    }
}
