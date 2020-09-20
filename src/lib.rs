use parcel::prelude::v1::*;

#[cfg(test)]
mod tests;

pub mod instruction_set;
use instruction_set::InstructionOrSymbol;
mod parser;

pub type AssemblerResult = Result<Vec<u8>, String>;

// Converts a source string to it's corresponding array of little endinan binary
// opcodes.
pub fn assemble(source: &str) -> AssemblerResult {
    let opcodes = match parser::instructions().parse(&source).unwrap() {
        parcel::MatchStatus::Match((_, insts)) => Ok(insts),
        _ => Err("match error".to_string()),
    }?
    .into_iter()
    .map(|ios| match ios {
        InstructionOrSymbol::Instruction(i) => i,
        _ => panic!("not implemented".to_string()),
    }) // DEVNOTE: temp unpack of instructions from InstructionOrSymbol enum
    .map(Into::<Vec<u8>>::into)
    .flatten()
    .collect::<Vec<u8>>();

    Ok(opcodes)
}
