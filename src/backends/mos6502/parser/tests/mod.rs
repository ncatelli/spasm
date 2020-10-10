use crate::backends::mos6502::instruction_set::address_mode::{
    AddressMode, AddressModeOrReference, AddressModeType, Symbol,
};
use crate::backends::mos6502::instruction_set::Mnemonic;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

mod address_mode;
mod instructions;

macro_rules! gen_instruction_only_program_test {
    ($input:expr, $insts:expr) => {
        assert_eq!(
            Ok(MatchStatus::Match((
                &$input[$input.len()..],
                $insts
                    .into_iter()
                    .map(|i| $crate::backends::mos6502::instruction_set::InstructionOrDefinition::Instruction(i))
                    .collect()
            ))),
            $crate::backends::mos6502::parser::instructions().parse($input)
        );
    };
}

macro_rules! gen_program_test {
    ($input:expr, $insts:expr) => {
        assert_eq!(
            Ok(MatchStatus::Match((&$input[$input.len()..], $insts))),
            $crate::backends::mos6502::parser::instructions().parse($input)
        );
    };
}

macro_rules! chars {
    ($input:expr) => {
        $input.chars().collect::<Vec<char>>()
    };
}

#[test]
fn should_parse_multiple_instructions_until_eof() {
    let input = chars!(
        "nop
lda #$12
sta $1234
jmp $1234"
    );
    gen_instruction_only_program_test!(
        &input,
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
    );
}

#[test]
fn should_parse_arbitrary_newlines_and_whitespaces_before_instruction() {
    let input = chars!(
        "
        
    nop
lda #$12

sta $1234
jmp $1234"
    );
    gen_instruction_only_program_test!(
        &input,
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
    );
}

#[test]
fn should_parse_labels() {
    let input = chars!(
        "
init:
  nop
  lda #$12
  sta $1234
  jmp $1234"
    );
    gen_program_test!(
        &input,
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
    );
}

#[test]
fn should_parse_symbols() {
    let input = chars!(
        "
define thisisatest $12
nop
lda #thisisatest
sta $1234
jmp $1234"
    );
    gen_program_test!(
        &input,
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
    );
}

#[test]
fn should_parse_singleline_comment() {
    let input = chars!("; test comment");
    gen_instruction_only_program_test!(&input, vec![]);
}

#[test]
fn should_ignore_comment_lines() {
    let input = chars!(
        "; nop
lda #$12 ; this is the first instruction
sta $1234
jmp $1234"
    );
    gen_instruction_only_program_test!(
        &input,
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
    );
}
