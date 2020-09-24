extern crate parcel;
use crate::instruction_set::address_mode::{
    AddressMode, AddressModeOrReference, AddressModeType, Symbol,
};
use crate::instruction_set::mnemonics::Mnemonic;
use crate::instruction_set::{Instruction, InstructionOrDefinition};
use parcel::prelude::v1::*;
use parcel::{join, left, one_or_more, optional, right, take_n, zero_or_more};
use std::convert::TryFrom;

mod combinators;
use combinators::*;

#[cfg(test)]
mod tests;

pub fn instructions<'a>() -> impl parcel::Parser<'a, &'a str, Vec<InstructionOrDefinition>> {
    one_or_more(right(join(
        zero_or_more(whitespace().or(|| newline())),
        left(join(
            labeldef()
                .map(|iod| Some(iod))
                .or(|| symboldef().map(|iod| Some(iod)))
                .or(|| comment().map(|_| None))
                .or(|| instruction().map(|i| Some(i))),
            newline().or(|| eof()),
        )),
    )))
    .map(|ioc| {
        ioc.into_iter()
            .filter(|oi| oi.is_some())
            .map(|oi| oi.unwrap())
            .collect()
    })
}

pub fn instruction<'a>() -> impl parcel::Parser<'a, &'a str, InstructionOrDefinition> {
    join(
        right(join(zero_or_more(whitespace()), mnemonic())),
        left(join(
            optional(right(join(one_or_more(whitespace()), address_mode()))),
            join(zero_or_more(whitespace()), optional(comment())),
        )),
    )
    .map(|(m, a)| match a {
        Some(amor) => Instruction::new(m, amor),
        None => Instruction::new(m, AddressModeOrReference::AddressMode(AddressMode::Implied)),
    })
    .map(|i| InstructionOrDefinition::Instruction(i))
}

fn comment<'a>() -> impl parcel::Parser<'a, &'a str, ()> {
    right(join(
        expect_character(';'),
        zero_or_more(character().or(|| whitespace())),
    ))
    .map(|_| ())
}

fn labeldef<'a>() -> impl parcel::Parser<'a, &'a str, InstructionOrDefinition> {
    left(join(one_or_more(alphabetic()), expect_character(':')))
        .map(|cv| InstructionOrDefinition::Label(cv.into_iter().collect()))
}

fn symboldef<'a>() -> impl parcel::Parser<'a, &'a str, InstructionOrDefinition> {
    right(join(
        join(expect_str("define"), one_or_more(whitespace())),
        join(
            left(join(one_or_more(alphabetic()), one_or_more(whitespace()))),
            unsigned8(),
        ),
    ))
    .map(|(s, v)| InstructionOrDefinition::Symbol((s.into_iter().collect(), v)))
}

fn mnemonic<'a>() -> impl parcel::Parser<'a, &'a str, Mnemonic> {
    take_n(alphabetic(), 3)
        .map(|m| Mnemonic::try_from(m.into_iter().collect::<String>().as_str()).unwrap())
}

#[allow(clippy::redundant_closure)]
fn address_mode<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
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
        .or(|| label().map(|l| AddressModeOrReference::Label(l)))
}

fn label<'a>() -> impl parcel::Parser<'a, &'a str, String> {
    one_or_more(alphabetic()).map(|l| l.into_iter().collect())
}

fn symbol<'a>() -> impl parcel::Parser<'a, &'a str, String> {
    one_or_more(alphabetic()).map(|l| l.into_iter().collect())
}

fn accumulator<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
    expect_character('A').map(|_| AddressModeOrReference::AddressMode(AddressMode::Accumulator))
}

fn absolute<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
    unsigned16().map(|h| AddressModeOrReference::AddressMode(AddressMode::Absolute(h)))
}

fn absolute_x_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
    left(join(
        unsigned16(),
        join(expect_character(','), expect_character('X')),
    ))
    .map(|h| AddressModeOrReference::AddressMode(AddressMode::AbsoluteIndexedWithX(h)))
}

fn absolute_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
    left(join(
        unsigned16(),
        join(expect_character(','), expect_character('Y')),
    ))
    .map(|h| AddressModeOrReference::AddressMode(AddressMode::AbsoluteIndexedWithY(h)))
}

fn immediate<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
    right(join(expect_character('#'), unsigned8()))
        .map(|u| AddressModeOrReference::AddressMode(AddressMode::Immediate(u)))
        .or(|| {
            right(join(expect_character('#'), symbol())).map(|sym| {
                AddressModeOrReference::Symbol(Symbol::new(AddressModeType::Immediate, sym))
            })
        })
}

fn indirect<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
    right(join(
        expect_character('('),
        left(join(unsigned16(), expect_character(')'))),
    ))
    .map(|bytes| AddressModeOrReference::AddressMode(AddressMode::Indirect(bytes)))
}

fn x_indexed_indirect<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
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
    .map(|u| AddressModeOrReference::AddressMode(AddressMode::IndexedIndirect(u)))
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
            AddressModeOrReference::Symbol(Symbol::new(AddressModeType::IndexedIndirect, sym))
        })
    })
}

fn indirect_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
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
    .map(|u| AddressModeOrReference::AddressMode(AddressMode::IndirectIndexed(u)))
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
            AddressModeOrReference::Symbol(Symbol::new(AddressModeType::IndirectIndexed, sym))
        })
    })
}

fn relative<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
    right(join(expect_character('*'), signed8()))
        .map(|i| AddressModeOrReference::AddressMode(AddressMode::Relative(i)))
}

fn zeropage<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
    left(join(unsigned8(), whitespace().or(|| eof())))
        .map(|u| AddressModeOrReference::AddressMode(AddressMode::ZeroPage(u)))
}

fn zeropage_x_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
    left(join(
        unsigned8(),
        join(expect_character(','), expect_character('X')),
    ))
    .map(|u| AddressModeOrReference::AddressMode(AddressMode::ZeroPageIndexedWithX(u)))
}

fn zeropage_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressModeOrReference> {
    left(join(
        unsigned8(),
        join(expect_character(','), expect_character('Y')),
    ))
    .map(|u| AddressModeOrReference::AddressMode(AddressMode::ZeroPageIndexedWithY(u)))
}
