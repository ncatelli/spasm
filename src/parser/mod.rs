extern crate parcel;
use crate::instruction_set::mnemonics::Mnemonic;
use parcel::prelude::v1::*;
use parcel::{join, left, predicate, right, zero_or_more, MatchStatus};

#[cfg(test)]
mod tests;

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

pub fn mnemonic<'a>() -> impl parcel::Parser<'a, &'a [char], Mnemonic> {
    right(join(
        zero_or_more(whitespace()),
        zero_or_more(predicate(alphabetic(), |&c| c.is_whitespace())),
    ))
    .map(|_m| Mnemonic::NOP)
}
