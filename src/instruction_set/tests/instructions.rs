use crate::instruction_set::address_mode::AddressMode;
use crate::instruction_set::mnemonics::Mnemonic;
use crate::instruction_set::StaticInstruction;

#[test]
fn instruction_with_accumulator_address_mode_should_return_single_byte() {
    let inst = StaticInstruction::new(Mnemonic::ASL, AddressMode::Accumulator);
    let op: Vec<u8> = inst.into();

    assert_eq!(op, vec![0x0a])
}

#[test]
fn instruction_with_single_byte_operand_should_order_instructions_correctly() {
    let inst = StaticInstruction::new(Mnemonic::CPY, AddressMode::Immediate(0x12));
    let op: Vec<u8> = inst.into();

    assert_eq!(op, vec![0xc0, 0x12])
}

#[test]
fn instruction_with_two_byte_operand_should_order_operands_after_opcode_in_little_endian_format() {
    let inst = StaticInstruction::new(Mnemonic::CPY, AddressMode::Absolute(0x1234));
    let op: Vec<u8> = inst.into();

    assert_eq!(op, vec![0xcc, 0x34, 0x12])
}
