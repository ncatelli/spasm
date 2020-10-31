extern crate parcel;
use crate::addressing::SizeOf;
use crate::{Emitter, Origin};
use parcel::parsers::character::*;
use parcel::prelude::v1::*;
use parcel::{join, left, one_of, one_or_more, optional, right, zero_or_more};

// Pull in shared combinators
use crate::parser::*;

#[cfg(test)]
mod tests;

/// Label represents a the string representation of a label.
pub type Label = String;

/// SymbolId represents a symbol identifier.
pub type SymbolId = String;

/// ByteValue represents a parser token value either representing a 1, 2, or
/// 4 byte value.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ByteValue {
    Byte(u8),
    Word(u16),
    DoubleWord(u32),
}

impl Emitter<Vec<u8>> for ByteValue {
    fn emit(&self) -> Vec<u8> {
        match self {
            ByteValue::Byte(v) => v.to_ne_bytes().to_vec(),
            ByteValue::Word(v) => v.to_ne_bytes().to_vec(),
            ByteValue::DoubleWord(v) => v.to_ne_bytes().to_vec(),
        }
    }
}

impl SizeOf for ByteValue {
    fn size_of(&self) -> usize {
        match self {
            ByteValue::Byte(_) => 1,
            ByteValue::Word(_) => 2,
            ByteValue::DoubleWord(_) => 4,
        }
    }
}

/// Token wraps the token variants that can be derived from the
/// parser.
#[derive(Debug, Clone, PartialEq)]
pub enum Token<T> {
    Instruction(T),
    Label(Label),
    Symbol((SymbolId, ByteValue)),
    Constant(ByteValue),
}

#[derive(Default)]
pub struct PreParser {}

impl PreParser {
    pub fn new() -> Self {
        Self::default()
    }
}

type PreparseTokenStream = Vec<Token<String>>;
type OriginStream = Vec<Origin<PreparseTokenStream>>;

impl<'a> Parser<'a, &'a [char], OriginStream> for PreParser {
    fn parse(&self, input: &'a [char]) -> ParseResult<'a, &'a [char], OriginStream> {
        join(
            origin_statements().or(|| statements().map(|tokens| Origin::new(tokens))),
            zero_or_more(origin_statements()),
        )
        .map(|(head, tail)| vec![head].into_iter().chain(tail.into_iter()).collect())
        .parse(input)
    }
}

fn origin_statements<'a>() -> impl parcel::Parser<'a, &'a [char], Origin<PreparseTokenStream>> {
    right(join(
        zero_or_more(non_newline_whitespace().or(|| newline())),
        join(
            origin(),
            zero_or_more(statement()).map(|ioc| {
                ioc.into_iter()
                    .filter(|oi| oi.is_some())
                    .map(|oi| oi.unwrap())
                    .collect()
            }),
        ),
    ))
    .map(|(offset, statements)| Origin::with_offset(offset as usize, statements))
}

fn statements<'a>() -> impl parcel::Parser<'a, &'a [char], PreparseTokenStream> {
    one_or_more(statement()).map(|ioc| {
        ioc.into_iter()
            .filter(|oi| oi.is_some())
            .map(|oi| oi.unwrap())
            .collect()
    })
}

fn statement<'a>() -> impl parcel::Parser<'a, &'a [char], Option<Token<String>>> {
    right(join(
        zero_or_more(non_newline_whitespace().or(|| newline())),
        left(join(
            labeldef()
                .map(|tok| Some(tok))
                .or(|| symboldef().map(|tok| Some(tok)))
                .or(|| constant().map(|tok| Some(tok)))
                .or(|| instruction().map(|tok| Some(tok)))
                .or(|| comment().map(|_| None)),
            right(join(
                join(zero_or_more(non_newline_whitespace()), optional(comment())),
                newline().or(|| eof()),
            )),
        )),
    ))
}

fn instruction<'a>() -> impl parcel::Parser<'a, &'a [char], Token<String>> {
    join(
        alphabetic(),
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
        })),
    )
    .map(|(head, tail)| {
        Token::Instruction(vec![head].into_iter().chain(tail.into_iter()).collect())
    })
}

fn comment<'a>() -> impl parcel::Parser<'a, &'a [char], ()> {
    right(join(
        expect_character(';'),
        zero_or_more(non_whitespace_character().or(|| non_newline_whitespace())),
    ))
    .map(|_| ())
}

fn labeldef<'a>() -> impl parcel::Parser<'a, &'a [char], Token<String>> {
    left(join(one_or_more(alphabetic()), expect_character(':')))
        .map(|cv| Token::Label(cv.into_iter().collect()))
}

fn symboldef<'a>() -> impl parcel::Parser<'a, &'a [char], Token<String>> {
    byte_def().or(|| two_byte_def()).or(|| four_byte_def())
}

fn byte_def<'a>() -> impl parcel::Parser<'a, &'a [char], Token<String>> {
    right(join(
        join(expect_str(".1byte"), one_or_more(non_newline_whitespace())),
        join(
            left(join(
                one_or_more(alphabetic()),
                one_or_more(non_newline_whitespace()),
            )),
            unsigned8(),
        ),
    ))
    .map(|(s, v)| Token::Symbol((s.into_iter().collect(), ByteValue::Byte(v))))
}

fn two_byte_def<'a>() -> impl parcel::Parser<'a, &'a [char], Token<String>> {
    right(join(
        join(expect_str(".2byte"), one_or_more(non_newline_whitespace())),
        join(
            left(join(
                one_or_more(alphabetic()),
                one_or_more(non_newline_whitespace()),
            )),
            unsigned16(),
        ),
    ))
    .map(|(s, v)| Token::Symbol((s.into_iter().collect(), ByteValue::Word(v))))
}

fn four_byte_def<'a>() -> impl parcel::Parser<'a, &'a [char], Token<String>> {
    right(join(
        join(expect_str(".4byte"), one_or_more(non_newline_whitespace())),
        join(
            left(join(
                one_or_more(alphabetic()),
                one_or_more(non_newline_whitespace()),
            )),
            unsigned32(),
        ),
    ))
    .map(|(s, v)| Token::Symbol((s.into_iter().collect(), ByteValue::DoubleWord(v))))
}

fn origin<'a>() -> impl parcel::Parser<'a, &'a [char], u32> {
    right(join(
        join(expect_str(".origin"), one_or_more(non_newline_whitespace())),
        unsigned32(),
    ))
}

fn constant<'a>() -> impl parcel::Parser<'a, &'a [char], Token<String>> {
    const_byte()
        .or(|| const_word())
        .or(|| const_doubleword())
        .map(|bv| Token::Constant(bv))
}

fn const_byte<'a>() -> impl parcel::Parser<'a, &'a [char], ByteValue> {
    right(join(
        join(expect_str(".byte"), one_or_more(non_newline_whitespace())),
        unsigned8(),
    ))
    .map(|v| ByteValue::Byte(v))
}

fn const_word<'a>() -> impl parcel::Parser<'a, &'a [char], ByteValue> {
    right(join(
        join(expect_str(".word"), one_or_more(non_newline_whitespace())),
        unsigned16(),
    ))
    .map(|v| ByteValue::Word(v))
}

fn const_doubleword<'a>() -> impl parcel::Parser<'a, &'a [char], ByteValue> {
    right(join(
        join(
            expect_str(".doubleword"),
            one_or_more(non_newline_whitespace()),
        ),
        unsigned32(),
    ))
    .map(|v| ByteValue::DoubleWord(v))
}
