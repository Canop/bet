
use {
    std::fmt,
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
    unary: bool, // true when there's an operator in a unary position
}

impl<Op> Node<Op>
where
    Op: fmt::Debug + Clone + PartialEq,
{
    /// a node is full when we can't add other childs
    fn is_full(&self) -> bool {
        if self.unary {
            self.left.is_some()
        } else {
            self.right.is_some()
        }
    }
    fn empty() -> Self {
        Self {
            operator: None,
            parent: None,
            left: Child::None,
            right: Child::None,
            unary: false,
        }
    }
}

/// An expression which may contain unary and binary operations
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
    op_count: usize, // number of operators
    openess: usize, // opening pars minus closing pars
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
            op_count: 0,
            openess: 0,
        }
    }
}

impl<Op, Atom> BeTree<Op, Atom>
where
    Op: fmt::Debug + Clone + PartialEq,
    Atom: fmt::Debug + Clone,
{

    /// create an empty expression, ready to be completed
    pub fn new() -> Self {
        Self::default()
    }

    /// tells whether the expression is devoid of any atom
    pub fn is_empty(&self) -> bool {
        self.atoms.is_empty()
    }

    /// tell whether the tree is exactly one atom
    pub fn is_atomic(&self) -> bool {
        self.atoms.len() == 1 && self.op_count == 0
    }

    /// take the atoms of the tree
    pub fn atoms(self) -> Vec<Atom> {
        self.atoms
    }

    /// iterate on all atoms
    pub fn iter_atoms<'a>(&'a self) -> std::slice::Iter<'a, Atom> {
        self.atoms.iter()
    }

    /// returns a reference to the last atom if it's the last
    /// pushed token. Return none in other cases (including
    /// when no atom has been pushed at all)
    pub fn current_atom<'a>(&'a self) -> Option<&'a Atom> {
        if self.last_pushed == TokenType::Atom {
            self.atoms.last()
        } else {
            None
        }
    }

    /// return the count of open parenthesis minus the
    /// one of closing parenthesis. Illegal closing parenthesis
    /// are ignored (hence why this count can be a usize)
    pub fn get_openess(&self) -> usize {
        self.openess
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
        debug_assert!(!self.nodes[self.tail].is_full());
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

    /// add an atom in a left-to-right expression building
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
        self.openess += 1;
    }

    /// add a closing parenthesis to the expression
    pub fn close_par(&mut self) {
        self.last_pushed = TokenType::ClosingPar;
        if let Some(parent) = self.nodes[self.tail].parent {
            self.tail = parent;
            self.openess -= 1;
        }
        // we might want to return an error if there are too
        // many closing parenthesis in the future
    }

    fn push_unary_operator(&mut self, operator: Op) {
        let node_idx = self.store_node(Node {
            operator: Some(operator),
            parent: Some(self.tail),
            left: Child::None,
            right: Child::None,
            unary: true,
        });
        self.add_child(Child::Node(node_idx));
        self.tail = node_idx;
    }

    fn push_binary_operator(&mut self, operator: Op) {
        if self.nodes[self.tail].is_full() {
            // we replace the current tail
            // which becomes the left child of the new node
            let new_idx = self.store_node(Node {
                operator: Some(operator),
                parent: self.nodes[self.tail].parent,
                left: Child::Node(self.tail),
                right: Child::None,
                unary: false,
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

    /// add an operator right of the expression
    ///
    /// The context will decide whether it's unary or binary
    pub fn push_operator(&mut self, operator: Op) {
        match self.last_pushed {
            TokenType::Atom | TokenType::ClosingPar => {
                // the operator is binary
                self.push_binary_operator(operator);
            }
            _ => {
                // the operator is unary
                self.push_unary_operator(operator);
            }
        }
        self.last_pushed = TokenType::Operator;
        self.op_count += 1;
    }

    /// tell whether it would make sense to push a unary
    /// operator at this point (for example it makes no
    /// sense just after an atom)
    pub fn accept_unary_operator(&self) -> bool {
        use TokenType::*;
        match self.last_pushed {
            Nothing | Operator | OpeningPar => true,
            _ => false,
        }
    }

    /// tell whether it would make sense to push a binary
    /// operator at this point (for example it makes no
    /// sense just after another operator)
    pub fn accept_binary_operator(&self) -> bool {
        use TokenType::*;
        match self.last_pushed {
            Atom | ClosingPar => true,
            _ => false,
        }
    }

    /// tell whether it would make sense to push an atom
    /// at this point (for example it makes no
    /// sense just after a closing parenthesis)
    pub fn accept_atom(&self) -> bool {
        use TokenType::*;
        match self.last_pushed {
            Nothing | Operator | OpeningPar => true,
            _ => false,
        }
    }

    /// tell whether it would make sense to open a parenthesis
    /// at this point (for example it makes no sense just after
    /// a closing parenthesis)
    pub fn accept_opening_par(&self) -> bool {
        use TokenType::*;
        match self.last_pushed {
            Nothing | Operator | OpeningPar => true,
            _ => false,
        }
    }

    /// tell whether it would make sense to close a parenthesis
    /// at this point (for example it makes no sense just after
    /// an operator or if there are more closing parenthesis than
    /// opening ones)
    pub fn accept_closing_par(&self) -> bool {
        use TokenType::*;
        match self.last_pushed {
            Atom | ClosingPar if self.openess > 0 => true,
            _ => false,
        }
    }

    /// produce a new expression by applying a transformation on all atoms
    ///
    /// The operation will stop at the first error
    pub fn try_map_atoms<Atom2, Err, F>(&self, f: F) -> Result<BeTree<Op, Atom2>, Err>
    where
        Atom2: fmt::Debug + Clone,
        F: Fn(&Atom) -> Result<Atom2, Err>,
    {
        let mut atoms = Vec::new();
        for atom in &self.atoms {
            atoms.push(f(atom)?);
        }
        Ok(BeTree {
            atoms,
            nodes: self.nodes.clone(),
            head: self.head,
            tail: self.tail,
            last_pushed: self.last_pushed,
            op_count: self.op_count,
            openess: self.openess,
        })
    }

    fn eval_child<R, EvalAtom, EvalOp, ShortCircuit>(
        &self,
        eval_atom: &EvalAtom,
        eval_op: &EvalOp,
        short_circuit: &ShortCircuit,
        child: Child,
    ) -> Option<R>
    where
        EvalAtom: Fn(&Atom) -> R,
        EvalOp: Fn(&Op, R, Option<R>) -> R,
        ShortCircuit: Fn(&Op, &R) -> bool,
    {
        match child {
            Child::None => None,
            Child::Node(node_idx) => self.eval_node(eval_atom, eval_op, short_circuit, node_idx),
            Child::Atom(atom_idx) => Some(eval_atom(&self.atoms[atom_idx])),
        }
    }

    fn eval_child_faillible<Err, R, EvalAtom, EvalOp, ShortCircuit>(
        &self,
        eval_atom: &EvalAtom,
        eval_op: &EvalOp,
        short_circuit: &ShortCircuit,
        child: Child,
    ) -> Result<Option<R>, Err>
    where
        EvalAtom: Fn(&Atom) -> Result<R, Err>,
        EvalOp: Fn(&Op, R, Option<R>) -> Result<R, Err>,
        ShortCircuit: Fn(&Op, &R) -> bool,
    {
        Ok(match child {
            Child::None => None,
            Child::Node(node_idx) => self.eval_node_faillible(eval_atom, eval_op, short_circuit, node_idx)?,
            Child::Atom(atom_idx) => Some(eval_atom(&self.atoms[atom_idx])?),
        })
    }

    fn eval_node<R, EvalAtom, EvalOp, ShortCircuit>(
        &self,
        eval_atom: &EvalAtom,
        eval_op: &EvalOp,
        short_circuit: &ShortCircuit,
        node_idx: usize,
    ) -> Option<R>
    where
        EvalAtom: Fn(&Atom) -> R,
        EvalOp: Fn(&Op, R, Option<R>) -> R,
        ShortCircuit: Fn(&Op, &R) -> bool,
    {
        let node = &self.nodes[node_idx];
        let left_value = self.eval_child(eval_atom, eval_op, short_circuit, node.left);
        if let Some(op) = &node.operator {
            if let Some(left_value) = left_value {
                if short_circuit(op, &left_value) {
                    Some(left_value)
                } else {
                    let right_value = self.eval_child(eval_atom, eval_op, short_circuit, node.right);
                    Some(eval_op(op, left_value, right_value))
                }
            } else {
                // probably pathological
                None
            }
        } else {
            left_value
        }
    }

    fn eval_node_faillible<Err, R, EvalAtom, EvalOp, ShortCircuit>(
        &self,
        eval_atom: &EvalAtom,
        eval_op: &EvalOp,
        short_circuit: &ShortCircuit,
        node_idx: usize,
    ) -> Result<Option<R>, Err>
    where
        EvalAtom: Fn(&Atom) -> Result<R, Err>,
        EvalOp: Fn(&Op, R, Option<R>) -> Result<R, Err>,
        ShortCircuit: Fn(&Op, &R) -> bool,
    {
        let node = &self.nodes[node_idx];
        let left_value = self.eval_child_faillible(eval_atom, eval_op, short_circuit, node.left)?;
        Ok(
            if let Some(op) = &node.operator {
                if let Some(left_value) = left_value {
                    if short_circuit(op, &left_value) {
                        Some(left_value)
                    } else {
                        let right_value = self.eval_child_faillible(
                            eval_atom,
                            eval_op,
                            short_circuit,
                            node.right,
                        )?;
                        Some(eval_op(op, left_value, right_value)?)
                    }
                } else {
                    // probably pathological
                    None
                }
            } else {
                left_value
            }
        )
    }

    /// evaluate the expression.
    ///
    /// `eval_atom` will be called on all atoms (leafs) of the expression while `eval_op`
    /// will be used to join values until the final result is obtained.
    ///
    /// `short_circuit` will be called on all binary operations with the operator
    /// and the left operands as arguments. If it returns `true` then the right
    /// operand isn't evaluated (it's guaranteed so it may serve as guard).
    ///
    /// This function should be used when neither atom evaluation nor operator
    /// execution can raise errors (this usually means consistency checks have
    /// been done during parsing).
    pub fn eval<R, EvalAtom, EvalOp, ShortCircuit>(
        &self,
        eval_atom: EvalAtom,
        eval_op: EvalOp,
        short_circuit: ShortCircuit,
    ) -> Option<R>
    where
        EvalAtom: Fn(&Atom) -> R,
        EvalOp: Fn(&Op, R, Option<R>) -> R,
        ShortCircuit: Fn(&Op, &R) -> bool,
    {
        self.eval_node(&eval_atom, &eval_op, &short_circuit, self.head)
    }

    /// evaluate the expression.
    ///
    /// `eval_atom` will be called on all atoms (leafs) of the expression while `eval_op`
    /// will be used to join values until the final result is obtained.
    ///
    /// `short_circuit` will be called on all binary operations with the operator
    /// and the left operands as arguments. If it returns `true` then the right
    /// operand isn't evaluated (it's guaranteed so it may serve as guard).
    ///
    /// This function should be used when errors are expected during either atom
    /// evaluation or operator execution (for example because parsing was lax).
    /// The first Error returned by one of those functions breaks the evaluation
    /// and is returned.
    pub fn eval_faillible<Err, R, EvalAtom, EvalOp, ShortCircuit>(
        &self,
        eval_atom: EvalAtom,
        eval_op: EvalOp,
        short_circuit: ShortCircuit,
    ) -> Result<Option<R>, Err>
    where
        EvalAtom: Fn(&Atom) -> Result<R, Err>,
        EvalOp: Fn(&Op, R, Option<R>) -> Result<R, Err>,
        ShortCircuit: Fn(&Op, &R) -> bool,
    {
        self.eval_node_faillible(&eval_atom, &eval_op, &short_circuit, self.head)
    }
}

