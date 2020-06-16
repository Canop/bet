

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

#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenType {
    Nothing,
    Atom,
    Operator,
    OpeningPar,
    ClosingPar,
}

#[derive(Debug, Clone)]
struct Node<Op>
where
    Op: fmt::Debug + Clone + PartialEq,
{
    operator: Option<Op>,
    parent: Option<usize>,
    left: Child,
    right: Child,
}

impl<Op> Node<Op>
where
    Op: fmt::Debug + Clone + PartialEq,
{
    /// a node is full when we can't add other childs
    fn is_full(&self) -> bool {
        self.right.is_some()
    }
    fn empty() -> Self {
        Self {
            operator: None,
            parent: None,
            left: Child::None,
            right: Child::None,
        }
    }
}

/// binary expression tree
#[derive(Debug, Clone)]
pub struct BeTree<Op, Atom>
where
    Op: fmt::Debug + Clone + PartialEq,
    Atom: fmt::Debug + Clone,
{
    atoms: Vec<Atom>,
    nodes: Vec<Node<Op>>,
    head: usize, // node index - where to start iterating
    tail: usize, // node index - where to add new nodes
    last_pushed: TokenType,
}


impl<Op, Atom> Default for BeTree<Op, Atom>
where
    Op: fmt::Debug + Clone + PartialEq,
    Atom: fmt::Debug + Clone,
{
    fn default() -> Self {
        Self {
            atoms: Vec::new(),
            nodes: vec![Node::empty()],
            head: 0,
            tail: 0,
            last_pushed: TokenType::Nothing,
        }
    }
}

impl<Op, Atom> BeTree<Op, Atom>
where
    Op: fmt::Debug + Clone + PartialEq,
    Atom: fmt::Debug + Clone,
{
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_empty(&self) -> bool {
        self.atoms.is_empty()
    }
    /// tell whether the tree is exactly one atom
    pub fn is_atomic(&self) -> bool {
        self.atoms.len() == 1
    }
    /// take the atoms of the tree
    pub fn atoms(self) -> Vec<Atom> {
        self.atoms
    }
    /// iterate on all atoms
    pub fn iter_atoms<'a>(&'a self) -> std::slice::Iter<'a, Atom> {
        self.atoms.iter()
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
        self.last_pushed = TokenType::Atom;
        let atom_idx = self.store_atom(atom);
        if !self.nodes[self.tail].left.is_some() {
            self.nodes[self.tail].left = Child::Atom(atom_idx);
        } else {
            self.nodes[self.tail].right = Child::Atom(atom_idx);
        }
    }
    /// if the last change was an atom pushed or modified, return a mutable
    /// reference to this atom. If not, push a new atom and return a mutable
    /// reference to it.
    pub fn mutate_or_create_atom<Create>(&mut self, create: Create) -> &mut Atom
    where
        Create: Fn() -> Atom,
    {
        if self.last_pushed != TokenType::Atom {
            self.push_atom(create());
        }
        self.atoms.last_mut().unwrap()
    }
    /// add an opening parenthesis to the expression
    pub fn open_par(&mut self) {
        self.last_pushed = TokenType::OpeningPar;
        let node_idx = self.store_node(Node::empty());
        self.add_child_node(node_idx);
    }
    /// add a closing parenthesis to the expression
    pub fn close_par(&mut self) {
        self.last_pushed = TokenType::ClosingPar;
        if let Some(parent) = self.nodes[self.tail].parent {
            self.tail = parent;
        }
        // we might want to return an error if there are too
        // many closing parenthesis in the future
    }
    pub fn push_operator(&mut self, operator: Op) {
        self.last_pushed = TokenType::Operator;
        if self.nodes[self.tail].is_full() {
            // we replace the current tail
            // which becomes the left child of the new node
            let new_idx = self.store_node(Node {
                operator: Some(operator),
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
            self.nodes[self.tail].operator = Some(operator);
        }
    }

    fn eval_child<EvalAtom, EvalOp, R>(&self, eval_atom: &EvalAtom, eval_op: &EvalOp, child: Child) -> Option<R>
    where
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
        EvalAtom: Fn(&Atom) -> R,
        EvalOp: Fn(&Op, R, R) -> R,
    {
        let node = &self.nodes[node_idx];
        let left_value = self.eval_child(eval_atom, eval_op, node.left);
        let right_value = self.eval_child(eval_atom, eval_op, node.right);
        if let Some(op) = &node.operator {
            match (left_value, right_value) {
                (None, None) => None,
                (Some(v), None) => Some(v),
                (None, Some(v)) => Some(v),
                (Some(left), Some(right)) => Some(eval_op(op, left, right)),
            }
        } else {
            left_value
        }
    }

    /// evaluate the expression.
    /// `eval_atom` will be called on all atoms (leafs) of the expression while `eval_op`
    /// will be used to join values until the final result is obtained.
    pub fn eval<EvalAtom, EvalOp, R>(&self, eval_atom: EvalAtom, eval_op: EvalOp) -> Option<R>
    where
        EvalAtom: Fn(&Atom) -> R,
        EvalOp: Fn(&Op, R, R) -> R,
    {
        self.eval_node(&eval_atom, &eval_op, self.head)
    }
}

