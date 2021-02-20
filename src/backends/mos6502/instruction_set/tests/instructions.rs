use crate::Emitter;
use isa_mos6502::{addressing_mode::AddressingMode, mnemonic::Mnemonic, InstructionVariant};

#[test]
fn instruction_with_accumulator_address_mode_should_return_single_byte() {
    let inst = InstructionVariant::new(Mnemonic::ASL, AddressingMode::Accumulator).unwrap();
    let op_res: Result<Vec<u8>, _> = inst.emit();
    let op: Vec<u8> = op_res.unwrap();

    assert_eq!(op, vec![0x0a])
}

#[test]
fn instruction_with_single_byte_operand_should_order_instructions_correctly() {
    let inst = InstructionVariant::new(Mnemonic::CPY, AddressingMode::Immediate(0x12)).unwrap();
    let op_res: Result<Vec<u8>, _> = inst.emit();
    let op: Vec<u8> = op_res.unwrap();

    assert_eq!(op, vec![0xc0, 0x12])
}

#[test]
fn instruction_with_two_byte_operand_should_order_operands_after_opcode_in_little_endian_format() {
    let inst = InstructionVariant::new(Mnemonic::CPY, AddressingMode::Absolute(0x1234)).unwrap();
    let op_res: Result<Vec<u8>, _> = inst.emit();
    let op: Vec<u8> = op_res.unwrap();

    assert_eq!(op, vec![0xcc, 0x34, 0x12])
}

#[test]
fn unknown_instruction_should_thrown_an_error() {
    let inst = InstructionVariant::new(Mnemonic::JMP, AddressingMode::Accumulator);

    assert!(inst.is_err())
}
