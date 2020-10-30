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

type UnparsedTokenStream = Vec<Token<String>>;
type ParsedTokenStream = Vec<Token<Instruction>>;

type LabelMap = HashMap<String, u16>;
type SymbolMap = HashMap<String, u8>;

struct SymbolTable {
    labels: LabelMap,
    symbols: SymbolMap,
}

impl SymbolTable {
    fn new(l: LabelMap, s: SymbolMap) -> Self {
        Self {
            labels: l,
            symbols: s,
        }
    }

    fn into_tuple(self) -> (LabelMap, SymbolMap) {
        (self.labels, self.symbols)
    }
}

#[derive(Default)]
pub struct MOS6502Assembler {}

impl MOS6502Assembler {
    pub fn new() -> Self {
        Self::default()
    }

    fn parse_string_instructions_to_token(
        &self,
        source: UnparsedTokenStream,
    ) -> Result<ParsedTokenStream, String> {
        source
            .into_iter()
            .map(|tok| match tok {
                Token::Label(v) => Ok(Token::Label(v)),
                Token::Symbol(v) => Ok(Token::Symbol(v)),
                Token::Instruction(inst) => {
                    let input = inst.chars().collect::<Vec<char>>();
                    let res = match parser::instruction().parse(&input) {
                        Ok(MatchStatus::Match((_, inst))) => Ok(Token::Instruction(inst)),
                        Ok(MatchStatus::NoMatch(remainder)) => Err(format!(
                            "no match found while parsing: {}",
                            remainder.into_iter().collect::<String>()
                        )),
                        Err(e) => Err(e),
                    };
                    res
                }
            })
            .collect()
    }
}

impl Assembler<UnparsedTokenStream> for MOS6502Assembler {
    fn assemble(&self, source: UnparsedTokenStream) -> AssemblerResult {
        let (_, labels, symbols, insts) = self
            .parse_string_instructions_to_token(source)?
            .into_iter()
            .fold(
                (0, LabelMap::new(), SymbolMap::new(), Vec::new()),
                |(offset, mut labels, mut symbols, mut insts), tok| match tok {
                    Token::Instruction(i) => {
                        let size_of = i.size_of();
                        insts.push(addressing::Positional::with_position(offset, i));
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
                },
            );

        let opcodes = insts
            .into_iter()
            .map(|pi| (pi.unwrap()))
            .map(|i| {
                let mnemonic = i.mnemonic;
                let amor = i.amor;
                match amor {
                    AddressModeOrReference::Label(l) => labels
                        .get(&l)
                        .map_or(Err(format!("label {} undefined", &l)), |offset| {
                            Ok((mnemonic, AddressMode::Absolute(*offset)))
                        }),
                    AddressModeOrReference::Symbol(s) => symbols.get(&s.symbol).map_or(
                        Err(format!("symbol {} undefined", &s.symbol)),
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
