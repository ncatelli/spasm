use parcel::prelude::v1::*;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[macro_use]
pub mod instruction_set;
use instruction_set::InstructionOrSymbol;
mod addressing;
mod parser;

pub type AssemblerResult = Result<Vec<u8>, String>;

// Converts a source string to it's corresponding array of little endinan binary
// opcodes.
pub fn assemble(source: &str) -> AssemblerResult {
    let opcodes: Vec<u8> = match parser::instructions().parse(&source).unwrap() {
        parcel::MatchStatus::Match((_, insts)) => Ok(insts),
        _ => Err("match error".to_string()),
    }?
    .into_iter()
    .fold(
        (HashMap::<String, u16>::new(), Vec::new()),
        |(mut labels, mut insts), ios| match ios {
            InstructionOrSymbol::Instruction(i) => {
                insts.push(i);
                (labels, insts)
            }
            InstructionOrSymbol::Label(l) => {
                labels.insert(l, 0);
                (labels, insts)
            }
        },
    ) // DEVNOTE: temp unpack of instructions from InstructionOrSymbol enum
    .1
    .into_iter()
    .map(Into::<Vec<u8>>::into)
    .flatten()
    .collect::<Vec<u8>>();

    Ok(opcodes)
}
