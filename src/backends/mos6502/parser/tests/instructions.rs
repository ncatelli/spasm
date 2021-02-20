use crate::backends::mos6502::parser::instruction;
use isa_mos6502::{addressing_mode::AddressingMode, mnemonic::Mnemonic};
use parcel::prelude::v1::*;
use parcel::MatchStatus;

macro_rules! gen_inst_test {
    ($input:expr, $mnemonic:expr, $am:expr) => {
        assert_eq!(
            Ok(MatchStatus::Match((
                &$input[$input.len()..],
                $crate::backends::mos6502::instruction_set::Instruction::from(($mnemonic, $am))
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
    gen_inst_test!(&input, Mnemonic::NOP, AddressingMode::Implied);
}

#[test]
fn should_strip_arbitrary_length_leading_chars_from_instruction() {
    let input = chars!("    nop");
    gen_inst_test!(&input, Mnemonic::NOP, AddressingMode::Implied);
}
