use {crate::Child, std::fmt};

pub type NodeId = usize;

/// A node in the expression tree
///
/// You probably don't need to use this struct
/// unless you want to inspect the tree
#[derive(Debug, Clone, PartialEq)]
pub struct Node<Op>
where
    Op: fmt::Debug + Clone + PartialEq,
{
    pub operator: Option<Op>,
    pub parent: Option<NodeId>,
    pub left: Child,
    pub right: Child,
    pub unary: bool, // true when there's an operator in a unary position
}

impl<Op> Node<Op>
where
    Op: fmt::Debug + Clone + PartialEq,
{
    /// a node is full when we can't add other childs
    pub fn is_full(&self) -> bool {
        if self.unary {
            self.left.is_some()
        } else {
            self.right.is_some()
        }
    }
    pub fn empty() -> Self {
        Self {
            operator: None,
            parent: None,
            left: Child::None,
            right: Child::None,
            unary: false,
        }
    }
}
