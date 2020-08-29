use crate::instruction_set::{AddressMode, Instruction, Mnemonic};
use crate::parser::instruction;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

// no guarantees about instruction validity are asserted in these tests.

#[test]
fn implied_address_mode_should_match_if_no_address_mode_supplied() {
    let input = "nop\n";

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            Instruction::new(Mnemonic::NOP, AddressMode::Implied)
        ))),
        instruction().parse(&input)
    );
}

#[test]
fn accumulator_address_mode_should_match_a() {
    let input = "nop A\n";

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            Instruction::new(Mnemonic::NOP, AddressMode::Accumulator)
        ))),
        instruction().parse(&input)
    );
}

#[test]
fn absolute_address_mode_should_match_valid_4_digit_hex_code() {
    let input = "nop $1a2b\n";

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            Instruction::new(Mnemonic::NOP, AddressMode::Absolute(0x1a2b))
        ))),
        instruction().parse(&input)
    );
}
