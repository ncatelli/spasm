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
use crate::backends::mos6502::instruction_set::{Instruction, StaticInstruction};
use crate::preparser::{ByteValue, Token};
use crate::{Assembler, AssemblerResult};
use crate::{Emitter, Origin};

type UnparsedTokenStream = Vec<Token<String>>;
type Token6502InstStream = Vec<Token<Instruction>>;
type PositionalToken6502Stream = Vec<Positional<Token<Instruction>>>;
type AssembledOrigins = Vec<Origin<Vec<u8>>>;

type LabelMap = HashMap<String, u16>;
type SymbolMap = HashMap<String, u8>;

#[derive(Default)]
struct SymbolTable {
    labels: LabelMap,
    symbols: SymbolMap,
}

impl SymbolTable {
    fn new(labels: LabelMap, symbols: SymbolMap) -> Self {
        Self { labels, symbols }
    }
}

impl From<Vec<SymbolTable>> for SymbolTable {
    fn from(src: Vec<SymbolTable>) -> Self {
        let (labels, symbols): (Vec<LabelMap>, Vec<SymbolMap>) =
            src.into_iter().map(|st| (st.labels, st.symbols)).unzip();

        let labels = labels.into_iter().fold(LabelMap::new(), |acc, lm| {
            acc.into_iter().chain(lm).collect()
        });

        let symbols = symbols.into_iter().fold(SymbolMap::new(), |acc, sm| {
            acc.into_iter().chain(sm).collect()
        });

        Self::new(labels, symbols)
    }
}

/// Stores either an instruction or a constant value for assembling into a byte value
enum InstructionOrConstant<T> {
    Instruction(T),
    Constant(ByteValue),
}

fn parse_string_instructions_origin_to_token_instructions_origin(
    source: Origin<UnparsedTokenStream>,
) -> Result<Origin<Token6502InstStream>, String> {
    let origin_offset = source.offset;
    let tokens = source
        .instructions
        .into_iter()
        .map(|tok| match tok {
            Token::Label(v) => Ok(Token::Label(v)),
            Token::Symbol(v) => Ok(Token::Symbol(v)),
            Token::Constant(v) => Ok(Token::Constant(v)),
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
        .collect::<Result<Token6502InstStream, String>>()?;

    Ok(Origin::with_offset(origin_offset, tokens))
}

fn convert_token_instructions_origins_to_positional_tokens_origin(
    source: Origin<Token6502InstStream>,
) -> Result<Origin<PositionalToken6502Stream>, String> {
    let origin_offset = source.offset;
    let tokens = source.instructions;
    let positional_instructions = tokens
        .into_iter()
        .fold(
            (origin_offset, Vec::new()),
            |(offset, mut tokens), token| match token {
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
            },
        )
        .1;

    Ok(Origin::with_offset(origin_offset, positional_instructions))
}

fn generate_symbol_table_from_instructions_origin(
    source: Origin<PositionalToken6502Stream>,
) -> Result<(SymbolTable, Origin<Vec<InstructionOrConstant<Instruction>>>), String> {
    let (origin_offset, instructions) = source.into();
    let (symbol_table, tokens) = instructions.into_iter().fold(
        (SymbolTable::default(), Vec::new()),
        |(mut st, mut insts), positional_token| {
            let offset = positional_token.position;
            let token = positional_token.unwrap();
            match token {
                Token::Instruction(i) => {
                    insts.push(InstructionOrConstant::Instruction(i));
                    (st, insts)
                }
                Token::Constant(v) => {
                    insts.push(InstructionOrConstant::Constant(v));
                    (st, insts)
                }
                Token::Label(l) => {
                    st.labels.insert(l, offset as u16);
                    (st, insts)
                }
                Token::Symbol((id, bv)) => {
                    let sv = match bv {
                        ByteValue::Byte(v) => v,
                        e @ _ => panic!(format!("Backend only supports u8: passed {:?}", e)),
                    };

                    st.symbols.insert(id, sv);
                    (st, insts)
                }
            }
        },
    );
    Ok((symbol_table, Origin::with_offset(origin_offset, tokens)))
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

impl Assembler<Vec<Origin<UnparsedTokenStream>>, AssembledOrigins> for MOS6502Assembler {
    fn assemble(
        &self,
        source: Vec<Origin<UnparsedTokenStream>>,
    ) -> AssemblerResult<AssembledOrigins> {
        let token_instructions: Vec<Origin<Token6502InstStream>> = source
            .into_iter()
            .map(|origin| parse_string_instructions_origin_to_token_instructions_origin(origin))
            .collect::<Result<Vec<Origin<Token6502InstStream>>, String>>()?;
        let positional_tokens: Vec<Origin<PositionalToken6502Stream>> = token_instructions
            .into_iter()
            .map(|origin| convert_token_instructions_origins_to_positional_tokens_origin(origin))
            .collect::<Result<Vec<Origin<PositionalToken6502Stream>>, String>>()?;

        // Collect the symbols and instructions into a vector with each item
        // representing an origins contents
        let (symbol_tables, instructions): (
            Vec<SymbolTable>,
            Vec<Origin<Vec<InstructionOrConstant<Instruction>>>>,
        ) = positional_tokens
            .into_iter()
            .map(|origin| generate_symbol_table_from_instructions_origin(origin))
            .collect::<Result<Vec<(SymbolTable, Origin<Vec<InstructionOrConstant<Instruction>>>)>, String>>()?
            .into_iter()
            .unzip();

        // Join all the origin's symbol tables
        let symbol_table: SymbolTable = SymbolTable::from(symbol_tables);

        let opcode_origins = instructions
            .into_iter()
            .map(|origin| {
                let origin_offset = origin.offset;
                let instructions = origin.instructions;

                let assembled_instructions = instructions
                    .into_iter()
                    .map(|ioc| match ioc {
                        InstructionOrConstant::Instruction(i) => {
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
                                        |byte_value| {
                                            Ok((mnemonic, AddressMode::Immediate(*byte_value)))
                                        },
                                    )
                                }
                                AddressModeOrReference::AddressMode(am) => Ok((mnemonic, am)),
                            }
                            .map(|(m, am)| {
                                InstructionOrConstant::Instruction(StaticInstruction::new(m, am))
                            })
                        }
                        InstructionOrConstant::Constant(v) => {
                            Ok(InstructionOrConstant::Constant(v))
                        }
                    })
                    .collect::<Result<Vec<InstructionOrConstant<StaticInstruction>>, String>>()?
                    .into_iter()
                    .map(|ioc| match ioc {
                        InstructionOrConstant::Instruction(si) => {
                            let mc: Vec<u8> = si.emit();
                            mc
                        }
                        InstructionOrConstant::Constant(v) => {
                            let mc: Vec<u8> = v.emit();
                            mc
                        }
                    })
                    .flatten()
                    .collect::<Vec<u8>>();

                Ok(Origin::with_offset(origin_offset, assembled_instructions))
            })
            .collect::<Result<Vec<Origin<Vec<u8>>>, String>>()?;

        Ok(opcode_origins)
    }
}
