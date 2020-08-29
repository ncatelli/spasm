use crate::instruction_set::{AddressMode, Instruction, Mnemonic};
use crate::parser::program;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

mod address_mode;
mod instructions;

#[ignore]
#[test]
fn should_multiline_program() {
    let input = "nop
sta $1234
jmp $1234\n";

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![
                Instruction::new(Mnemonic::NOP, AddressMode::Implied),
                Instruction::new(Mnemonic::STA, AddressMode::Absolute(0x1234)),
                Instruction::new(Mnemonic::JMP, AddressMode::Absolute(0x1234))
            ]
        ))),
        program().parse(&input)
    );
}
