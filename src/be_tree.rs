

use {
    std::{
        fmt,
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Child {
    None,
    Node(usize),
    Atom(usize),
}
impl Child {
    pub fn is_some(self) -> bool {
        match self {
            Self::None => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
struct Node<Op>
where
    Op: fmt::Debug + Clone + PartialEq + Default,
{
    operator: Op,
    parent: Option<usize>,
    left: Child,
    right: Child,
}

#[derive(Debug, Clone)]
pub struct BeTree<Op, Atom>
where
    Op: fmt::Debug + Clone + PartialEq + Default,
    Atom: fmt::Debug + Clone,
{
    atoms: Vec<Atom>,
    nodes: Vec<Node<Op>>,
    head: usize, // node index - where to start iterating
    tail: usize, // node index - where to add new nodes
}

impl<Op> Node<Op>
where
    Op: fmt::Debug + Clone + PartialEq + Default,
{
    fn is_full(&self) -> bool {
        self.right.is_some()
    }
    fn empty() -> Self {
        Self {
            operator: Op::default(),
            parent: None,
            left: Child::None,
            right: Child::None,
        }
    }
}

impl<Op, Atom> BeTree<Op, Atom>
where
    Op: fmt::Debug + Clone + PartialEq + Default,
    Atom: fmt::Debug + Clone,
{
    pub fn new() -> Self {
        Self {
            atoms: Vec::new(),
            nodes: vec![Node::empty()],
            head: 0,
            tail: 0,
        }
    }
    fn store_node(&mut self, node: Node<Op>) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }
    fn store_atom(&mut self,  atom: Atom) -> usize {
        self.atoms.push(atom);
        self.atoms.len() - 1
    }
    fn add_child(&mut self, child: Child) {
        if !self.nodes[self.tail].left.is_some() {
            self.nodes[self.tail].left = child;
        } else {
            self.nodes[self.tail].right = child;
        }
    }
    fn add_child_node(&mut self, child_idx: usize) {
        self.nodes[child_idx].parent = Some(self.tail);
        self.add_child(Child::Node(child_idx));
        self.tail = child_idx;
    }
    pub fn push_atom(&mut self, atom: Atom) {
        let atom_idx = self.store_atom(atom);
        if !self.nodes[self.tail].left.is_some() {
            self.nodes[self.tail].left = Child::Atom(atom_idx);
        } else {
            self.nodes[self.tail].right = Child::Atom(atom_idx);
        }
    }
    pub fn open_par(&mut self) {
        let node_idx = self.store_node(Node::empty());
        self.add_child_node(node_idx);
    }
    pub fn close_par(&mut self) {
        if let Some(parent) = self.nodes[self.tail].parent {
            self.tail = parent;
        }
        // we might want to return an error if there are too
        // many closing parenthesis in the future
    }
    pub fn push_operator(&mut self, operator: Op) {
        if self.nodes[self.tail].is_full() {
            // we replace the current tail
            // which becomes the left child of the new node
            let new_idx = self.store_node(Node {
                operator,
                parent: self.nodes[self.tail].parent,
                left: Child::Node(self.tail),
                right: Child::None,
            });
            // we connect the parent to the new node
            if let Some(parent_idx) = self.nodes[new_idx].parent {
                if self.nodes[parent_idx].left == Child::Node(self.tail) {
                    // the connection was to the left child
                    self.nodes[parent_idx].left = Child::Node(new_idx);
                } else {
                    // it must have been to the right child
                    debug_assert_eq!(self.nodes[parent_idx].right, Child::Node(self.tail));
                    self.nodes[parent_idx].right = Child::Node(new_idx);
                }
            } else {
                // the replaced node was the head
                self.head = new_idx;
            }
            // we connect the tail to the new node
            //if let Child::Node(child_idx) = self.nodes[self.tail]I
            self.nodes[self.tail].parent = Some(new_idx);
            // and we update the tail
            self.tail = new_idx;
        } else {
            self.nodes[self.tail].operator = operator;
        }
    }

    fn eval_child<EvalAtom, EvalOp, R>(&self, eval_atom: &EvalAtom, eval_op: &EvalOp, child: Child) -> Option<R>
    where
        R: fmt::Debug,
        EvalAtom: Fn(&Atom) -> R,
        EvalOp: Fn(&Op, R, R) -> R,
    {
        match child {
            Child::None => None,
            Child::Node(node_idx) => self.eval_node(eval_atom, eval_op, node_idx),
            Child::Atom(atom_idx) => Some(eval_atom(&self.atoms[atom_idx])),
        }
    }

    fn eval_node<EvalAtom, EvalOp, R>(&self, eval_atom: &EvalAtom, eval_op: &EvalOp, node_idx: usize) -> Option<R>
    where
        R: fmt::Debug,
        EvalAtom: Fn(&Atom) -> R,
        EvalOp: Fn(&Op, R, R) -> R,
    {
        let node = &self.nodes[node_idx];
        let left_value = self.eval_child(eval_atom, eval_op, node.left);
        let right_value = self.eval_child(eval_atom, eval_op, node.right);
        match (&node.operator, left_value, right_value) {
            (_, None, None) => None,
            (_, Some(v), None) => Some(v),
            (_, None, Some(v)) => Some(v),
            (op, Some(left), Some(right)) => Some(eval_op(op, left, right)),
        }
    }

    pub fn eval<EvalAtom, EvalOp, R>(&self, eval_atom: EvalAtom, eval_op: EvalOp) -> Option<R>
    where
        R: fmt::Debug,
        EvalAtom: Fn(&Atom) -> R,
        EvalOp: Fn(&Op, R, R) -> R,
    {
        self.eval_node(&eval_atom, &eval_op, self.head)
    }
}

