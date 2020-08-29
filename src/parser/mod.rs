extern crate parcel;
use crate::instruction_set::address_mode::AddressMode;
use crate::instruction_set::mnemonics::Mnemonic;
use crate::instruction_set::Instruction;
use parcel::prelude::v1::*;
use parcel::{join, left, one_or_more, optional, right, take_n, zero_or_more};

mod combinators;
use combinators::*;

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
    .map(|h| {
        let hex_str: String = h.into_iter().collect();
        let addr = u16::from_str_radix(&hex_str, 16).unwrap();
        AddressMode::Absolute(u16::from_le(addr))
    })
}

fn absolute_x_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        character('$'),
        left(join(take_n(hex(), 4), join(character(','), character('X')))),
    ))
    .map(|h| {
        let hex_str: String = h.into_iter().collect();
        let addr = u16::from_str_radix(&hex_str, 16).unwrap();
        AddressMode::AbsoluteIndexedWithX(u16::from_le(addr))
    })
}

fn absolute_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    right(join(
        character('$'),
        left(join(take_n(hex(), 4), join(character(','), character('Y')))),
    ))
    .map(|h| {
        let hex_str: String = h.into_iter().collect();
        let addr = u16::from_str_radix(&hex_str, 16).unwrap();
        AddressMode::AbsoluteIndexedWithX(u16::from_le(addr))
    })
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
