use std::fmt::Debug;

pub enum ControlFlowGraphErr {
    BlockIdOutOfRange(BlockId),
}

/// BlockId represents a unique identifier for a block in the control flow graph.
pub type BlockId = usize;
type Edge = Option<BlockId>;
type BranchTargets = (Edge, Edge);

/// Graph represents a series of value IDs as upstream and downstream sets
/// where upstream sets map all the nodes that have edges to a given node
/// and downsets that map all edges from a given node.
#[derive(Debug, Default, Clone)]
pub struct Graph {
    downstream_edges: Vec<BranchTargets>,
}

impl Graph {
    /// Adds a new node in place by reference, returning the Id of the node.
    pub fn allocate_block_mut(&mut self) -> usize {
        self.downstream_edges.push((None, None));

        self.downstream_edges.len() - 1
    }

    /// Adds a new node by value, returning the modified instance of itself.
    #[allow(dead_code)]
    pub fn allocate_block(mut self) -> (Self, BlockId) {
        let new_id = self.allocate_block_mut();
        (self, new_id)
    }

    pub fn register_block_edge_mut(
        &mut self,
        blockid: BlockId,
        branch_match: Edge,
        branch_no_match: Edge,
    ) -> Result<(), ControlFlowGraphErr> {
        if self.downstream_edges.len() > blockid {
            self.downstream_edges[blockid] = (branch_match, branch_no_match);
            Ok(())
        } else {
            Err(ControlFlowGraphErr::BlockIdOutOfRange(blockid))
        }
    }

    pub fn register_block_edge(
        mut self,
        blockid: BlockId,
        branch_match: Edge,
        branch_no_match: Edge,
    ) -> Result<Self, ControlFlowGraphErr> {
        self.register_block_edge_mut(blockid, branch_match, branch_no_match)
            .map(|_| self)
    }
}

/// BasicBlock represents a unit of code that is a straight-line code sequence.
/// Branching instructions can only exist as the last instruction in the block
/// effictively giving a single path in and 2 branches out of any one block.
pub struct BasicBlock<T> {
    id: BlockId,
    inner: Vec<T>,
    branch_match: Edge,
    branch_no_match: Edge,
}
