extern crate parcel;
use crate::instruction_set::address_mode::AddressMode;
use crate::instruction_set::mnemonics::Mnemonic;
use crate::instruction_set::Instruction;
use parcel::prelude::v1::*;
use parcel::{join, left, one_or_more, optional, right, take_n, zero_or_more};

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

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub fn instruction<'a>() -> impl parcel::Parser<'a, &'a str, Instruction> {
    join(
        right(join(zero_or_more(whitespace()), mnemonic())),
        right(join(
            one_or_more(whitespace()),
            optional(left(join(address_mode(), zero_or_more(whitespace())))),
        )),
    )
    .map(|(m, a)| match a {
        Some(am) => Instruction::new(m, am),
        None => Instruction::new(m, AddressMode::Implied),
    })
}

fn mnemonic<'a>() -> impl parcel::Parser<'a, &'a str, Mnemonic> {
    one_or_more(alphabetic()).map(|_m| Mnemonic::NOP)
}

fn address_mode<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    accumulator()
        .or(|| absolute())
        .or(|| absolute_x_indexed())
        .or(|| absolute_y_indexed())
        .or(|| immediate())
        .or(|| indirect())
        .or(|| x_indexed_indirect())
        .or(|| indirect_y_indexed())
        .or(|| relative())
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
        left(join(take_n(hex(), 4), one_or_more(whitespace()))),
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

fn immediate<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(join(character('#'), character('$')), take_n(hex(), 2)))
        .map(|h| AddressMode::Immediate(hex_char_vec_to_u8!(h)))
}

/// TODO
fn indirect<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    character('A').map(|_| AddressMode::Accumulator)
}

/// TODO
fn x_indexed_indirect<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    character('A').map(|_| AddressMode::Accumulator)
}

/// TODO
fn indirect_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    character('A').map(|_| AddressMode::Accumulator)
}

/// TODO
fn relative<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    character('A').map(|_| AddressMode::Accumulator)
}

/// TODO
fn zeropage<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    character('A').map(|_| AddressMode::Accumulator)
}

/// TODO
fn zeropage_x_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    character('A').map(|_| AddressMode::Accumulator)
}

/// TODO
fn zeropage_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    character('A').map(|_| AddressMode::Accumulator)
}
