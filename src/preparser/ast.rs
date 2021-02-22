use crate::preparser::interpreter;
use crate::preparser::types;

/// Expr represents a runtime expression in the preparser.
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(types::PrimitiveVariant),
}
