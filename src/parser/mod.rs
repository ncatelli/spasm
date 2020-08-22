extern crate parcel;
use crate::instruction_set::address_mode::AddressMode;
use crate::instruction_set::mnemonics::Mnemonic;
use crate::instruction_set::Instruction;
use parcel::prelude::v1::*;
use parcel::{join, left, one_or_more, optional, right, zero_or_more, MatchStatus};

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
    zero_or_more(alphabetic()).map(|_a| AddressMode::Implied)
}

fn whitespace<'a>() -> impl Parser<'a, &'a str, &'a str> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if next.is_whitespace() => Ok(MatchStatus::Match((&input[1..], &input[0..1]))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}

fn alphabetic<'a>() -> impl Parser<'a, &'a str, &'a str> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if next.is_alphabetic() => Ok(MatchStatus::Match((&input[1..], &input[0..1]))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}
