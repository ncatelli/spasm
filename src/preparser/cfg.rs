use std::fmt::Debug;

/// BlockId represents a unique identifier for a block in the control flow graph.
pub type BlockId = usize;
type Edge = Option<BlockId>;

/// BlockTerminal defines the methods to signal if an instruction is branching or not.
pub trait BlockTerminal {
    fn is_branching() -> bool {
        false
    }
}

/// BasicBlock represents a unit of code that is a straight-line code sequence.
/// Branching instructions can only exist as the last instruction in the block
/// effictively giving a single path in and 2 branches out of any one block.
#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock<A, T>
where
    A: Into<usize>,
    T: BlockTerminal,
{
    pub id: BlockId,
    pub fixed_address: Option<A>,
    pub inner: Vec<T>,
    pub branch_match: Edge,
    pub branch_no_match: Edge,
}

impl<A, T> BasicBlock<A, T>
where
    A: Into<usize>,
    T: BlockTerminal,
{
    pub fn new(id: BlockId) -> Self {
        Self {
            id,
            fixed_address: None,
            inner: Vec::new(),
            branch_match: None,
            branch_no_match: None,
        }
    }
}
