#[macro_use]
pub mod instruction_set;
pub mod parser;

use parcel::prelude::v1::*;
use std::collections::HashMap;

use crate::addressing;
use crate::addressing::SizeOf;
use crate::backends::mos6502::instruction_set::address_mode::{
    AddressMode, AddressModeOrReference,
};
use crate::backends::mos6502::instruction_set::{Instruction, Mnemonic, StaticInstruction};
use crate::preparser::{ByteValue, Token};
use crate::Emitter;
use crate::{Assembler, AssemblerResult};

type LabelMap = HashMap<String, u16>;
type SymbolMap = HashMap<String, u8>;

#[derive(Default)]
pub struct MOS6502Assembler {}

impl MOS6502Assembler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Assembler<Vec<Token<String>>> for MOS6502Assembler {
    fn assemble(&self, source: Vec<Token<String>>) -> AssemblerResult {
        let (_, labels, symbols, insts) = source
            .into_iter()
            .map(|tok| match tok {
                Token::Label(v) => Ok(Token::Label(v)),
                Token::Offset(u) => Ok(Token::Offset(u)),
                Token::Symbol(v) => Ok(Token::Symbol(v)),
                Token::Instruction(inst) => {
                    let input = inst.chars().collect::<Vec<char>>();
                    let res = match parser::instruction().parse(&input) {
                        Ok(MatchStatus::Match((_, iod))) => Ok(Token::Instruction(iod)),
                        Ok(MatchStatus::NoMatch(remainder)) => Err(format!(
                            "Unable to parse: {}",
                            remainder.into_iter().collect::<String>()
                        )),
                        Err(e) => Err(e),
                    };
                    res
                }
            })
            .collect::<Result<Vec<Token<Instruction>>, String>>()?
            .into_iter()
            .enumerate()
            .fold(
                (0, LabelMap::new(), SymbolMap::new(), Vec::new()),
                |(offset, mut labels, mut symbols, mut insts), (line, tok)| match tok {
                    Token::Instruction(i) => {
                        let size_of = i.size_of();
                        let line_number = line + 1;
                        insts.push((
                            line_number,
                            addressing::Positional::with_position(offset, i),
                        ));
                        (offset + size_of, labels, symbols, insts)
                    }
                    Token::Label(l) => {
                        labels.insert(l, offset as u16);
                        (offset, labels, symbols, insts)
                    }
                    Token::Symbol((id, bv)) => {
                        let sv = match bv {
                            ByteValue::One(v) => v,
                            e @ _ => panic!(format!("Backend only supports u8: passed {:?}", e)),
                        };

                        symbols.insert(id, sv);
                        (offset, labels, symbols, insts)
                    }
                    Token::Offset(o) => (o as usize, labels, symbols, insts),
                },
            );

        let opcodes = insts
            .into_iter()
            .map(|(line, pi)| (line, pi.unwrap()))
            .map(|(line, i)| {
                let mnemonic = i.mnemonic;
                let amor = i.amor;
                match amor {
                    AddressModeOrReference::Label(l) => labels.get(&l).map_or(
                        Err(format!("label {}, undefined at line: {}", &l, line)),
                        |offset| Ok((mnemonic, AddressMode::Absolute(*offset))),
                    ),
                    AddressModeOrReference::Symbol(s) => symbols.get(&s.symbol).map_or(
                        Err(format!("symbol {}, undefined at line: {}", &s.symbol, line)),
                        |byte_value| Ok((mnemonic, AddressMode::Immediate(*byte_value))),
                    ),
                    AddressModeOrReference::AddressMode(am) => Ok((mnemonic, am)),
                }
            })
            .collect::<Result<Vec<(Mnemonic, AddressMode)>, String>>()?
            .into_iter()
            .map(|ti| StaticInstruction::new(ti.0, ti.1))
            .map(|si| {
                let mc: Vec<u8> = si.emit();
                mc
            })
            .flatten()
            .collect::<Vec<u8>>();

        Ok(opcodes)
    }
}
