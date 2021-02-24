use crate::Origin;
use parcel::parsers::character::*;
use parcel::prelude::v1::*;
use parcel::{join, left, one_of, one_or_more, optional, right, zero_or_more};

// Pull in shared combinators
use crate::parser::*;

#[cfg(test)]
mod tests;

pub mod ast;
pub mod compile;
pub mod typechecker;
pub mod types;

/// Label represents a the string representation of a label.
pub type Label = String;

/// SymbolId represents a symbol identifier.
pub type SymbolId = String;

/// PrimitiveOrReference represents a case where a value can be represented as
/// either a static value or a reference.
#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveOrReference {
    Primitive(types::PrimitiveVariant),
    Reference(String),
}

/// Token wraps the token variants that can be derived from the
/// parser.
#[derive(Debug, Clone, PartialEq)]
pub enum Token<T> {
    Instruction(T),
    Label(Label),
    Symbol((SymbolId, types::PrimitiveVariant)),
    Constant(PrimitiveOrReference),
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
            origin_statements().or(|| statements().map(Origin::new)),
            zero_or_more(origin_statements()),
        )
        .map(|(head, tail)| vec![head].into_iter().chain(tail.into_iter()).collect())
        .parse(input)
    }
}

#[allow(clippy::redundant_closure)]
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

#[allow(clippy::redundant_closure)]
fn statement<'a>() -> impl parcel::Parser<'a, &'a [char], Option<Token<String>>> {
    right(join(
        zero_or_more(non_newline_whitespace().or(|| newline())),
        left(join(
            labeldef()
                .map(Some)
                .or(|| symboldef().map(Some))
                .or(|| constant().map(Some))
                .or(|| instruction().map(Some))
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

#[allow(clippy::redundant_closure)]
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

#[allow(clippy::redundant_closure)]
fn symboldef<'a>() -> impl parcel::Parser<'a, &'a [char], Token<String>> {
    byte_def().or(|| two_byte_def()).or(|| four_byte_def())
}

fn byte_def<'a>() -> impl parcel::Parser<'a, &'a [char], Token<String>> {
    right(join(
        join(
            expect_str(".define byte"),
            one_or_more(non_newline_whitespace()),
        ),
        join(
            left(join(
                one_or_more(alphabetic()),
                one_or_more(non_newline_whitespace()),
            )),
            unsigned8(),
        ),
    ))
    .map(|(s, v)| {
        Token::Symbol((
            s.into_iter().collect(),
            types::PrimitiveVariant::from(types::Primitive::new(v)),
        ))
    })
}

fn two_byte_def<'a>() -> impl parcel::Parser<'a, &'a [char], Token<String>> {
    right(join(
        join(
            expect_str(".define word"),
            one_or_more(non_newline_whitespace()),
        ),
        join(
            left(join(
                one_or_more(alphabetic()),
                one_or_more(non_newline_whitespace()),
            )),
            unsigned16(),
        ),
    ))
    .map(|(s, v)| {
        Token::Symbol((
            s.into_iter().collect(),
            types::PrimitiveVariant::from(types::Primitive::new(v)),
        ))
    })
}

fn four_byte_def<'a>() -> impl parcel::Parser<'a, &'a [char], Token<String>> {
    right(join(
        join(
            expect_str(".define doubleword"),
            one_or_more(non_newline_whitespace()),
        ),
        join(
            left(join(
                one_or_more(alphabetic()),
                one_or_more(non_newline_whitespace()),
            )),
            unsigned32(),
        ),
    ))
    .map(|(s, v)| {
        Token::Symbol((
            s.into_iter().collect(),
            types::PrimitiveVariant::from(types::Primitive::new(v)),
        ))
    })
}

fn origin<'a>() -> impl parcel::Parser<'a, &'a [char], u32> {
    right(join(
        join(expect_str(".origin"), one_or_more(non_newline_whitespace())),
        unsigned32(),
    ))
}

#[allow(clippy::redundant_closure)]
fn constant<'a>() -> impl parcel::Parser<'a, &'a [char], Token<String>> {
    const_byte()
        .or(|| const_word())
        .or(|| const_doubleword())
        .map(Token::Constant)
}

fn const_byte<'a>() -> impl parcel::Parser<'a, &'a [char], PrimitiveOrReference> {
    right(join(
        join(expect_str(".byte"), one_or_more(non_newline_whitespace())),
        unsigned8()
            .map(|b| PrimitiveOrReference::Primitive(types::Primitive::new(b).into()))
            .or(|| {
                one_or_more(alphabetic())
                    .map(|vc| PrimitiveOrReference::Reference(vc.into_iter().collect()))
            }),
    ))
}

fn const_word<'a>() -> impl parcel::Parser<'a, &'a [char], PrimitiveOrReference> {
    right(join(
        join(expect_str(".word"), one_or_more(non_newline_whitespace())),
        unsigned16()
            .map(|w| PrimitiveOrReference::Primitive(types::Primitive::new(w).into()))
            .or(|| {
                one_or_more(alphabetic())
                    .map(|vc| PrimitiveOrReference::Reference(vc.into_iter().collect()))
            }),
    ))
}

fn const_doubleword<'a>() -> impl parcel::Parser<'a, &'a [char], PrimitiveOrReference> {
    right(join(
        join(
            expect_str(".doubleword"),
            one_or_more(non_newline_whitespace()),
        ),
        unsigned32()
            .map(|dw| PrimitiveOrReference::Primitive(types::Primitive::new(dw).into()))
            .or(|| {
                one_or_more(alphabetic())
                    .map(|vc| PrimitiveOrReference::Reference(vc.into_iter().collect()))
            }),
    ))
}
