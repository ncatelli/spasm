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

/// A type storing the results of an assemble representing an array of bytes
/// or a String Error.
pub type AssemblerResult = Result<Vec<u8>, String>;

/// The Assembler trait takes in an arbitrary length str, assembling it against
// a target and returning a result containing either the assembled bytecode or
// an error.
pub trait Assembler<T> {
    fn assemble(&self, source: T) -> AssemblerResult;
}

// Converts a source string to it's corresponding array of little endinan binary
// opcodes.
pub fn assemble(backend: Backend, source: &str) -> AssemblerResult {
    let input: Vec<char> = source.chars().collect();
    let tokens = preparser::PreParser::new().parse(&input).unwrap().unwrap();

    match backend {
        Backend::MOS6502 => backends::mos6502::MOS6502Assembler::new().assemble(tokens),
    }
}
