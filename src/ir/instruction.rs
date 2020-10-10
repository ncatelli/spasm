use parcel::prelude::v1::*;
use parcel::{BoxedParser, ParseResult};

/// TargetInstruction
pub struct TargetInstruction<'a, T, U> {
    asm_to_mc: BoxedParser<'a, T, U>,
}

impl<'a, T, U> TargetInstruction<'a, T, U> {
    pub fn new(asm_to_mc: BoxedParser<'a, T, U>) -> Self {
        TargetInstruction { asm_to_mc }
    }
}

impl<'a, T, U> Parser<'a, T, U> for TargetInstruction<'a, T, U> {
    fn parse(&self, input: T) -> ParseResult<'a, T, U> {
        self.asm_to_mc.parse(input)
    }
}
