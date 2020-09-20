extern crate parcel;
use crate::instruction_set::address_mode::AddressMode;
use crate::instruction_set::mnemonics::Mnemonic;
use crate::instruction_set::Instruction;
use parcel::prelude::v1::*;
use parcel::{join, left, one_or_more, optional, right, take_n, zero_or_more};
use std::convert::TryFrom;

mod combinators;
use combinators::*;

#[cfg(test)]
mod tests;

pub fn instructions<'a>() -> impl parcel::Parser<'a, &'a str, Vec<Instruction>> {
    one_or_more(right(join(
        zero_or_more(whitespace().or(|| newline())),
        left(join(
            instruction()
                .map(|i| Some(i))
                .or(|| comment().map(|_| None)),
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

pub fn instruction<'a>() -> impl parcel::Parser<'a, &'a str, Instruction> {
    join(
        right(join(zero_or_more(whitespace()), mnemonic())),
        left(join(
            optional(right(join(one_or_more(whitespace()), address_mode()))),
            join(zero_or_more(whitespace()), optional(comment())),
        )),
    )
    .map(|(m, a)| match a {
        Some(am) => Instruction::new(m, am),
        None => Instruction::new(m, AddressMode::Implied),
    })
}

fn comment<'a>() -> impl parcel::Parser<'a, &'a str, ()> {
    right(join(
        expect_character(';'),
        zero_or_more(character().or(|| whitespace())),
    ))
    .map(|_| ())
}

fn mnemonic<'a>() -> impl parcel::Parser<'a, &'a str, Mnemonic> {
    take_n(alphabetic(), 3)
        .map(|m| Mnemonic::try_from(m.into_iter().collect::<String>().as_str()).unwrap())
}

#[allow(clippy::redundant_closure)]
fn address_mode<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
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
    //        .or(|| relative())
}

fn accumulator<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    expect_character('A').map(|_| AddressMode::Accumulator)
}

fn absolute<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    unsigned16().map(|h| AddressMode::Absolute(h))
}

fn absolute_x_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    left(join(
        unsigned16(),
        join(expect_character(','), expect_character('X')),
    ))
    .map(|h| AddressMode::AbsoluteIndexedWithX(h))
}

fn absolute_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    left(join(
        unsigned16(),
        join(expect_character(','), expect_character('Y')),
    ))
    .map(|h| AddressMode::AbsoluteIndexedWithY(h))
}

fn immediate<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(expect_character('#'), unsigned8())).map(|u| AddressMode::Immediate(u))
}

fn indirect<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        expect_character('('),
        left(join(unsigned16(), expect_character(')'))),
    ))
    .map(|bytes| AddressMode::Indirect(bytes))
}

fn x_indexed_indirect<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
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
    .map(|u| AddressMode::IndexedIndirect(u))
}

fn indirect_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
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
    .map(|u| AddressMode::IndirectIndexed(u))
}

// Needs implementation of signed bits
#[allow(dead_code)]
fn relative<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(expect_character('*'), signed8())).map(|i| AddressMode::Relative(i))
}

fn zeropage<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    left(join(unsigned8(), whitespace().or(|| eof()))).map(|u| AddressMode::ZeroPage(u))
}

fn zeropage_x_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    left(join(
        unsigned8(),
        join(expect_character(','), expect_character('X')),
    ))
    .map(|u| AddressMode::ZeroPageIndexedWithX(u))
}

fn zeropage_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    left(join(
        unsigned8(),
        join(expect_character(','), expect_character('Y')),
    ))
    .map(|u| AddressMode::ZeroPageIndexedWithY(u))
}
