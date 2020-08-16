/// OpCode represents an unsigned 8bit value.
pub type OpCode = u8;

/// OpCodeOctal represents the [aaabbbcc] representation of an instructions
/// translation to an opcode table.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct OpCodeOctal(pub u8, pub u8, pub u8);

impl From<(u8, u8, u8)> for OpCodeOctal {
    fn from(tuple: (u8, u8, u8)) -> OpCodeOctal {
        OpCodeOctal(tuple.0, tuple.1, tuple.2)
    }
}

impl Into<(u8, u8, u8)> for OpCodeOctal {
    fn into(self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }
}

const OPCODES: [[[OpCode; 8]; 8]; 3] = [[[0; 8]; 8]; 3];
