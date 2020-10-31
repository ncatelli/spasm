extern crate parcel;
use parcel::parsers::character::{expect_character, expect_str};
use parcel::prelude::v1::*;
use parcel::MatchStatus;
use parcel::{join, one_or_more, optional, right, take_n};

macro_rules! char_vec_to_u32_from_radix {
    ($chars:expr, $radix:expr) => {
        u32::from_le(u32::from_str_radix(&$chars.into_iter().collect::<String>(), $radix).unwrap())
    };
}

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

pub fn non_whitespace_character<'a>() -> impl Parser<'a, &'a [char], char> {
    move |input: &'a [char]| match input.get(0) {
        Some(&next) if !next.is_whitespace() => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}

pub fn unsigned32<'a>() -> impl Parser<'a, &'a [char], u32> {
    hex_u32().or(|| binary_u32()).or(|| dec_u32())
}

pub fn unsigned16<'a>() -> impl Parser<'a, &'a [char], u16> {
    hex_u16().or(|| binary_u16()).or(|| dec_u16())
}

pub fn unsigned8<'a>() -> impl Parser<'a, &'a [char], u8> {
    hex_u8().or(|| binary_u8()).or(|| dec_u8())
}

pub fn signed8<'a>() -> impl Parser<'a, &'a [char], i8> {
    hex_i8().or(|| binary_i8()).or(|| dec_i8())
}

fn sign<'a>() -> impl Parser<'a, &'a [char], Sign> {
    expect_character('+')
        .or(|| expect_character('-'))
        .map(|c| match c {
            '-' => Sign::Negative,
            _ => Sign::Positive,
        })
}

fn hex_u32<'a>() -> impl Parser<'a, &'a [char], u32> {
    right(join(expect_str("0x"), hex_bytes(4))).map(|hex| char_vec_to_u32_from_radix!(hex, 16))
}

fn hex_u16<'a>() -> impl Parser<'a, &'a [char], u16> {
    right(join(expect_str("0x"), hex_bytes(2))).map(|hex| char_vec_to_u16_from_radix!(hex, 16))
}

fn hex_u8<'a>() -> impl Parser<'a, &'a [char], u8> {
    right(join(expect_str("0x"), hex_bytes(1))).map(|hex| char_vec_to_u8_from_radix!(hex, 16))
}

fn hex_i8<'a>() -> impl Parser<'a, &'a [char], i8> {
    right(join(expect_str("0x"), hex_bytes(1))).map(|hex| char_vec_to_i8_from_radix!(hex, 16))
}

pub fn hex_bytes<'a>(bytes: usize) -> impl Parser<'a, &'a [char], Vec<char>> {
    take_n(hex_digit(), bytes * 2)
}

pub fn hex_digit<'a>() -> impl Parser<'a, &'a [char], char> {
    move |input: &'a [char]| match input.get(0) {
        Some(&next) if next.is_digit(16) => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}

fn binary_u32<'a>() -> impl Parser<'a, &'a [char], u32> {
    right(join(expect_str("0b"), binary_bytes(4))).map(|bin| char_vec_to_u32_from_radix!(bin, 2))
}

fn binary_u16<'a>() -> impl Parser<'a, &'a [char], u16> {
    right(join(expect_str("0b"), binary_bytes(2))).map(|bin| char_vec_to_u16_from_radix!(bin, 2))
}

fn binary_u8<'a>() -> impl Parser<'a, &'a [char], u8> {
    right(join(expect_str("0b"), binary_bytes(1))).map(|bin| char_vec_to_u8_from_radix!(bin, 2))
}

fn binary_i8<'a>() -> impl Parser<'a, &'a [char], i8> {
    right(join(expect_str("0b"), binary_bytes(1))).map(|bin| char_vec_to_i8_from_radix!(bin, 2))
}

pub fn binary_bytes<'a>(bytes: usize) -> impl Parser<'a, &'a [char], Vec<char>> {
    take_n(binary(), bytes * 8)
}

pub fn binary<'a>() -> impl Parser<'a, &'a [char], char> {
    move |input: &'a [char]| match input.get(0) {
        Some(&next) if next.is_digit(2) => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}

fn dec_u32<'a>() -> impl Parser<'a, &'a [char], u32> {
    move |input: &'a [char]| {
        let preparsed_input = input;
        let res = one_or_more(decimal())
            .map(|digits| {
                let vd: String = digits.into_iter().collect();
                u32::from_str_radix(&vd, 10)
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

fn dec_u16<'a>() -> impl Parser<'a, &'a [char], u16> {
    move |input: &'a [char]| {
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

fn dec_u8<'a>() -> impl Parser<'a, &'a [char], u8> {
    move |input: &'a [char]| {
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

fn dec_i8<'a>() -> impl Parser<'a, &'a [char], i8> {
    move |input: &'a [char]| {
        let preparsed_input = input;
        let res = join(optional(sign()), one_or_more(decimal()))
            .map(|(sign, digits)| {
                let pos_or_neg = match sign {
                    Some(Sign::Negative) => '-',
                    _ => '+',
                };

                let vd: String = vec![pos_or_neg]
                    .into_iter()
                    .chain(digits.into_iter())
                    .collect();
                i8::from_str_radix(&vd, 10)
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

pub fn decimal<'a>() -> impl Parser<'a, &'a [char], char> {
    move |input: &'a [char]| match input.get(0) {
        Some(&next) if next.is_digit(10) => Ok(MatchStatus::Match((&input[1..], next))),
        _ => Ok(MatchStatus::NoMatch(input)),
    }
}
