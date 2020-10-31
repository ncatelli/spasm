use parcel::prelude::v1::*;
mod addressing;
mod backends;
pub use backends::Backend;
mod parser;
mod preparser;

#[cfg(test)]
mod tests;

/// Emitter defines the trait necessary to output a machine code
/// representation of an object. An example case where this would be
/// used is converting an assembly instruction from it's higher level
/// for to it's machine specific representation.
pub trait Emitter<T> {
    fn emit(&self) -> T;
}

/// Origin provides a structure for denoting memory offsets.
#[derive(Debug, PartialEq, Clone)]
pub struct Origin<T> {
    pub offset: usize,
    pub instructions: T,
}

impl<T> Origin<T> {
    pub fn new(instructions: T) -> Self {
        Self {
            offset: 0,
            instructions,
        }
    }

    pub fn with_offset(offset: usize, instructions: T) -> Self {
        Self {
            offset,
            instructions,
        }
    }
}

impl<T> From<(usize, T)> for Origin<T> {
    fn from(src: (usize, T)) -> Self {
        Origin::with_offset(src.0, src.1)
    }
}

impl<T> Into<(usize, T)> for Origin<T> {
    fn into(self) -> (usize, T) {
        (self.offset, self.instructions)
    }
}

impl Emitter<Vec<u8>> for u8 {
    fn emit(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl Emitter<Vec<u8>> for Vec<u8> {
    fn emit(&self) -> Vec<u8> {
        self.clone()
    }
}

impl Emitter<Vec<u8>> for Vec<Origin<Vec<u8>>> {
    fn emit(&self) -> Vec<u8> {
        let mut origins = self.clone();
        origins.sort_by(|a, b| b.offset.cmp(&a.offset));
        let (offsets, unpadded_bytecode): (Vec<(usize, usize)>, Vec<Vec<u8>>) = origins
            .into_iter()
            .map(|origin| {
                (
                    (
                        origin.offset,
                        (origin.offset + origin.instructions.len() - 1),
                    ),
                    origin.instructions,
                )
            })
            .unzip();

        let (offset_start, offset_end): (Vec<usize>, Vec<usize>) = offsets.into_iter().unzip();

        let padding = offset_start[1..]
            .into_iter()
            .map(|offset| *offset)
            .zip(
                offset_end[..offset_end.len() - 1]
                    .into_iter()
                    .map(|offset| *offset),
            )
            .map(|(end_of_last, start_of_next)| start_of_next - end_of_last)
            .chain(vec![0].into_iter())
            .collect::<Vec<usize>>();

        unpadded_bytecode
            .into_iter()
            .zip(padding)
            .map(|(bytecode, pad_size)| {
                let normaized_pad_size = if pad_size > 0 { pad_size - 1 } else { 0 };
                bytecode
                    .into_iter()
                    .chain(vec![0 as u8].into_iter().cycle().take(normaized_pad_size))
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .collect()
    }
}

type AssembledOrigins = Vec<Origin<Vec<u8>>>;

/// A type storing the results of an assemble representing an array of bytes
/// or a String Error.
pub type AssemblerResult<U> = Result<U, String>;

/// The Assembler trait takes in an arbitrary length str, assembling it against
// a target and returning a result containing either the assembled bytecode or
// an error.
pub trait Assembler<T, U> {
    fn assemble(&self, source: T) -> AssemblerResult<U>;
}

// Converts a source string to it's corresponding array of little endinan binary
// opcodes.
pub fn assemble(backend: Backend, source: &str) -> AssemblerResult<AssembledOrigins> {
    let input: Vec<char> = source.chars().collect();
    let origin_tokens = preparser::PreParser::new().parse(&input).unwrap().unwrap();

    match backend {
        Backend::MOS6502 => backends::mos6502::MOS6502Assembler::new().assemble(origin_tokens),
    }
}
