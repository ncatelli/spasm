extern crate parcel;
use parcel::prelude::v1::*;
use parcel::MatchStatus;
use parcel::{join, one_or_more, right, take_n};

macro_rules! char_vec_to_u16_from_radix {
    ($chars:expr, $radix:expr) => {
        u16::from_le(u16::from_str_radix(&$chars.into_iter().collect::<String>(), $radix).unwrap())
    };
}

macro_rules! char_vec_to_u8_from_radix {
    ($chars:expr, $radix:expr) => {
        u8::from_le(u8::from_str_radix(&$chars.into_iter().collect::<String>(), $radix).unwrap())
    };
}

macro_rules! char_vec_to_i8_from_radix {
    ($chars:expr, $radix:expr) => {
        i8::from_le(i8::from_str_radix(&$chars.into_iter().collect::<String>(), $radix).unwrap())
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
    hex_u16().or(|| binary_u16()).or(|| dec_u16())
}

pub fn unsigned8<'a>() -> impl Parser<'a, &'a str, u8> {
    hex_u8().or(|| binary_u8()).or(|| dec_u8())
}

pub fn signed8<'a>() -> impl Parser<'a, &'a str, i8> {
    join(sign(), right(join(expect_character('$'), hex_bytes(1)))).map(|(sign, hex)| {
        let signed_char_vec = if sign == Sign::Negative {
            vec![vec!['-'], hex].into_iter().flatten().collect()
        } else {
            hex
        };
        char_vec_to_i8_from_radix!(signed_char_vec, 16)
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

fn hex_u16<'a>() -> impl Parser<'a, &'a str, u16> {
    right(join(expect_character('$'), hex_bytes(2))).map(|hex| char_vec_to_u16_from_radix!(hex, 16))
}

fn hex_u8<'a>() -> impl Parser<'a, &'a str, u8> {
    right(join(expect_character('$'), hex_bytes(1))).map(|hex| char_vec_to_u8_from_radix!(hex, 16))
}

pub fn hex_bytes<'a>(bytes: usize) -> impl Parser<'a, &'a str, Vec<char>> {
    take_n(hex_digit(), bytes * 2)
}

pub fn hex_digit<'a>() -> impl Parser<'a, &'a str, char> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if next.is_digit(16) => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}

fn binary_u16<'a>() -> impl Parser<'a, &'a str, u16> {
    right(join(expect_character('%'), binary_bytes(2)))
        .map(|hex| char_vec_to_u16_from_radix!(hex, 2))
}

fn binary_u8<'a>() -> impl Parser<'a, &'a str, u8> {
    right(join(expect_character('%'), binary_bytes(1)))
        .map(|hex| char_vec_to_u8_from_radix!(hex, 2))
}

pub fn binary_bytes<'a>(bytes: usize) -> impl Parser<'a, &'a str, Vec<char>> {
    take_n(binary(), bytes * 8)
}

pub fn binary<'a>() -> impl Parser<'a, &'a str, char> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if next.is_digit(2) => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}

fn dec_u16<'a>() -> impl Parser<'a, &'a str, u16> {
    move |input: &'a str| {
        let preparsed_input = input;
        let res = one_or_more(decimal())
            .map(|digits| {
                let vd: String = digits.into_iter().collect();
                u16::from_str_radix(&vd, 10)
            })
            .parse(input);

        match res {
            Ok(MatchStatus::Match((rem, Ok(u)))) => Ok(MatchStatus::Match((rem, u))),
            Ok(MatchStatus::Match((_, Err(_)))) => Ok(MatchStatus::NoMatch(preparsed_input)),
            Ok(MatchStatus::NoMatch(rem)) => Ok(MatchStatus::NoMatch(rem)),
            Err(e) => Err(e),
        }
    }
}

fn dec_u8<'a>() -> impl Parser<'a, &'a str, u8> {
    move |input: &'a str| {
        let preparsed_input = input;
        let res = one_or_more(decimal())
            .map(|digits| {
                let vd: String = digits.into_iter().collect();
                u8::from_str_radix(&vd, 10)
            })
            .parse(input);

        match res {
            Ok(MatchStatus::Match((rem, Ok(u)))) => Ok(MatchStatus::Match((rem, u))),
            Ok(MatchStatus::Match((_, Err(_)))) => Ok(MatchStatus::NoMatch(preparsed_input)),
            Ok(MatchStatus::NoMatch(rem)) => Ok(MatchStatus::NoMatch(rem)),
            Err(e) => Err(e),
        }
    }
}

#[allow(dead_code)]
pub fn decimal<'a>() -> impl Parser<'a, &'a str, char> {
    move |input: &'a str| match input.chars().next() {
        Some(next) if next.is_digit(10) => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}
