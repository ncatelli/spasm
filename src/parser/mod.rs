extern crate parcel;
use crate::instruction_set::address_mode::AddressMode;
use crate::instruction_set::mnemonics::Mnemonic;
use crate::instruction_set::Instruction;
use parcel::prelude::v1::*;
use parcel::{join, left, one_or_more, optional, right, take_n, zero_or_more};
use std::convert::TryFrom;

mod combinators;
use combinators::*;

macro_rules! hex_char_vec_to_u16 {
    ($chars:expr) => {
        u16::from_le(u16::from_str_radix(&$chars.into_iter().collect::<String>(), 16).unwrap())
    };
}

macro_rules! hex_char_vec_to_u8 {
    ($chars:expr) => {
        u8::from_le(u8::from_str_radix(&$chars.into_iter().collect::<String>(), 16).unwrap())
    };
}

macro_rules! hex_char_vec_to_i8 {
    ($chars:expr) => {
        i8::from_le(i8::from_str_radix(&$chars.into_iter().collect::<String>(), 16).unwrap())
    };
}

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub fn instructions<'a>() -> impl parcel::Parser<'a, &'a str, Vec<Instruction>> {
    one_or_more(left(join(instruction(), newline().or(|| eof()))))
}

pub fn instruction<'a>() -> impl parcel::Parser<'a, &'a str, Instruction> {
    join(
        right(join(zero_or_more(whitespace()), mnemonic())),
        left(join(
            optional(right(join(one_or_more(whitespace()), address_mode()))),
            zero_or_more(whitespace()),
        )),
    )
    .map(|(m, a)| match a {
        Some(am) => Instruction::new(m, am),
        None => Instruction::new(m, AddressMode::Implied),
    })
}

#[allow(dead_code)]
fn comment<'a>() -> impl parcel::Parser<'a, &'a str, ()> {
    right(join(expect_character(';'), zero_or_more(character()))).map(|_| ())
}

fn mnemonic<'a>() -> impl parcel::Parser<'a, &'a str, Mnemonic> {
    take_n(alphabetic(), 3)
        .map(|m| Mnemonic::try_from(m.into_iter().collect::<String>().as_str()).unwrap())
}

#[allow(clippy::redundant_closure)]
fn address_mode<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    accumulator()
        .or(|| absolute_x_indexed())
        .or(|| absolute_y_indexed())
        .or(|| absolute())
        .or(|| immediate())
        .or(|| indirect())
        .or(|| x_indexed_indirect())
        .or(|| indirect_y_indexed())
        //        .or(|| relative())
        .or(|| zeropage())
        .or(|| zeropage_x_indexed())
        .or(|| zeropage_y_indexed())
}

fn accumulator<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    expect_character('A').map(|_| AddressMode::Accumulator)
}

fn absolute<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(expect_character('$'), take_n(hex(), 4)))
        .map(|h| AddressMode::Absolute(hex_char_vec_to_u16!(h)))
}

fn absolute_x_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        expect_character('$'),
        left(join(
            take_n(hex(), 4),
            join(expect_character(','), expect_character('X')),
        )),
    ))
    .map(|h| AddressMode::AbsoluteIndexedWithX(hex_char_vec_to_u16!(h)))
}

fn absolute_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        expect_character('$'),
        left(join(
            take_n(hex(), 4),
            join(expect_character(','), expect_character('Y')),
        )),
    ))
    .map(|h| AddressMode::AbsoluteIndexedWithY(hex_char_vec_to_u16!(h)))
}

#[allow(dead_code)]
fn immediate<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(expect_character('#'), take_n(hex(), 2)))
        .map(|h| AddressMode::Immediate(hex_char_vec_to_u8!(h)))
}

fn indirect<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        join(expect_character('('), expect_character('$')),
        left(join(take_n(hex(), 4), expect_character(')'))),
    ))
    .map(|h| AddressMode::Indirect(hex_char_vec_to_u16!(h)))
}

fn x_indexed_indirect<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        join(expect_character('('), expect_character('$')),
        left(join(
            take_n(hex(), 2),
            join(
                join(expect_character(','), expect_character('X')),
                expect_character(')'),
            ),
        )),
    ))
    .map(|h| AddressMode::IndexedIndirect(hex_char_vec_to_u8!(h)))
}

fn indirect_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        join(expect_character('('), expect_character('$')),
        left(join(
            take_n(hex(), 2),
            join(
                join(expect_character(')'), expect_character(',')),
                expect_character('Y'),
            ),
        )),
    ))
    .map(|h| AddressMode::IndirectIndexed(hex_char_vec_to_u8!(h)))
}

// Needs implementation of signed bits
#[allow(dead_code)]
fn relative<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        expect_character('*'),
        join(
            expect_character('+').or(|| expect_character('-')),
            take_n(hex(), 2),
        ),
    ))
    .map(|(_sign, h)| AddressMode::Relative(hex_char_vec_to_i8!(h)))
}

fn zeropage<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        expect_character('$'),
        left(join(take_n(hex(), 2), whitespace().or(|| eof()))),
    ))
    .map(|h| AddressMode::ZeroPage(hex_char_vec_to_u8!(h)))
}

fn zeropage_x_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        expect_character('$'),
        left(join(
            take_n(hex(), 2),
            join(expect_character(','), expect_character('X')),
        )),
    ))
    .map(|h| AddressMode::ZeroPageIndexedWithX(hex_char_vec_to_u8!(h)))
}

fn zeropage_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        expect_character('$'),
        left(join(
            take_n(hex(), 2),
            join(expect_character(','), expect_character('Y')),
        )),
    ))
    .map(|h| AddressMode::ZeroPageIndexedWithY(hex_char_vec_to_u8!(h)))
}
