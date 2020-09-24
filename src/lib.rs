use parcel::prelude::v1::*;
use std::collections::HashMap;

#[macro_use]
pub mod instruction_set;
use instruction_set::address_mode::{AddressMode, AddressModeOrReference};
use instruction_set::{InstructionOrDefinition, Mnemonic, StaticInstruction};
mod addressing;
use addressing::SizeOf;
mod parser;

#[cfg(test)]
mod tests;

/// A type storing the results of an assemble representing an array of bytes
/// or a String Error.
pub type AssemblerResult = Result<Vec<u8>, String>;

type LabelMap = HashMap<String, u16>;
type SymbolMap = HashMap<String, u8>;

// Converts a source string to it's corresponding array of little endinan binary
// opcodes.
pub fn assemble(source: &str) -> AssemblerResult {
    let (_, labels, symbols, insts) = match parser::instructions().parse(&source).unwrap() {
        parcel::MatchStatus::Match((_, insts)) => Ok(insts),
        _ => Err("match error".to_string()),
    }?
    .into_iter()
    .enumerate()
    .fold(
        (0 as u16, LabelMap::new(), SymbolMap::new(), Vec::new()),
        |(offset, mut labels, mut symbols, mut insts), (line, iod)| match iod {
            InstructionOrDefinition::Instruction(i) => {
                let size_of = i.size_of();
                let line_number = line + 1;
                insts.push((
                    line_number,
                    addressing::Positional::with_position(offset, i),
                ));
                (offset + size_of, labels, symbols, insts)
            }
            InstructionOrDefinition::Label(l) => {
                labels.insert(l, offset);
                (offset, labels, symbols, insts)
            }
            InstructionOrDefinition::Symbol((s, v)) => {
                symbols.insert(s, v);
                (offset, labels, symbols, insts)
            }
        },
    );

    let opcodes = insts
        .into_iter()
        .map(|(line, pi)| (line, pi.unwrap()))
        .map(|(line, i)| {
            let mnemonic = i.mnemonic;
            let amol = i.amol;
            match amol {
                AddressModeOrReference::Label(l) => labels.get(&l).map_or(
                    Err(format!("label {}, undefined at line: {}", &l, line)),
                    |offset| Ok((mnemonic, AddressMode::Absolute(*offset))),
                ),
                AddressModeOrReference::Symbol(s) => symbols.get(&s).map_or(
                    Err(format!("symbol {}, undefined at line: {}", &s, line)),
                    |byte_value| Ok((mnemonic, AddressMode::Immediate(*byte_value))),
                ),
                AddressModeOrReference::AddressMode(am) => Ok((mnemonic, am)),
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
