use crate::instruction_set::{AddressMode, Mnemonic};
use crate::parser::instruction;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

macro_rules! gen_inst_test {
    ($input:expr, $mnemonic:expr, $am:expr) => {
        assert_eq!(
            Ok(MatchStatus::Match((
                &$input[$input.len()..],
                $crate::instruction_set::InstructionOrDefinition::Instruction(
                    $crate::instruction_set::Instruction::from(
                        $crate::instruction_set::StaticInstruction::new($mnemonic, $am)
                    )
                )
            ))),
            instruction().parse($input)
        );
    };
}

macro_rules! chars {
    ($input:expr) => {
        $input.chars().collect::<Vec<char>>()
    };
}

#[test]
fn should_parse_valid_nop_instruction() {
    let input = chars!("nop");
    gen_inst_test!(&input, Mnemonic::NOP, AddressMode::Implied);
}

#[test]
fn should_strip_arbitrary_length_leading_chars_from_instruction() {
    let input = chars!("    nop");
    gen_inst_test!(&input, Mnemonic::NOP, AddressMode::Implied);
}

#[test]
fn should_parse_and_ignore_inline_comments() {
    let input = chars!("    nop ; this is a comment");
    gen_inst_test!(&input, Mnemonic::NOP, AddressMode::Implied);
}
