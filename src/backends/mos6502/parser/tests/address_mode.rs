use crate::backends::mos6502::parser::instruction;
use isa_mos6502::{addressing_mode::AddressingMode, mnemonic::Mnemonic};
use parcel::prelude::v1::*;
use parcel::MatchStatus;

macro_rules! gen_am_test {
    ($input:expr, $mnemonic:expr, $am:expr) => {
        assert_eq!(
            Ok(MatchStatus::Match((
                &$input[$input.len()..],
                $crate::backends::mos6502::instruction_set::Instruction::from(
                    $crate::backends::mos6502::instruction_set::StaticInstruction::new(
                        $mnemonic, $am
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
fn implied_address_mode_should_match_if_no_address_mode_supplied() {
    let input = chars!("nop");
    gen_am_test!(&input, Mnemonic::NOP, AddressingMode::Implied);
}

#[test]
fn accumulator_address_mode_should_match_a() {
    let input = chars!("asl A");

    gen_am_test!(&input, Mnemonic::ASL, AddressingMode::Accumulator);
}

#[test]
fn absolute_address_mode_should_match_valid_u16() {
    let hinput = chars!("lda 0x1a2b");
    let binput = chars!("lda 0b0001101000101011");
    let dinput = chars!("lda 6699");

    gen_am_test!(&hinput, Mnemonic::LDA, AddressingMode::Absolute(0x1a2b));
    gen_am_test!(&binput, Mnemonic::LDA, AddressingMode::Absolute(0x1a2b));
    gen_am_test!(&dinput, Mnemonic::LDA, AddressingMode::Absolute(0x1a2b));
}

#[test]
fn absolute_x_indexed_address_mode_should_match_valid_u16() {
    let hinput = chars!("adc 0x1a2b,X");
    let binput = chars!("adc 0b0001101000101011,X");
    let dinput = chars!("adc 6699,X");

    gen_am_test!(
        &hinput,
        Mnemonic::ADC,
        AddressingMode::AbsoluteIndexedWithX(0x1a2b)
    );
    gen_am_test!(
        &binput,
        Mnemonic::ADC,
        AddressingMode::AbsoluteIndexedWithX(0x1a2b)
    );
    gen_am_test!(
        &dinput,
        Mnemonic::ADC,
        AddressingMode::AbsoluteIndexedWithX(0x1a2b)
    );
}

#[test]
fn absolute_y_indexed_address_mode_should_match_valid_u16() {
    let hinput = chars!("inc 0x1a2b,Y");
    let binput = chars!("inc 0b0001101000101011,Y");
    let dinput = chars!("inc 6699,Y");

    gen_am_test!(
        &hinput,
        Mnemonic::INC,
        AddressingMode::AbsoluteIndexedWithY(0x1a2b)
    );
    gen_am_test!(
        &binput,
        Mnemonic::INC,
        AddressingMode::AbsoluteIndexedWithY(0x1a2b)
    );
    gen_am_test!(
        &dinput,
        Mnemonic::INC,
        AddressingMode::AbsoluteIndexedWithY(0x1a2b)
    );
}

#[test]
fn immediate_address_mode_should_match_valid_u8() {
    let hinput = chars!("lda #0x1a");
    let binput = chars!("lda #0b00011010");
    let dinput = chars!("lda #26");

    gen_am_test!(&hinput, Mnemonic::LDA, AddressingMode::Immediate(0x1a));
    gen_am_test!(&binput, Mnemonic::LDA, AddressingMode::Immediate(0x1a));
    gen_am_test!(&dinput, Mnemonic::LDA, AddressingMode::Immediate(0x1a));
}

#[test]
fn indirect_address_mode_should_match_valid_u16() {
    let hinput = chars!("jmp (0x1a2b)");
    let binput = chars!("jmp (0b0001101000101011)");
    let dinput = chars!("jmp (6699)");

    gen_am_test!(&hinput, Mnemonic::JMP, AddressingMode::Indirect(0x1a2b));
    gen_am_test!(&binput, Mnemonic::JMP, AddressingMode::Indirect(0x1a2b));
    gen_am_test!(&dinput, Mnemonic::JMP, AddressingMode::Indirect(0x1a2b));
}

#[test]
fn indexed_indirect_address_mode_should_match_valid_u8() {
    let hinput = chars!("sta (0x1a,X)");
    let binput = chars!("sta (0b00011010,X)");
    let dinput = chars!("sta (26,X)");

    gen_am_test!(
        &hinput,
        Mnemonic::STA,
        AddressingMode::XIndexedIndirect(0x1a)
    );
    gen_am_test!(
        &binput,
        Mnemonic::STA,
        AddressingMode::XIndexedIndirect(0x1a)
    );
    gen_am_test!(
        &dinput,
        Mnemonic::STA,
        AddressingMode::XIndexedIndirect(0x1a)
    );
}

#[test]
fn indirect_indexed_address_mode_should_match_valid_u8() {
    let hinput = chars!("eor (0x1a),Y");
    let binput = chars!("eor (0b00011010),Y");
    let dinput = chars!("eor (26),Y");

    gen_am_test!(
        &hinput,
        Mnemonic::EOR,
        AddressingMode::IndirectYIndexed(0x1a)
    );
    gen_am_test!(
        &dinput,
        Mnemonic::EOR,
        AddressingMode::IndirectYIndexed(0x1a)
    );
    gen_am_test!(
        &binput,
        Mnemonic::EOR,
        AddressingMode::IndirectYIndexed(0x1a)
    );
}

#[test]
fn relative_address_mode_should_match_valid_u8() {
    let hinput = chars!("bpl *0x1a");
    let binput = chars!("bpl *0b00011010");
    let dinput = chars!("bpl *26");
    let dspinput = chars!("bpl *+26");
    let dsninput = chars!("bpl *-26");

    gen_am_test!(&hinput, Mnemonic::BPL, AddressingMode::Relative(0x1a));
    gen_am_test!(&dinput, Mnemonic::BPL, AddressingMode::Relative(0x1a));
    gen_am_test!(&dspinput, Mnemonic::BPL, AddressingMode::Relative(0x1a));
    gen_am_test!(&dsninput, Mnemonic::BPL, AddressingMode::Relative(-26));
    gen_am_test!(&binput, Mnemonic::BPL, AddressingMode::Relative(0x1a));
}

#[test]
fn zeropage_address_mode_should_match_valid_u8() {
    let hinput = chars!("ldy 0x1a");
    let binput = chars!("ldy 0b00011010");
    let dinput = chars!("ldy 26");

    gen_am_test!(&hinput, Mnemonic::LDY, AddressingMode::ZeroPage(0x1a));
    gen_am_test!(&dinput, Mnemonic::LDY, AddressingMode::ZeroPage(0x1a));
    gen_am_test!(&binput, Mnemonic::LDY, AddressingMode::ZeroPage(0x1a));
}

#[test]
fn zeropage_x_indexed_address_mode_should_match_valid_u8() {
    let hinput = chars!("lda 0x1a,X");
    let binput = chars!("lda 0b00011010,X");
    let dinput = chars!("lda 26,X");

    gen_am_test!(
        &hinput,
        Mnemonic::LDA,
        AddressingMode::ZeroPageIndexedWithX(0x1a)
    );
    gen_am_test!(
        &dinput,
        Mnemonic::LDA,
        AddressingMode::ZeroPageIndexedWithX(0x1a)
    );
    gen_am_test!(
        &binput,
        Mnemonic::LDA,
        AddressingMode::ZeroPageIndexedWithX(0x1a)
    );
}

#[test]
fn zeropage_y_indexed_address_mode_should_match_valid_u8() {
    let hinput = chars!("lda 0x1a,Y");
    let dinput = chars!("lda 26,Y");
    let binput = chars!("lda 0b00011010,Y");

    gen_am_test!(
        &hinput,
        Mnemonic::LDA,
        AddressingMode::ZeroPageIndexedWithY(0x1a)
    );
    gen_am_test!(
        &dinput,
        Mnemonic::LDA,
        AddressingMode::ZeroPageIndexedWithY(0x1a)
    );
    gen_am_test!(
        &binput,
        Mnemonic::LDA,
        AddressingMode::ZeroPageIndexedWithY(0x1a)
    );
}
