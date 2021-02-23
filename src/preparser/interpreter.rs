use crate::preparser::{ast, types};
pub enum InterpreterError {
    UnknownErr,
    TypeError(types::TypeError),
}

impl std::fmt::Debug for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::UnknownErr => write!(f, "unknown interpreter error"),
            Self::TypeError(e) => write!(f, "{:?}", e),
        }
    }
}

pub trait WalkNode<Output> {
    type Error;

    fn walk_node(self) -> Result<Output, Self::Error>;
}

impl<Input, Output> WalkNode<Output> for Input
where
    Input: std::convert::TryInto<Output> + std::fmt::Display + Copy,
{
    type Error = InterpreterError;

    fn walk_node(self) -> Result<Output, Self::Error> {
        use std::convert::TryInto;
        let lhs = self;
        TryInto::<Output>::try_into(lhs).map_err(|_| {
            InterpreterError::TypeError(types::TypeError::IllegalType(self.to_string()))
        })
    }
}
