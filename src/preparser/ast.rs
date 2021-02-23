use crate::preparser::{ast, interpreter, types};

/// Expr represents a runtime expression in the preparser.
#[derive(Debug, Clone)]
pub enum Expr<T> {
    Literal(T),
}

impl<T> std::convert::From<Expr<types::Primitive<T>>> for types::Primitive<T> {
    fn from(src: Expr<types::Primitive<T>>) -> Self {
        match src {
            Expr::Literal(ty) => ty,
        }
    }
}

impl<Input, Output> interpreter::WalkNode<ast::Expr<Output>> for ast::Expr<Input>
where
    Output: Copy,
    Input: std::convert::TryInto<Output> + std::fmt::Display + Copy,
{
    type Error = interpreter::InterpreterError;

    fn walk_node(self) -> Result<ast::Expr<Output>, Self::Error> {
        match self {
            ast::Expr::Literal(lhs) => {
                Ok(ast::Expr::Literal(interpreter::WalkNode::walk_node(lhs)?))
            }
        }
    }
}
