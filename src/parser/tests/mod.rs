use crate::instruction_set::{AddressMode, Instruction, Mnemonic};
use crate::parser::instructions;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

mod address_mode;
mod instructions;

#[test]
fn should_parse_multiple_instructions() {
    let input = "nop
lda #12
sta $1234
jmp $1234\n";

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![
                Instruction::new(Mnemonic::NOP, AddressMode::Implied),
                Instruction::new(Mnemonic::LDA, AddressMode::Immediate(0x12)),
                Instruction::new(Mnemonic::STA, AddressMode::Absolute(0x1234)),
                Instruction::new(Mnemonic::JMP, AddressMode::Absolute(0x1234))
            ]
        ))),
        instructions().parse(&input)
    );
}
