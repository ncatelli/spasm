use crate::preparser::{ast, types};
pub enum InterpreterError {
    UnknownErr,
}

impl std::fmt::Debug for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::UnknownErr => write!(f, "unknown interpreter error"),
        }
    }
}

pub trait WalkNode<A, B> {
    type Error;

    fn walk_node(&self, input: A) -> Result<B, Self::Error>;
}

pub struct StatefulInterpreter;

impl StatefulInterpreter {
    pub fn new() -> StatefulInterpreter {
        StatefulInterpreter
    }
}

impl Default for StatefulInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// Interpreter<Expr, object::Object> begins implemententing the required state
/// for interpreting Expressions in a stateful way.
impl WalkNode<ast::Expr, types::PrimitiveVariant> for StatefulInterpreter {
    type Error = InterpreterError;

    fn walk_node(&self, expr: ast::Expr) -> Result<types::PrimitiveVariant, Self::Error> {
        match expr {
            ast::Expr::Literal(pv) => self.interpret_literal(pv),
        }
    }
}

/// This functions only to unpack an Expr and dispatch to the upstream
/// Interpreter<Expr, object::Object> implementation.
impl WalkNode<Box<ast::Expr>, types::PrimitiveVariant> for StatefulInterpreter {
    type Error = InterpreterError;

    fn walk_node(&self, expr: Box<ast::Expr>) -> Result<types::PrimitiveVariant, Self::Error> {
        self.walk_node(*expr)
    }
}

impl StatefulInterpreter {
    fn interpret_literal(
        &self,
        primitive: types::PrimitiveVariant,
    ) -> Result<types::PrimitiveVariant, InterpreterError> {
        Ok(primitive)
    }
}
