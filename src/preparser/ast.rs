use crate::preparser::types;
/// Type System errors.
#[derive(Clone, PartialEq)]
pub enum InterpreterError {
    Unspecified(String),
    TypeErr(types::TypeError),
}

impl std::fmt::Debug for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unspecified(v) => write!(f, "unspecified: {:?}", v),
            Self::TypeErr(t) => write!(f, "{:?}", t),
        }
    }
}

pub trait Interpreter<O> {
    type Error;

    fn interpret(self) -> Result<O, Self::Error>;
}

pub enum Node {
    Expr(Expr),
}

impl Interpreter<types::Primitive<u8>> for Node {
    type Error = InterpreterError;

    fn interpret(self) -> Result<types::Primitive<u8>, Self::Error> {
        match self {
            Self::Expr(expr) => expr.interpret(),
        }
    }
}

pub enum Expr {
    Literal(Literal),
}

impl Interpreter<types::Primitive<u8>> for Expr {
    type Error = InterpreterError;

    fn interpret(self) -> Result<types::Primitive<u8>, Self::Error> {
        match self {
            Self::Literal(l) => l.interpret(),
        }
    }
}

pub enum Literal {
    U8(types::Primitive<u8>),
}

impl Interpreter<types::Primitive<u8>> for Literal {
    type Error = InterpreterError;

    fn interpret(self) -> Result<types::Primitive<u8>, Self::Error> {
        match self {
            Self::U8(prim) => Ok(prim),
        }
    }
}
