use crate::backends::mos6502::instruction_set::addressing_mode::{
    AddressingModeOrReference, Symbol,
};
use crate::backends::mos6502::instruction_set::Instruction;
use isa_mos6502::{
    addressing_mode::{AddressingMode, AddressingModeType},
    mnemonic::Mnemonic,
};
use parcel::parsers::character::*;
use parcel::prelude::v1::*;
use parcel::{join, left, one_or_more, optional, right, take_n, zero_or_more};
use std::convert::TryFrom;

use crate::parser::*;

#[cfg(test)]
mod tests;

/// Error type returned from backends.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ParseErr {
    Unmatched(String),
    Unspecified(String),
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ParseErr::Unmatched(input) => format!("unable to parse expected tokens: {}", input),
            ParseErr::Unspecified(input) => format!("unspecified parse error: {}", input),
        };

        write!(f, "{}", output)
    }
}

pub fn instruction<'a>() -> impl parcel::Parser<'a, &'a [char], Instruction> {
    join(
        right(join(zero_or_more(non_newline_whitespace()), mnemonic())),
        left(join(
            optional(right(join(
                one_or_more(non_newline_whitespace()),
                address_mode(),
            ))),
            zero_or_more(non_newline_whitespace()),
        )),
    )
    .map(|(m, a)| match a {
        Some(amor) => Instruction::new(m, amor),
        None => Instruction::new(
            m,
            AddressingModeOrReference::AddressingMode(AddressingMode::Implied),
        ),
    })
}

fn mnemonic<'a>() -> impl parcel::Parser<'a, &'a [char], Mnemonic> {
    take_n(alphabetic(), 3)
        .map(|m| Mnemonic::try_from(m.into_iter().collect::<String>().as_str()).unwrap())
}

#[allow(clippy::redundant_closure)]
fn address_mode<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    accumulator()
        .or(|| zeropage())
        .or(|| zeropage_x_indexed())
        .or(|| zeropage_y_indexed())
        .or(|| absolute_x_indexed())
        .or(|| absolute_y_indexed())
        .or(|| x_indexed_indirect())
        .or(|| indirect_y_indexed())
        .or(|| absolute())
        .or(|| immediate())
        .or(|| indirect())
        .or(|| relative())
        .map(|amor| amor)
        .or(|| label().map(|l| AddressingModeOrReference::Label(l)))
}

fn label<'a>() -> impl parcel::Parser<'a, &'a [char], String> {
    one_or_more(alphabetic()).map(|l| l.into_iter().collect())
}

fn symbol<'a>() -> impl parcel::Parser<'a, &'a [char], String> {
    one_or_more(alphabetic()).map(|l| l.into_iter().collect())
}

fn accumulator<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    expect_character('A')
        .map(|_| AddressingModeOrReference::AddressingMode(AddressingMode::Accumulator))
}

fn absolute<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    unsigned16().map(|h| AddressingModeOrReference::AddressingMode(AddressingMode::Absolute(h)))
}

fn absolute_x_indexed<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    left(join(
        unsigned16(),
        join(expect_character(','), expect_character('X')),
    ))
    .map(|h| AddressingModeOrReference::AddressingMode(AddressingMode::AbsoluteIndexedWithX(h)))
}

fn absolute_y_indexed<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    left(join(
        unsigned16(),
        join(expect_character(','), expect_character('Y')),
    ))
    .map(|h| AddressingModeOrReference::AddressingMode(AddressingMode::AbsoluteIndexedWithY(h)))
}

fn immediate<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    right(join(expect_character('#'), unsigned8()))
        .map(|u| AddressingModeOrReference::AddressingMode(AddressingMode::Immediate(u)))
        .or(|| {
            right(join(expect_character('#'), symbol())).map(|sym| {
                AddressingModeOrReference::Symbol(Symbol::new(AddressingModeType::Immediate, sym))
            })
        })
}

fn indirect<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    right(join(
        expect_character('('),
        left(join(unsigned16(), expect_character(')'))),
    ))
    .map(|bytes| AddressingModeOrReference::AddressingMode(AddressingMode::Indirect(bytes)))
}

fn x_indexed_indirect<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    right(join(
        expect_character('('),
        left(join(
            unsigned8(),
            join(
                join(expect_character(','), expect_character('X')),
                expect_character(')'),
            ),
        )),
    ))
    .map(|u| AddressingModeOrReference::AddressingMode(AddressingMode::XIndexedIndirect(u)))
    .or(|| {
        right(join(
            expect_character('('),
            left(join(
                symbol(),
                join(
                    join(expect_character(','), expect_character('X')),
                    expect_character(')'),
                ),
            )),
        ))
        .map(|sym| {
            AddressingModeOrReference::Symbol(Symbol::new(
                AddressingModeType::XIndexedIndirect,
                sym,
            ))
        })
    })
}

fn indirect_y_indexed<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    right(join(
        expect_character('('),
        left(join(
            unsigned8(),
            join(
                join(expect_character(')'), expect_character(',')),
                expect_character('Y'),
            ),
        )),
    ))
    .map(|u| AddressingModeOrReference::AddressingMode(AddressingMode::IndirectYIndexed(u)))
    .or(|| {
        right(join(
            expect_character('('),
            left(join(
                symbol(),
                join(
                    join(expect_character(')'), expect_character(',')),
                    expect_character('Y'),
                ),
            )),
        ))
        .map(|sym| {
            AddressingModeOrReference::Symbol(Symbol::new(
                AddressingModeType::IndirectYIndexed,
                sym,
            ))
        })
    })
}

fn relative<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    right(join(expect_character('*'), signed8()))
        .map(|i| AddressingModeOrReference::AddressingMode(AddressingMode::Relative(i)))
}

#[allow(clippy::clippy::redundant_closure)]
fn zeropage<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    left(join(unsigned8(), non_newline_whitespace().or(|| eof())))
        .map(|u| AddressingModeOrReference::AddressingMode(AddressingMode::ZeroPage(u)))
}

fn zeropage_x_indexed<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    left(join(
        unsigned8(),
        join(expect_character(','), expect_character('X')),
    ))
    .map(|u| AddressingModeOrReference::AddressingMode(AddressingMode::ZeroPageIndexedWithX(u)))
}

fn zeropage_y_indexed<'a>() -> impl parcel::Parser<'a, &'a [char], AddressingModeOrReference> {
    left(join(
        unsigned8(),
        join(expect_character(','), expect_character('Y')),
    ))
    .map(|u| AddressingModeOrReference::AddressingMode(AddressingMode::ZeroPageIndexedWithY(u)))
}
