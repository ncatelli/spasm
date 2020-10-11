extern crate parcel;
use parcel::prelude::v1::*;
mod parser;

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
        parser::statement().parse(input)
    }
}
