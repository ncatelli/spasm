use crate::instruction_set::address_mode::{AddressMode, AddressModeWithOperand};

#[test]
fn address_mode_with_operand_should_cast_into_corresponding_address_mode_type() {
    let amwo: AddressModeWithOperand = AddressModeWithOperand::Accumulator;
    let am: AddressMode = amwo.into();

    assert!(am == AddressMode::Accumulator);
}

#[test]
fn address_mode_with_operand_should_be_comparable_to_address_mode() {
    let amwo: AddressModeWithOperand = AddressModeWithOperand::Accumulator;

    assert!(amwo == AddressMode::Accumulator);
}

#[test]
fn validate_address_mode_with_operands_into_bytes_returns_correct_endian_bytes() {
    let addresses = vec![
        AddressModeWithOperand::Absolute(0x8008),
        AddressModeWithOperand::AbsoluteIndexedWithX(0x8008),
        AddressModeWithOperand::AbsoluteIndexedWithY(0x8008),
        AddressModeWithOperand::Accumulator,
        AddressModeWithOperand::Immediate(0x80),
        AddressModeWithOperand::Implied,
        AddressModeWithOperand::IndexedIndirect(0x80),
        AddressModeWithOperand::Indirect(0x8008),
        AddressModeWithOperand::IndirectIndexed(0x80),
        AddressModeWithOperand::Relative(0x80),
        AddressModeWithOperand::ZeroPage(0x80),
        AddressModeWithOperand::ZeroPageIndexedWithX(0x80),
        AddressModeWithOperand::ZeroPageIndexedWithY(0x80),
    ];
    let operands = vec![
        [0x08, 0x80],
        [0x08, 0x80],
        [0x08, 0x80],
        [0x00, 0x00],
        [0x80, 0x00],
        [0x00, 0x00],
        [0x80, 0x00],
        [0x08, 0x80],
        [0x80, 0x00],
        [0x80, 0x00],
        [0x80, 0x00],
        [0x80, 0x00],
        [0x80, 0x00],
    ];

    for (am, rs) in addresses.into_iter().zip(operands.into_iter()) {
        let am_bytes: [u8; 2] = am.into();
        assert_eq!(am_bytes, rs)
    }
}
