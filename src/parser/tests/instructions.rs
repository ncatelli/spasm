use crate::instruction_set::{AddressMode, Instruction, Mnemonic};
use crate::parser::instruction;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

macro_rules! gen_inst_test {
    ($input:literal, $mnemonic:expr, $am:expr) => {
        assert_eq!(
            Ok(MatchStatus::Match((
                &$input[$input.len()..],
                Instruction::new($mnemonic, $am)
            ))),
            instruction().parse(&$input)
        );
    };
}

#[test]
fn should_parse_valid_nop_instruction() {
    gen_inst_test!("nop", Mnemonic::NOP, AddressMode::Implied)
}

#[test]
fn should_strip_arbitrary_length_leading_chars_from_instruction() {
    gen_inst_test!("    nop", Mnemonic::NOP, AddressMode::Implied)
}

#[test]
fn should_parse_and_ignore_inline_comments() {
    gen_inst_test!(
        "    nop ; this is a comment",
        Mnemonic::NOP,
        AddressMode::Implied
    )
}
