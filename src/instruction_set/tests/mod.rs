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
