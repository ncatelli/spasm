use crate::instruction_set::address_mode::{
    AddressMode, AddressModeOrReference, AddressModeType, Symbol,
};
use crate::instruction_set::Mnemonic;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

mod address_mode;
mod instructions;

macro_rules! gen_instruction_only_program_test {
    ($input:literal, $insts:expr) => {
        assert_eq!(
            Ok(MatchStatus::Match((
                &$input[$input.len()..],
                $insts
                    .into_iter()
                    .map(|i| $crate::instruction_set::InstructionOrDefinition::Instruction(i))
                    .collect()
            ))),
            $crate::parser::instructions().parse(&$input)
        );
    };
}

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
    gen_instruction_only_program_test!(
        "nop
lda #$12
sta $1234
jmp $1234",
        vec![
            instruction!(
                Mnemonic::NOP,
                AddressModeOrReference::AddressMode(AddressMode::Implied)
            ),
            instruction!(
                Mnemonic::LDA,
                AddressModeOrReference::AddressMode(AddressMode::Immediate(0x12))
            ),
            instruction!(
                Mnemonic::STA,
                AddressModeOrReference::AddressMode(AddressMode::Absolute(0x1234))
            ),
            instruction!(
                Mnemonic::JMP,
                AddressModeOrReference::AddressMode(AddressMode::Absolute(0x1234))
            )
        ]
    )
}

#[test]
fn should_parse_arbitrary_newlines_and_whitespaces_before_instruction() {
    gen_instruction_only_program_test!(
        "
        
        nop
lda #$12

sta $1234
jmp $1234",
        vec![
            instruction!(
                Mnemonic::NOP,
                AddressModeOrReference::AddressMode(AddressMode::Implied)
            ),
            instruction!(
                Mnemonic::LDA,
                AddressModeOrReference::AddressMode(AddressMode::Immediate(0x12))
            ),
            instruction!(
                Mnemonic::STA,
                AddressModeOrReference::AddressMode(AddressMode::Absolute(0x1234))
            ),
            instruction!(
                Mnemonic::JMP,
                AddressModeOrReference::AddressMode(AddressMode::Absolute(0x1234))
            )
        ]
    )
}

#[test]
fn should_parse_labels() {
    gen_program_test!(
        "
init:
  nop
  lda #$12
  sta $1234
  jmp $1234",
        vec![
            iod_label!("init"),
            iod_instruction!(instruction!(
                Mnemonic::NOP,
                AddressModeOrReference::AddressMode(AddressMode::Implied)
            )),
            iod_instruction!(instruction!(
                Mnemonic::LDA,
                AddressModeOrReference::AddressMode(AddressMode::Immediate(0x12))
            )),
            iod_instruction!(instruction!(
                Mnemonic::STA,
                AddressModeOrReference::AddressMode(AddressMode::Absolute(0x1234))
            )),
            iod_instruction!(instruction!(
                Mnemonic::JMP,
                AddressModeOrReference::AddressMode(AddressMode::Absolute(0x1234))
            ))
        ]
    )
}

#[test]
fn should_parse_symbols() {
    gen_program_test!(
        "
define thisisatest $12
nop
lda #thisisatest
sta $1234
jmp $1234",
        vec![
            iod_symbol!("thisisatest", 0x12),
            iod_instruction!(instruction!(
                Mnemonic::NOP,
                AddressModeOrReference::AddressMode(AddressMode::Implied)
            )),
            iod_instruction!(instruction!(
                Mnemonic::LDA,
                AddressModeOrReference::Symbol(Symbol::new(
                    AddressModeType::Immediate,
                    "thisisatest".to_string()
                ))
            )),
            iod_instruction!(instruction!(
                Mnemonic::STA,
                AddressModeOrReference::AddressMode(AddressMode::Absolute(0x1234))
            )),
            iod_instruction!(instruction!(
                Mnemonic::JMP,
                AddressModeOrReference::AddressMode(AddressMode::Absolute(0x1234))
            ))
        ]
    )
}

#[test]
fn should_parse_singleline_comment() {
    gen_instruction_only_program_test!("; test comment", vec![])
}

#[test]
fn should_ignore_comment_lines() {
    gen_instruction_only_program_test!(
        "; nop
lda #$12 ; this is the first instruction
sta $1234
jmp $1234",
        vec![
            instruction!(
                Mnemonic::LDA,
                AddressModeOrReference::AddressMode(AddressMode::Immediate(0x12))
            ),
            instruction!(
                Mnemonic::STA,
                AddressModeOrReference::AddressMode(AddressMode::Absolute(0x1234))
            ),
            instruction!(
                Mnemonic::JMP,
                AddressModeOrReference::AddressMode(AddressMode::Absolute(0x1234))
            )
        ]
    )
}
