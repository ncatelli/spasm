use crate::instruction_set::address_mode::AddressMode;

#[test]
fn address_mode_with_operand_should_be_comparable_to_address_mode() {
    let amwo: AddressMode = AddressMode::Accumulator;

    assert!(amwo == AddressMode::Accumulator);
}

#[test]
fn validate_address_mode_with_operands_into_bytes_returns_correct_endian_bytes() {
    let addresses = vec![
        AddressMode::Absolute(0x8008),
        AddressMode::AbsoluteIndexedWithX(0x8008),
        AddressMode::AbsoluteIndexedWithY(0x8008),
        AddressMode::Accumulator,
        AddressMode::Immediate(0x80),
        AddressMode::Implied,
        AddressMode::IndexedIndirect(0x80),
        AddressMode::Indirect(0x8008),
        AddressMode::IndirectIndexed(0x80),
        AddressMode::Relative(0x80),
        AddressMode::ZeroPage(0x80),
        AddressMode::ZeroPageIndexedWithX(0x80),
        AddressMode::ZeroPageIndexedWithY(0x80),
    ];
    let operands = vec![
        vec![0x08, 0x80],
        vec![0x08, 0x80],
        vec![0x08, 0x80],
        vec![],
        vec![0x80],
        vec![],
        vec![0x80],
        vec![0x08, 0x80],
        vec![0x80],
        vec![0x80],
        vec![0x80],
        vec![0x80],
        vec![0x80],
    ];

    for (am, rs) in addresses.into_iter().zip(operands.into_iter()) {
        let am_bytes: Vec<u8> = am.into();
        assert_eq!(am_bytes, rs)
    }
}
