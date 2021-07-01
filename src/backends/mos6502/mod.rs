#[macro_use]
pub mod instruction_set;
pub mod parser;

use parcel::prelude::v1::*;
use std::collections::HashMap;

use crate::addressing;
use crate::addressing::{Positional, SizeOf};
use crate::backends::mos6502::instruction_set::addressing_mode::AddressingModeOrReference;
use crate::backends::mos6502::instruction_set::Instruction;
use crate::backends::BackendErr;
use crate::preparser::{types, PrimitiveOrReference, Token};
use crate::{Assembler, AssemblerResult};
use crate::{Emitter, Origin};
use isa_mos6502::addressing_mode::AddressingMode;

type UnparsedTokenStream = Vec<Token<String>>;
type Token6502InstStream = Vec<Token<Instruction>>;
type PositionalToken6502Stream = Vec<Positional<Token<Instruction>>>;
type MemoryAligned6502Stream = Vec<InstructionOrConstant<Instruction, PrimitiveOrReference>>;
type AssembledOrigins = Vec<Origin<Vec<u8>>>;

use crate::preparser::types::Reify;
impl Reify<u8> for crate::preparser::types::LeByteEncodedValue {
    type Error = crate::preparser::types::TypeError;

    fn reify(&self) -> Result<u8, Self::Error> {
        match self.bits() {
            b if b == 0 => Ok(0),
            b if b <= 8 => Ok(self.to_vec().first().copied().unwrap_or(0)),
            _ => Err(Self::Error::IllegalType(format!(
                "bit-width {}",
                self.bits()
            ))),
        }
    }
}

impl Reify<u16> for crate::preparser::types::LeByteEncodedValue {
    type Error = crate::preparser::types::TypeError;

    fn reify(&self) -> Result<u16, Self::Error> {
        match self.bits() {
            b if b == 0 => Ok(0),
            b if b <= 8 => Reify::<u8>::reify(self).map(u16::from),
            b if b > 8 && b <= 16 => {
                let bytes = self.to_vec();
                Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
            }
            _ => Err(Self::Error::IllegalType(format!(
                "bit-width {}",
                self.bits()
            ))),
        }
    }
}

type SymbolMap = HashMap<String, LeByteEncodedValue>;

#[derive(Default, Debug)]
struct SymbolTable {
    symbols: SymbolMap,
}

use crate::preparser::types::LeByteEncodedValue;
impl SymbolTable {
    fn new(symbols: SymbolMap) -> Self {
        Self { symbols }
    }

    fn get(&self, k: &str) -> Option<LeByteEncodedValue> {
        self.symbols.get(k).cloned()
    }

    fn get_as_u8(&self, k: &str) -> Option<u8> {
        self.get(k)
            .map(|lebev| lebev.reify())
            .and_then(|res| res.ok())
    }

    fn get_as_u16(&self, k: &str) -> Option<u16> {
        self.get(k)
            .map(|lebev| Reify::<u16>::reify(&lebev))
            .and_then(|res| res.ok())
    }

    fn insert(&mut self, k: &str, v: LeByteEncodedValue) -> Option<LeByteEncodedValue> {
        self.symbols.insert(k.to_string(), v)
    }
}

impl From<Vec<SymbolTable>> for SymbolTable {
    fn from(src: Vec<SymbolTable>) -> Self {
        let symbols = src
            .into_iter()
            .map(|st| st.symbols)
            .fold(SymbolMap::new(), |acc, sm| {
                acc.into_iter().chain(sm).collect()
            });

        Self::new(symbols)
    }
}

/// Stores either an instruction or a constant with either value being
/// generalized as these values are commonly transformed through the pipeline.
enum InstructionOrConstant<T, U> {
    Instruction(T),
    Constant(U),
}

fn parse_string_instructions_origin_to_token_instructions_origin(
    source: Origin<UnparsedTokenStream>,
) -> Result<Origin<Token6502InstStream>, parser::ParseErr> {
    let origin_offset = source.offset;
    let tokens = source
        .instructions
        .into_iter()
        .map(|tok| match tok {
            Token::Symbol(id, v) => Ok(Token::Symbol(id, v)),
            Token::Constant(v) => Ok(Token::Constant(v)),
            Token::Instruction(inst) => {
                let input = inst.chars().collect::<Vec<char>>();
                let res = match parser::instruction().parse(&input) {
                    Ok(MatchStatus::Match((_, inst))) => Ok(Token::Instruction(inst)),
                    Ok(MatchStatus::NoMatch(remainder)) => Err(parser::ParseErr::Unspecified(
                        remainder.iter().collect::<String>(),
                    )),
                    Err(e) => Err(parser::ParseErr::Unspecified(e)),
                };
                res
            }
        })
        .collect::<Result<Token6502InstStream, parser::ParseErr>>()?;

    Ok(Origin::with_offset(origin_offset, tokens))
}

fn convert_token_instructions_origins_to_positional_tokens_origin(
    source: Origin<Token6502InstStream>,
) -> Origin<PositionalToken6502Stream> {
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
                t => {
                    tokens.push(addressing::Positional::with_position(offset, t));
                    (offset, tokens)
                }
            },
        )
        .1;

    Origin::with_offset(origin_offset, positional_instructions)
}

fn generate_symbol_table_from_instructions_origin(
    source: Origin<PositionalToken6502Stream>,
) -> (SymbolTable, Origin<MemoryAligned6502Stream>) {
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
                Token::Constant(bvol) => {
                    insts.push(InstructionOrConstant::Constant(bvol));
                    (st, insts)
                }
                Token::Symbol(l, None) => {
                    let normalized_offset = offset as u16;
                    st.insert(&l, LeByteEncodedValue::from(normalized_offset));
                    (st, insts)
                }
                Token::Symbol(id, Some(bv)) => {
                    st.insert(&id, bv);
                    (st, insts)
                }
            }
        },
    );
    (symbol_table, Origin::with_offset(origin_offset, tokens))
}

fn dereference_instructions_to_static_instructions(
    symbol_table: &SymbolTable,
    src_ioc: InstructionOrConstant<Instruction, PrimitiveOrReference>,
) -> Result<
    InstructionOrConstant<isa_mos6502::InstructionVariant, types::LeByteEncodedValue>,
    BackendErr,
> {
    match src_ioc {
        InstructionOrConstant::Instruction(i) => {
            let mnemonic = i.mnemonic;
            let amor = i.amor;
            match amor {
                AddressingModeOrReference::Label(l) => symbol_table
                    .get_as_u16(&l)
                    .map_or(Err(BackendErr::UndefinedReference(l.clone())), |offset| {
                        Ok((mnemonic, AddressingMode::Absolute(offset)))
                    }),
                AddressingModeOrReference::Symbol(s) => symbol_table.get_as_u8(&s.symbol).map_or(
                    Err(BackendErr::UndefinedReference(s.symbol.clone())),
                    |byte_value| Ok((mnemonic, AddressingMode::Immediate(byte_value))),
                ),
                AddressingModeOrReference::AddressingMode(am) => Ok((mnemonic, am)),
            }
            .map(|(m, am)| {
                isa_mos6502::InstructionVariant::new(m, am)
                    .map_err(|e| BackendErr::UndefinedInstruction(e.to_string()))
                    .map(InstructionOrConstant::Instruction)
            })?
        }
        InstructionOrConstant::Constant(bvol) => match bvol {
            PrimitiveOrReference::Primitive(bv) => Ok(bv),
            PrimitiveOrReference::Reference(id) => symbol_table
                .get(&id)
                .ok_or_else(|| BackendErr::UndefinedReference(id.clone())),
        }
        .map(InstructionOrConstant::Constant),
    }
}

/// Mos6502Assembler functions as a wrapper struct to facilitate an
/// implementation of the Assembler trait for the 6502 instruction set.
#[derive(Default)]
pub struct Mos6502Assembler {}

impl Mos6502Assembler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Assembler<Vec<Origin<UnparsedTokenStream>>, AssembledOrigins, BackendErr>
    for Mos6502Assembler
{
    fn assemble(
        &self,
        source: Vec<Origin<UnparsedTokenStream>>,
    ) -> AssemblerResult<AssembledOrigins, BackendErr> {
        // Parse a stream of text tokens into their corresponding types.
        let token_instructions: Vec<Origin<Token6502InstStream>> = source
            .into_iter()
            .map(parse_string_instructions_origin_to_token_instructions_origin)
            .collect::<Result<Vec<Origin<Token6502InstStream>>, parser::ParseErr>>()
            .map_err(|e| BackendErr::Parse(e.to_string()))?;

        // Annotate parsed tokens with their position and offsets. Then collect
        // the symbols and instructions into a vector of origin-aligned offsets.
        let (symbol_tables, instructions): (
            Vec<SymbolTable>,
            Vec<Origin<MemoryAligned6502Stream>>,
        ) = token_instructions
            .into_iter()
            .map(convert_token_instructions_origins_to_positional_tokens_origin)
            .map(generate_symbol_table_from_instructions_origin)
            .unzip();

        // Join all the origin's symbol tables into a global symbol table
        let symbol_table: SymbolTable = SymbolTable::from(symbol_tables);

        let opcode_origins = instructions
            .into_iter()
            // strip empty origins
            .filter(|origin| !origin.instructions.is_empty())
            .map(|origin| {
                let origin_offset = origin.offset;
                let instructions = origin.instructions;

                let assembled_instructions = instructions
                    .into_iter()
                    .map(|ioc| (&symbol_table, ioc))
                    .map(|(st, ioc)| dereference_instructions_to_static_instructions(st, ioc))
                    .collect::<Result<
                        Vec<
                            InstructionOrConstant<
                                isa_mos6502::InstructionVariant,
                                types::LeByteEncodedValue,
                            >,
                        >,
                        BackendErr,
                    >>()?
                    .into_iter()
                    .map(|ioc| match ioc {
                        InstructionOrConstant::Instruction(si) => {
                            let mc: Result<Vec<u8>, _> = si.emit();
                            mc
                        }
                        InstructionOrConstant::Constant(v) => {
                            let mc: Vec<u8> = v.emit();
                            Ok(mc)
                        }
                    })
                    .collect::<Result<Vec<Vec<u8>>, _>>()
                    .map_err(|e| BackendErr::UndefinedInstruction(e.to_string()))?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<u8>>();

                Ok(Origin::with_offset(origin_offset, assembled_instructions))
            })
            .collect::<Result<Vec<Origin<Vec<u8>>>, BackendErr>>()?;

        Ok(opcode_origins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_reify_a_16_bit_lebytesvalue() {
        let lebev = LeByteEncodedValue::new(vec![0, 128]);

        assert_eq!(Ok(0x8000), Reify::<u16>::reify(&lebev))
    }
}
