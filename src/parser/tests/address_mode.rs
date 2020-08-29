use crate::instruction_set::{AddressMode, Instruction, Mnemonic};
use crate::parser::instruction;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

macro_rules! gen_am_test {
    ($input:literal, $inst:expr, $am:expr) => {
        assert_eq!(
            Ok(MatchStatus::Match((
                &$input[$input.len()..],
                Instruction::new($inst, $am)
            ))),
            instruction().parse(&$input)
        );
    };
}

#[test]
fn implied_address_mode_should_match_if_no_address_mode_supplied() {
    gen_am_test!("nop\n", Mnemonic::NOP, AddressMode::Implied)
}

#[test]
fn accumulator_address_mode_should_match_a() {
    gen_am_test!("nop A\n", Mnemonic::NOP, AddressMode::Accumulator)
}

#[test]
fn absolute_address_mode_should_match_valid_4_digit_hex_code() {
    gen_am_test!("nop $1a2b\n", Mnemonic::NOP, AddressMode::Absolute(0x1a2b))
}

#[test]
fn absolute_x_indexed_address_mode_should_match_valid_4_digit_hex_code() {
    gen_am_test!(
        "nop $1a2b,X\n",
        Mnemonic::NOP,
        AddressMode::AbsoluteIndexedWithX(0x1a2b)
    )
}

#[test]
fn absolute_y_indexed_address_mode_should_match_valid_4_digit_hex_code() {
    gen_am_test!(
        "nop $1a2b,Y\n",
        Mnemonic::NOP,
        AddressMode::AbsoluteIndexedWithY(0x1a2b)
    )
}

#[test]
fn immediate_address_mode_should_match_valid_2_digit_hex_code() {
    gen_am_test!("nop #$1a\n", Mnemonic::NOP, AddressMode::Immediate(0x1a))
}

#[test]
fn indirect_address_mode_should_match_valid_2_digit_hex_code() {
    gen_am_test!(
        "nop ($1a2b)\n",
        Mnemonic::NOP,
        AddressMode::Indirect(0x1a2b)
    )
}
