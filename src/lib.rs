use parcel::prelude::v1::*;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[macro_use]
pub mod instruction_set;
use instruction_set::address_mode::{AddressMode, AddressModeOrLabel};
use instruction_set::{InstructionOrSymbol, Mnemonic, StaticInstruction};
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

    let opcodes = insts
        .into_iter()
        .map(|pi| pi.unwrap())
        .map(|i| {
            let mnemonic = i.mnemonic;
            let amol = i.amol;
            match amol {
                AddressModeOrLabel::Label(l) => symbols
                    .get(&l)
                    .map_or(Err("Symbol undefined".to_string()), |offset| {
                        Ok((mnemonic, AddressMode::Absolute(*offset)))
                    }),
                AddressModeOrLabel::AddressMode(am) => Ok((mnemonic, am)),
            }
        })
        .collect::<Result<Vec<(Mnemonic, AddressMode)>, String>>()?
        .into_iter()
        .map(|ti| StaticInstruction::new(ti.0, ti.1))
        .map(Into::<Vec<u8>>::into)
        .flatten()
        .collect::<Vec<u8>>();

    Ok(opcodes)
}
