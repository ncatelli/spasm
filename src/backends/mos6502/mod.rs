#[macro_use]
pub mod instruction_set;
pub mod parser;

use parcel::prelude::v1::*;
use std::collections::HashMap;

use crate::addressing;
use crate::addressing::{Positional, SizeOf};
use crate::backends::mos6502::instruction_set::address_mode::{
    AddressMode, AddressModeOrReference,
};
use crate::backends::mos6502::instruction_set::{Instruction, Mnemonic, StaticInstruction};
use crate::preparser::{ByteValue, Token};
use crate::Emitter;
use crate::{Assembler, AssemblerResult};

type UnparsedTokenStream = Vec<Token<String>>;
type Token6502InstStream = Vec<Token<Instruction>>;
type PositionalToken6502Stream = Vec<Positional<Token<Instruction>>>;

type LabelMap = HashMap<String, u16>;
type SymbolMap = HashMap<String, u8>;

struct SymbolTable {
    labels: LabelMap,
    symbols: SymbolMap,
}

impl SymbolTable {
    fn new() -> Self {
        Self {
            labels: LabelMap::new(),
            symbols: SymbolMap::new(),
        }
    }

    #[allow(dead_code)]
    fn from() -> Self {
        Self {
            labels: LabelMap::new(),
            symbols: SymbolMap::new(),
        }
    }

    #[allow(dead_code)]
    fn into_tuple(self) -> (LabelMap, SymbolMap) {
        (self.labels, self.symbols)
    }
}

fn parse_string_instructions_to_token_instructions(
    source: UnparsedTokenStream,
) -> Result<Token6502InstStream, String> {
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

fn convert_token_instructions_to_positional_tokens(
    source: Token6502InstStream,
) -> Result<PositionalToken6502Stream, String> {
    let positional_instructions = source
        .into_iter()
        .fold((0, Vec::new()), |(offset, mut tokens), token| match token {
            Token::Instruction(i) => {
                let size_of = i.size_of();
                tokens.push(addressing::Positional::with_position(
                    offset,
                    Token::Instruction(i),
                ));
                (offset + size_of, tokens)
            }
            t @ _ => {
                tokens.push(addressing::Positional::with_position(offset, t));
                (offset, tokens)
            }
        })
        .1;
    Ok(positional_instructions)
}

fn generate_symbol_table_from_instructions(
    source: PositionalToken6502Stream,
) -> Result<(SymbolTable, Vec<Instruction>), String> {
    let (symbol_table, tokens) = source.into_iter().fold(
        (SymbolTable::new(), Vec::new()),
        |(mut st, mut insts), positional_token| {
            let offset = positional_token.position;
            let token = positional_token.unwrap();
            match token {
                Token::Instruction(i) => {
                    insts.push(i);
                    (st, insts)
                }
                Token::Label(l) => {
                    st.labels.insert(l, offset as u16);
                    (st, insts)
                }
                Token::Symbol((id, bv)) => {
                    let sv = match bv {
                        ByteValue::One(v) => v,
                        e @ _ => panic!(format!("Backend only supports u8: passed {:?}", e)),
                    };

                    st.symbols.insert(id, sv);
                    (st, insts)
                }
            }
        },
    );
    Ok((symbol_table, tokens))
}

/// MOS6502Assembler functions as a wrapper struct to facilitate an
/// implementation of the Assembler trait for the 6502 instruction set.
#[derive(Default)]
pub struct MOS6502Assembler {}

impl MOS6502Assembler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Assembler<UnparsedTokenStream> for MOS6502Assembler {
    fn assemble(&self, source: UnparsedTokenStream) -> AssemblerResult {
        let token_instructions = parse_string_instructions_to_token_instructions(source)?;
        let positional_tokens =
            convert_token_instructions_to_positional_tokens(token_instructions)?;
        let (symbol_table, instructions) =
            generate_symbol_table_from_instructions(positional_tokens)?;

        let opcodes = instructions
            .into_iter()
            .map(|i| {
                let mnemonic = i.mnemonic;
                let amor = i.amor;
                match amor {
                    AddressModeOrReference::Label(l) => symbol_table
                        .labels
                        .get(&l)
                        .map_or(Err(format!("label {} undefined", &l)), |offset| {
                            Ok((mnemonic, AddressMode::Absolute(*offset)))
                        }),
                    AddressModeOrReference::Symbol(s) => {
                        symbol_table.symbols.get(&s.symbol).map_or(
                            Err(format!("symbol {} undefined", &s.symbol)),
                            |byte_value| Ok((mnemonic, AddressMode::Immediate(*byte_value))),
                        )
                    }
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
