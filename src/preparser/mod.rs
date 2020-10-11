extern crate parcel;
use parcel::parsers::character::*;
use parcel::prelude::v1::*;
use parcel::{join, left, one_of, one_or_more, right, zero_or_more};

// Pull in shared combinators
use crate::parser::*;

#[cfg(test)]
mod tests;

/// Token wraps the token variants that can be derived from the
/// parser.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Instruction(String),
    Label(String),
    Symbol((String, u8)),
}

#[derive(Default)]
pub struct PreParser {}

impl PreParser {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> Parser<'a, &'a [char], Vec<Token>> for PreParser {
    fn parse(&self, input: &'a [char]) -> ParseResult<'a, &'a [char], Vec<Token>> {
        statement().parse(input)
    }
}

#[allow(dead_code)]
pub fn statement<'a>() -> impl parcel::Parser<'a, &'a [char], Vec<Token>> {
    one_or_more(right(join(
        zero_or_more(non_newline_whitespace().or(|| newline())),
        left(join(
            labeldef()
                .map(|tok| Some(tok))
                .or(|| symboldef().map(|tok| Some(tok)))
                .or(|| instruction().map(|tok| Some(tok)))
                .or(|| comment().map(|_| None)),
            newline().or(|| eof()),
        )),
    )))
    .map(|ioc| {
        ioc.into_iter()
            .filter(|oi| oi.is_some())
            .map(|oi| oi.unwrap())
            .collect()
    })
}

#[allow(dead_code)]
fn instruction<'a>() -> impl parcel::Parser<'a, &'a [char], Token> {
    one_or_more(alphabetic().or(|| {
        non_newline_whitespace().or(|| digit(10)).or(|| {
            one_of(vec![
                expect_character('-'),
                expect_character('_'),
                expect_character('\\'),
                expect_character('#'),
                expect_character('&'),
                expect_character('\''),
                expect_character('|'),
                expect_character('('),
                expect_character(')'),
                expect_character('*'),
                expect_character('+'),
                expect_character(','),
                expect_character('.'),
                expect_character('/'),
                expect_character(':'),
                expect_character('<'),
                expect_character('='),
                expect_character('>'),
            ])
        })
    }))
    .map(|v| Token::Instruction(v.into_iter().collect()))
}

fn comment<'a>() -> impl parcel::Parser<'a, &'a [char], ()> {
    right(join(
        expect_character(';'),
        zero_or_more(non_whitespace_character().or(|| non_newline_whitespace())),
    ))
    .map(|_| ())
}

fn labeldef<'a>() -> impl parcel::Parser<'a, &'a [char], Token> {
    left(join(one_or_more(alphabetic()), expect_character(':')))
        .map(|cv| Token::Label(cv.into_iter().collect()))
}

fn symboldef<'a>() -> impl parcel::Parser<'a, &'a [char], Token> {
    right(join(
        join(expect_str("define"), one_or_more(non_newline_whitespace())),
        join(
            left(join(
                one_or_more(alphabetic()),
                one_or_more(non_newline_whitespace()),
            )),
            unsigned8(),
        ),
    ))
    .map(|(s, v)| Token::Symbol((s.into_iter().collect(), v)))
}
