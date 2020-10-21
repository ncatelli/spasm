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
    match backend {
        Backend::MOS6502 => backends::mos6502::MOS6502Assembler::new().assemble(source),
    }
}
