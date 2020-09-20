use crate::instruction_set::{AddressMode, Instruction, Mnemonic};
use crate::parser::instruction;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

macro_rules! gen_am_test {
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
fn implied_address_mode_should_match_if_no_address_mode_supplied() {
    gen_am_test!("nop", Mnemonic::NOP, AddressMode::Implied)
}

#[test]
fn accumulator_address_mode_should_match_a() {
    gen_am_test!("asl A", Mnemonic::ASL, AddressMode::Accumulator)
}

#[test]
fn absolute_address_mode_should_match_valid_u16() {
    gen_am_test!("lda $1a2b", Mnemonic::LDA, AddressMode::Absolute(0x1a2b));
    gen_am_test!("lda 6699", Mnemonic::LDA, AddressMode::Absolute(0x1a2b));
}

#[test]
fn absolute_x_indexed_address_mode_should_match_valid_u16() {
    gen_am_test!(
        "adc $1a2b,X",
        Mnemonic::ADC,
        AddressMode::AbsoluteIndexedWithX(0x1a2b)
    );
    gen_am_test!(
        "adc 6699,X",
        Mnemonic::ADC,
        AddressMode::AbsoluteIndexedWithX(0x1a2b)
    );
}

#[test]
fn absolute_y_indexed_address_mode_should_match_valid_u16() {
    gen_am_test!(
        "inc $1a2b,Y",
        Mnemonic::INC,
        AddressMode::AbsoluteIndexedWithY(0x1a2b)
    );
    gen_am_test!(
        "inc 6699,Y",
        Mnemonic::INC,
        AddressMode::AbsoluteIndexedWithY(0x1a2b)
    );
}

#[test]
fn immediate_address_mode_should_match_valid_u8() {
    gen_am_test!("lda #$1a", Mnemonic::LDA, AddressMode::Immediate(0x1a));
    gen_am_test!("lda #26", Mnemonic::LDA, AddressMode::Immediate(0x1a));
}

#[test]
fn indirect_address_mode_should_match_valid_u16() {
    gen_am_test!("jmp ($1a2b)", Mnemonic::JMP, AddressMode::Indirect(0x1a2b));
    gen_am_test!("jmp (6699)", Mnemonic::JMP, AddressMode::Indirect(0x1a2b));
}

#[test]
fn indexed_indirect_address_mode_should_match_valid_u8() {
    gen_am_test!(
        "sta ($1a,X)",
        Mnemonic::STA,
        AddressMode::IndexedIndirect(0x1a)
    );
    gen_am_test!(
        "sta (26,X)",
        Mnemonic::STA,
        AddressMode::IndexedIndirect(0x1a)
    );
}

#[test]
fn indirect_indexed_address_mode_should_match_valid_u8() {
    gen_am_test!(
        "eor ($1a),Y",
        Mnemonic::EOR,
        AddressMode::IndirectIndexed(0x1a)
    );
    gen_am_test!(
        "eor (26),Y",
        Mnemonic::EOR,
        AddressMode::IndirectIndexed(0x1a)
    );
}

#[ignore]
#[test]
fn relative_address_mode_should_match_valid_u8() {
    gen_am_test!("bpl $1a", Mnemonic::BPL, AddressMode::Relative(0x1a));
    gen_am_test!("bpl 26", Mnemonic::BPL, AddressMode::Relative(0x1a));
}

#[test]
fn zeropage_address_mode_should_match_valid_u8() {
    gen_am_test!("ldy $1a", Mnemonic::LDY, AddressMode::ZeroPage(0x1a));
    gen_am_test!("ldy 26", Mnemonic::LDY, AddressMode::ZeroPage(0x1a));
}

#[test]
fn zeropage_x_indexed_address_mode_should_match_valid_u8() {
    gen_am_test!(
        "lda $1a,X",
        Mnemonic::LDA,
        AddressMode::ZeroPageIndexedWithX(0x1a)
    );
    gen_am_test!(
        "lda 26,X",
        Mnemonic::LDA,
        AddressMode::ZeroPageIndexedWithX(0x1a)
    );
}

#[test]
fn zeropage_y_indexed_address_mode_should_match_valid_u8() {
    gen_am_test!(
        "lda $1a,Y",
        Mnemonic::LDA,
        AddressMode::ZeroPageIndexedWithY(0x1a)
    );
    gen_am_test!(
        "lda 26,Y",
        Mnemonic::LDA,
        AddressMode::ZeroPageIndexedWithY(0x1a)
    );
}
