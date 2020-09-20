extern crate parcel;
use parcel::prelude::v1::*;
use parcel::MatchStatus;
use parcel::{join, right, take_n};

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

#[derive(Clone, Copy, PartialEq)]
enum Sign {
    Positive,
    Negative,
}

impl PartialEq<char> for Sign {
    fn eq(&self, other: &char) -> bool {
        match self {
            &Self::Positive if *other == '+' => true,
            &Self::Negative if *other == '-' => true,
            _ => false,
        }
    }
}

// whitespaces matches any wh
pub fn whitespace<'a>() -> impl Parser<'a, &'a str, char> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if next.is_whitespace() && next != '\n' => {
            Ok(MatchStatus::Match((&input[1..], next)))
        }
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

pub fn newline<'a>() -> impl Parser<'a, &'a str, char> {
    expect_character('\n')
}

pub fn character<'a>() -> impl Parser<'a, &'a str, char> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if !next.is_whitespace() => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}

pub fn expect_character<'a>(expected: char) -> impl Parser<'a, &'a str, char> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if next == expected => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}

pub fn unsigned16<'a>() -> impl Parser<'a, &'a str, u16> {
    right(join(expect_character('$'), hexbytes(2))).map(|hex| hex_char_vec_to_u16!(hex))
}

pub fn unsigned8<'a>() -> impl Parser<'a, &'a str, u8> {
    right(join(expect_character('$'), hexbytes(1))).map(|hex| hex_char_vec_to_u8!(hex))
}

pub fn signed8<'a>() -> impl Parser<'a, &'a str, i8> {
    join(sign(), right(join(expect_character('$'), hexbytes(1)))).map(|(sign, hex)| {
        let signed_char_vec = if sign == Sign::Negative {
            vec![vec!['-'], hex].into_iter().flatten().collect()
        } else {
            hex
        };
        hex_char_vec_to_i8!(signed_char_vec)
    })
}

fn sign<'a>() -> impl Parser<'a, &'a str, Sign> {
    expect_character('+')
        .or(|| expect_character('-'))
        .map(|c| match c {
            '-' => Sign::Negative,
            _ => Sign::Positive,
        })
}

pub fn hexbytes<'a>(bytes: usize) -> impl Parser<'a, &'a str, Vec<char>> {
    take_n(hexdigit(), bytes * 2)
}

#[allow(dead_code)]
pub fn hexdigit<'a>() -> impl Parser<'a, &'a str, char> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if next.is_ascii_hexdigit() => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}
