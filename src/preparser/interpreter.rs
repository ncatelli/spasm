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
