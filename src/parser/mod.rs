extern crate parcel;
use crate::instruction_set::address_mode::AddressMode;
use crate::instruction_set::mnemonics::Mnemonic;
use crate::instruction_set::Instruction;
use parcel::prelude::v1::*;
use parcel::{join, left, one_or_more, optional, predicate, right, zero_or_more, MatchStatus};

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub fn instruction<'a>() -> impl parcel::Parser<'a, &'a [char], Instruction> {
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

fn mnemonic<'a>() -> impl parcel::Parser<'a, &'a [char], Mnemonic> {
    one_or_more(predicate(alphabetic(), |&c| !c.is_whitespace())).map(|_m| Mnemonic::NOP)
}

fn address_mode<'a>() -> impl parcel::Parser<'a, &'a [char], AddressMode> {
    zero_or_more(predicate(alphabetic(), |&c| !c.is_whitespace())).map(|_a| AddressMode::Implied)
}

fn whitespace<'a>() -> impl Parser<'a, &'a [char], char> {
    move |input: &'a [char]| match input.get(0) {
        Some(&next) if next.is_whitespace() => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}

fn alphabetic<'a>() -> impl Parser<'a, &'a [char], char> {
    move |input: &'a [char]| match input.get(0) {
        Some(&next) if next.is_alphabetic() => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}
