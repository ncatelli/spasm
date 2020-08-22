use crate::instruction_set::{AddressMode, Instruction, Mnemonic};
use crate::parser::instruction;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

#[test]
fn should_parse_valid_nop_instruction() {
    let input: Vec<char> = "nop\n".chars().collect();

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[4..],
            Instruction::new(Mnemonic::NOP, AddressMode::Implied)
        ))),
        instruction().parse(&input)
    );
}

#[test]
fn should_strip_arbitrary_length_leading_chars_from_instruction() {
    let input: Vec<char> = "    nop\n".chars().collect();

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[8..],
            Instruction::new(Mnemonic::NOP, AddressMode::Implied)
        ))),
        instruction().parse(&input)
    );
}
