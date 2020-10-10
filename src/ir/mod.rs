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

pub mod instruction;
#[cfg(test)]
mod tests;
