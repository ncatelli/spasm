use isa_mos6502::{addressing_mode::AddressingMode, ByteSized};

#[test]
fn address_mode_with_operand_should_be_comparable_to_address_mode() {
    let amwo: AddressingMode = AddressingMode::Accumulator;

    assert!(amwo == AddressingMode::Accumulator);
}

#[test]
fn validate_address_mode_with_operands_into_bytes_returns_correct_endian_bytes() {
    let addresses = vec![
        AddressingMode::Absolute(0x8008),
        AddressingMode::AbsoluteIndexedWithX(0x8008),
        AddressingMode::AbsoluteIndexedWithY(0x8008),
        AddressingMode::Accumulator,
        AddressingMode::Immediate(0x80),
        AddressingMode::Implied,
        AddressingMode::XIndexedIndirect(0x80),
        AddressingMode::Indirect(0x8008),
        AddressingMode::IndirectYIndexed(0x80),
        AddressingMode::Relative(0x40),
        AddressingMode::ZeroPage(0x80),
        AddressingMode::ZeroPageIndexedWithX(0x80),
        AddressingMode::ZeroPageIndexedWithY(0x80),
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
        vec![0x40],
        vec![0x80],
        vec![0x80],
        vec![0x80],
    ];

    for (am, rs) in addresses.into_iter().zip(operands.into_iter()) {
        let am_bytes: Vec<u8> = am.into();
        assert_eq!(am_bytes, rs)
    }
}

#[test]
fn validate_address_mode_maps_to_the_correct_size_of() {
    let address_sizing: Vec<usize> = vec![
        AddressingMode::Absolute(0x8008),
        AddressingMode::AbsoluteIndexedWithX(0x8008),
        AddressingMode::AbsoluteIndexedWithY(0x8008),
        AddressingMode::Accumulator,
        AddressingMode::Immediate(0x80),
        AddressingMode::Implied,
        AddressingMode::XIndexedIndirect(0x80),
        AddressingMode::Indirect(0x8008),
        AddressingMode::IndirectYIndexed(0x80),
        //AddressingMode::Relative(0x80),
        AddressingMode::ZeroPage(0x80),
        AddressingMode::ZeroPageIndexedWithX(0x80),
        AddressingMode::ZeroPageIndexedWithY(0x80),
    ]
    .into_iter()
    .map(|am| am.byte_size())
    .collect();

    assert_eq!(vec![2, 2, 2, 0, 1, 0, 1, 2, 1, 1, 1, 1], address_sizing)
}
