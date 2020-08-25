extern crate parcel;
use crate::instruction_set::address_mode::AddressMode;
use crate::instruction_set::mnemonics::Mnemonic;
use crate::instruction_set::Instruction;
use parcel::prelude::v1::*;
use parcel::{join, left, one_or_more, optional, right, zero_or_more};

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
            optional(left(join(address_mode(), one_or_more(whitespace())))),
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
        .or(|| implied())
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

/// TODO
fn absolute<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    character('A').map(|_| AddressMode::Accumulator)
}

/// TODO
fn absolute_x_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    character('A').map(|_| AddressMode::Accumulator)
}

/// TODO
fn absolute_y_indexed<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    character('A').map(|_| AddressMode::Accumulator)
}

/// TODO
fn implied<'a>() -> impl parcel::Parser<'a, &'a str, AddressMode> {
    character('A').map(|_| AddressMode::Accumulator)
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
