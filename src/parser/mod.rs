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
    one_or_more(instruction())
}

#[allow(dead_code)]
pub fn instruction<'a>() -> impl parcel::Parser<'a, &'a str, Instruction> {
    join(
        right(join(zero_or_more(whitespace()), mnemonic())),
        left(join(
            optional(right(join(
                one_or_more(whitespace()),
                left(join(address_mode(), zero_or_more(whitespace()))),
            ))),
            zero_or_more(whitespace()),
        )),
    )
    .map(|(m, a)| match a {
        Some(am) => Instruction::new(m, am),
        None => Instruction::new(m, AddressMode::Implied),
    })
}

fn mnemonic<'a>() -> impl parcel::Parser<'a, &'a str, Mnemonic> {
    one_or_more(alphabetic())
        .map(|m| Mnemonic::try_from(m.into_iter().collect::<String>().as_str()).unwrap())
}

#[allow(clippy::redundant_closure)]
fn address_mode<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    accumulator()
        .or(|| absolute())
        .or(|| absolute_x_indexed())
        .or(|| absolute_y_indexed())
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
    character('A').map(|_| AddressMode::Accumulator)
}

fn absolute<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        character('$'),
        left(join(take_n(hex(), 4), whitespace().or(|| eof()))),
    ))
    .map(|h| AddressMode::Absolute(hex_char_vec_to_u16!(h)))
}

fn absolute_x_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        character('$'),
        left(join(take_n(hex(), 4), join(character(','), character('X')))),
    ))
    .map(|h| AddressMode::AbsoluteIndexedWithX(hex_char_vec_to_u16!(h)))
}

fn absolute_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        character('$'),
        left(join(take_n(hex(), 4), join(character(','), character('Y')))),
    ))
    .map(|h| AddressMode::AbsoluteIndexedWithY(hex_char_vec_to_u16!(h)))
}

#[allow(dead_code)]
fn immediate<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(character('#'), take_n(hex(), 2)))
        .map(|h| AddressMode::Immediate(hex_char_vec_to_u8!(h)))
}

fn indirect<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        join(character('('), character('$')),
        left(join(take_n(hex(), 4), character(')'))),
    ))
    .map(|h| AddressMode::Indirect(hex_char_vec_to_u16!(h)))
}

fn x_indexed_indirect<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        join(character('('), character('$')),
        left(join(
            take_n(hex(), 2),
            join(join(character(','), character('X')), character(')')),
        )),
    ))
    .map(|h| AddressMode::IndexedIndirect(hex_char_vec_to_u8!(h)))
}

fn indirect_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        join(character('('), character('$')),
        left(join(
            take_n(hex(), 2),
            join(join(character(')'), character(',')), character('Y')),
        )),
    ))
    .map(|h| AddressMode::IndirectIndexed(hex_char_vec_to_u8!(h)))
}

// Needs implementation of signed bits
#[allow(dead_code)]
fn relative<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        character('*'),
        join(character('+').or(|| character('-')), take_n(hex(), 2)),
    ))
    .map(|(_sign, h)| AddressMode::Relative(hex_char_vec_to_i8!(h)))
}

fn zeropage<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        character('$'),
        left(join(take_n(hex(), 2), whitespace().or(|| eof()))),
    ))
    .map(|h| AddressMode::ZeroPage(hex_char_vec_to_u8!(h)))
}

fn zeropage_x_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        character('$'),
        left(join(take_n(hex(), 2), join(character(','), character('X')))),
    ))
    .map(|h| AddressMode::ZeroPageIndexedWithX(hex_char_vec_to_u8!(h)))
}

fn zeropage_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        character('$'),
        left(join(take_n(hex(), 2), join(character(','), character('Y')))),
    ))
    .map(|h| AddressMode::ZeroPageIndexedWithY(hex_char_vec_to_u8!(h)))
}
