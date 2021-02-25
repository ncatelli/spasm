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

impl From<types::TypeError> for InterpreterError {
    fn from(src: types::TypeError) -> Self {
        InterpreterError::TypeErr(src)
    }
}

pub trait Interpreter<O> {
    type Error;

    fn interpret(self) -> Result<O, Self::Error>;
}

pub enum Node {
    Expr(Expr),
}

impl Interpreter<types::PrimitiveVariant> for Node {
    type Error = InterpreterError;

    fn interpret(self) -> Result<types::PrimitiveVariant, Self::Error> {
        match self {
            Node::Expr(expr) => expr.interpret(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Plus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OperatorPlus;

pub enum Expr {
    Literal(types::PrimitiveVariant),
    Binary(Operator, Box<Expr>, Box<Expr>),
}

impl Interpreter<types::PrimitiveVariant> for Box<Expr> {
    type Error = InterpreterError;

    fn interpret(self) -> Result<types::PrimitiveVariant, Self::Error> {
        (*self).interpret()
    }
}

impl Interpreter<types::PrimitiveVariant> for Expr {
    type Error = InterpreterError;

    fn interpret(self) -> Result<types::PrimitiveVariant, Self::Error> {
        match self {
            Expr::Literal(prim) => Ok(prim),
            Expr::Binary(op, lhs, rhs) => interpret_binary(op, *lhs, *rhs),
        }
    }
}

// Implement helper methdods for Expr -> types::PrimitiveVariant
fn interpret_binary(
    op: Operator,
    lhs: Expr,
    rhs: Expr,
) -> Result<types::PrimitiveVariant, InterpreterError> {
    todo!()
}

// Primitive types

impl Interpreter<types::Primitive<u8>> for Expr {
    type Error = InterpreterError;

    fn interpret(self) -> Result<types::Primitive<u8>, Self::Error> {
        use types::PrimitiveVariant;
        let pv: PrimitiveVariant = self.interpret()?;

        match pv {
            PrimitiveVariant::Uint8(prim) => Ok(prim),
            PrimitiveVariant::Uint16(prim) => Err(InterpreterError::from(
                types::TypeError::IllegalType(prim.to_string()),
            )),
            PrimitiveVariant::Uint32(prim) => Err(InterpreterError::from(
                types::TypeError::IllegalType(prim.to_string()),
            )),
        }
    }
}

impl Interpreter<types::Primitive<u16>> for Expr {
    type Error = InterpreterError;

    fn interpret(self) -> Result<types::Primitive<u16>, Self::Error> {
        use types::PrimitiveVariant;
        let pv: PrimitiveVariant = self.interpret()?;

        match pv {
            PrimitiveVariant::Uint8(prim) => Err(InterpreterError::from(
                types::TypeError::IllegalType(prim.to_string()),
            )),
            PrimitiveVariant::Uint16(prim) => Ok(prim),
            PrimitiveVariant::Uint32(prim) => Err(InterpreterError::from(
                types::TypeError::IllegalType(prim.to_string()),
            )),
        }
    }
}

impl Interpreter<types::Primitive<u32>> for Expr {
    type Error = InterpreterError;

    fn interpret(self) -> Result<types::Primitive<u32>, Self::Error> {
        use types::PrimitiveVariant;
        let pv: PrimitiveVariant = self.interpret()?;

        match pv {
            PrimitiveVariant::Uint8(prim) => Err(InterpreterError::from(
                types::TypeError::IllegalType(prim.to_string()),
            )),
            PrimitiveVariant::Uint16(prim) => Err(InterpreterError::from(
                types::TypeError::IllegalType(prim.to_string()),
            )),
            PrimitiveVariant::Uint32(prim) => Ok(prim),
        }
    }
}
