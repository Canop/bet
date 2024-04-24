use crate::*;

/// One of the children of a node
///
/// You probably don't need to use this struct unless
/// you want to inspect the binary expression tree.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Child {
    None,
    Node(NodeId),
    Atom(AtomId),
}
impl Child {
    pub fn is_some(self) -> bool {
        !matches!(self, Self::None)
    }
}
