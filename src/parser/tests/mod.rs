use crate::instruction_set::{AddressMode, Instruction, Mnemonic};
use parcel::prelude::v1::*;
use parcel::MatchStatus;

mod address_mode;
mod instructions;

macro_rules! gen_program_test {
    ($input:literal, $insts:expr) => {
        assert_eq!(
            Ok(MatchStatus::Match((&$input[$input.len()..], $insts))),
            $crate::parser::instructions().parse(&$input)
        );
    };
}

#[test]
fn should_parse_multiple_instructions_until_eof() {
    gen_program_test!(
        "nop
lda #$12
sta $1234
jmp $1234",
        vec![
            Instruction::new(Mnemonic::NOP, AddressMode::Implied),
            Instruction::new(Mnemonic::LDA, AddressMode::Immediate(0x12)),
            Instruction::new(Mnemonic::STA, AddressMode::Absolute(0x1234)),
            Instruction::new(Mnemonic::JMP, AddressMode::Absolute(0x1234))
        ]
    )
}

#[test]
fn should_parse_arbitrary_newlines_and_whitespaces_before_instruction() {
    gen_program_test!(
        "
        
        nop
lda #$12

sta $1234
jmp $1234",
        vec![
            Instruction::new(Mnemonic::NOP, AddressMode::Implied),
            Instruction::new(Mnemonic::LDA, AddressMode::Immediate(0x12)),
            Instruction::new(Mnemonic::STA, AddressMode::Absolute(0x1234)),
            Instruction::new(Mnemonic::JMP, AddressMode::Absolute(0x1234))
        ]
    )
}

#[test]
fn should_parse_singleline_comment() {
    gen_program_test!(
        "; test comment 
",
        vec![]
    )
}

#[test]
fn should_ignore_comment_lines() {
    gen_program_test!(
        "; nop
lda #$12 ; this is the first instruction
sta $1234
jmp $1234",
        vec![
            Instruction::new(Mnemonic::LDA, AddressMode::Immediate(0x12)),
            Instruction::new(Mnemonic::STA, AddressMode::Absolute(0x1234)),
            Instruction::new(Mnemonic::JMP, AddressMode::Absolute(0x1234))
        ]
    )
}
