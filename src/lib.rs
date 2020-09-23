use parcel::prelude::v1::*;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[macro_use]
pub mod instruction_set;
use instruction_set::address_mode::{AddressMode, AddressModeOrLabel};
use instruction_set::{InstructionOrSymbol, StaticInstruction};
mod addressing;
use addressing::SizeOf;
mod parser;

/// A type storing the results of an assemble representing an array of bytes
/// or a String Error.
pub type AssemblerResult = Result<Vec<u8>, String>;

type SymbolConfig = HashMap<String, u16>;

// Converts a source string to it's corresponding array of little endinan binary
// opcodes.
pub fn assemble(source: &str) -> AssemblerResult {
    let (_, symbols, insts) = match parser::instructions().parse(&source).unwrap() {
        parcel::MatchStatus::Match((_, insts)) => Ok(insts),
        _ => Err("match error".to_string()),
    }?
    .into_iter()
    .fold(
        (0 as u16, SymbolConfig::new(), Vec::new()),
        |(offset, mut labels, mut insts), ios| match ios {
            InstructionOrSymbol::Instruction(i) => {
                let size_of = i.size_of();
                insts.push(addressing::Positional::with_position(offset, i));
                (offset + size_of, labels, insts)
            }
            InstructionOrSymbol::Label(l) => {
                labels.insert(l, offset);
                (offset, labels, insts)
            }
        },
    );

    let mut static_insts = Vec::new();

    for i in insts.into_iter().map(|pi| pi.unwrap()).into_iter() {
        let amol = i.amol;
        let am = match amol {
            AddressModeOrLabel::Label(l) => symbols
                .get(&l)
                .map_or(Err("Symbol undefined".to_string()), |offset| {
                    Ok(AddressMode::Absolute(*offset))
                }),
            AddressModeOrLabel::AddressMode(am) => Ok(am),
        }?;

        static_insts.push(StaticInstruction::new(i.mnemonic, am));
    }

    let opcodes = static_insts
        .into_iter()
        .map(Into::<Vec<u8>>::into)
        .flatten()
        .collect::<Vec<u8>>();

    Ok(opcodes)
}
