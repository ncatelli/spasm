extern crate parcel;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

pub fn whitespace<'a>() -> impl Parser<'a, &'a str, char> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if next.is_whitespace() => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}

pub fn alphabetic<'a>() -> impl Parser<'a, &'a str, char> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if next.is_alphabetic() => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}

pub fn eof<'a>() -> impl Parser<'a, &'a str, char> {
    move |input: &'a str| match input.chars().next() {
        Some(_) => Ok(MatchStatus::NoMatch(input)),
        None => Ok(MatchStatus::Match((&input[0..], ' '))),
    }
}

pub fn character<'a>(expected: char) -> impl Parser<'a, &'a str, char> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if next == expected => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}

#[allow(dead_code)]
pub fn hex<'a>() -> impl Parser<'a, &'a str, char> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if next.is_ascii_hexdigit() => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}
